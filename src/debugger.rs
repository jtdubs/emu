use chrono::Duration;
use crossterm::{cursor, event, execute, style::Print, terminal};
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::sync::mpsc::channel;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex,
};
use std::thread;
use std::time::Instant;
use timer::Timer;

use crate::component::mos6502;
use crate::component::snes_controller::Button;
use crate::system::System;

const CYCLE_NANOSECONDS: u64 = 1000;
const CYCLES_PER_EPOCH: u64 = 10000;
const WINDOW_SIZE: u64 = 200;

pub struct Debugger<SystemType: System> {
    sys: SystemType,
    breakpoints: Vec<u16>,
    sym2addr: HashMap<String, u16>,
    addr2sym: HashMap<u16, String>,
    cycle_count: u64,
    epoch_start: Instant,
    avg_nanos_per_epoch: u64,
    timer: Timer,
    cycle_gate: Arc<(Condvar, Mutex<u32>)>,
    bench: bool,
}

#[allow(dead_code)]
impl<SystemType: System> Debugger<SystemType> {
    pub fn new(sys: SystemType) -> Debugger<SystemType> {
        Debugger {
            sys: sys,
            breakpoints: Vec::new(),
            sym2addr: HashMap::new(),
            addr2sym: HashMap::new(),
            cycle_count: 0,
            epoch_start: Instant::now(),
            avg_nanos_per_epoch: CYCLES_PER_EPOCH * WINDOW_SIZE * CYCLE_NANOSECONDS,
            timer: Timer::new(),
            cycle_gate: Arc::new((Condvar::new(), Mutex::new(0))),
            bench: false,
        }
    }

    pub fn step(&mut self) {
        let cycle_schedule = self.start_timer();
        self.step_next();
        drop(cycle_schedule);
    }

    pub fn step_over(&mut self) {
        if self.sys.get_cpu().ir.0 == mos6502::Instruction::JSR {
            self.breakpoints.push(self.sys.get_cpu().pc + 2);
            self.run();
            self.breakpoints.pop();
        } else {
            self.step_next();
        }
    }

    pub fn step_out(&mut self) {
        let mut depth: i32 = 0;

        let cycle_schedule = self.start_timer();

        loop {
            match self.sys.get_cpu().ir.0 {
                mos6502::Instruction::JSR => depth += 1,
                mos6502::Instruction::RTS => depth -= 1,
                _ => {}
            }

            self.step_next();

            if self.sys.is_halted() {
                break;
            }

            if depth < 0 {
                break;
            }
        }

        drop(cycle_schedule);
    }

    pub fn run_headless(&mut self) {
        let cycle_schedule = self.start_timer();

        loop {
            self.step_next();

            {
                if self.sys.is_halted() {
                    break;
                }

                if self.breakpoints.contains(&(self.sys.get_cpu().pc - 1)) {
                    break;
                }
            }
        }

        drop(cycle_schedule);
    }

    pub fn bench(&mut self) {
        self.bench = true;
        self.run();
        self.bench = false;
    }

    pub fn bench_n(&mut self, skip_breakpoints: u32) {
        self.bench = true;
        self.run_n(skip_breakpoints);
        self.bench = false;
    }

    pub fn run_n(&mut self, mut skip_breakpoints: u32) {
        let mut stdout = stdout();
        let cycle_schedule = self.start_timer();

        let (key_send, key_recv) = channel();
        let key_exit = Arc::new(AtomicBool::new(false));
        let thread_key_exit = key_exit.clone();
        thread::spawn(move || loop {
            if let Ok(true) = event::poll(std::time::Duration::from_millis(10)) {
                let e = event::read().unwrap();
                if key_send.send(e).is_err() {
                    break;
                }
            }

            if thread_key_exit.load(Ordering::Relaxed) {
                break;
            }
        });

        let fps_refresh = Arc::new(AtomicBool::new(false));
        let fps_refresh_copy = fps_refresh.clone();
        let fps_schedule = self
            .timer
            .schedule_repeating(Duration::seconds(1), move || {
                fps_refresh_copy.store(true, Ordering::Release);
            });

        execute!(stdout, cursor::Hide).unwrap();

        {
            if let Some(dsp) = self.sys.get_display() {
                let (line1, line2) = dsp.get_output();
                execute!(
                    stdout,
                    Print(format!(
                        "┌────────────────┐\r\n│{}|\r\n|{}|\r\n└────────────────┘\r\n>\r",
                        line1, line2
                    )),
                    cursor::MoveUp(3)
                )
                .unwrap();
            }
        }

        self.epoch_start = Instant::now();
        loop {
            self.step_next();

            if let Ok(event) = key_recv.try_recv() {
                match event {
                    event::Event::Key(key_event) => match key_event.code {
                        event::KeyCode::Esc => {
                            break;
                        }
                        event::KeyCode::Char(c) => {
                            if let Some(con) = self.sys.get_controller() {
                                match c {
                                    'w' => {
                                        con.on_press(Button::Up);
                                        con.on_release(Button::Up);
                                    }
                                    's' => {
                                        con.on_press(Button::Down);
                                        con.on_release(Button::Down);
                                    }
                                    'a' => {
                                        con.on_press(Button::Left);
                                        con.on_release(Button::Left);
                                    }
                                    'd' => {
                                        con.on_press(Button::Right);
                                        con.on_release(Button::Right);
                                    }
                                    'j' => {
                                        con.on_press(Button::A);
                                        con.on_release(Button::A);
                                    }
                                    'k' => {
                                        con.on_press(Button::B);
                                        con.on_release(Button::B);
                                    }
                                    'l' => {
                                        con.on_press(Button::Select);
                                        con.on_release(Button::Select);
                                    }
                                    ';' => {
                                        con.on_press(Button::Start);
                                        con.on_release(Button::Start);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            if self.sys.is_halted() {
                break;
            }

            if self.breakpoints.contains(&(self.sys.get_cpu().pc - 1)) {
                if skip_breakpoints == 0 {
                    break;
                } else {
                    skip_breakpoints -= 1;
                }
            }

            if let Some(dsp) = self.sys.get_display() {
                if dsp.get_updated() || fps_refresh.load(Ordering::Acquire) {
                    fps_refresh.store(false, Ordering::Release);
                    let (line1, line2) = dsp.get_output();
                    execute!(
                        stdout,
                        Print(format!(
                            "│{}│\r\n│{}│\r\n\n> {:2.2?}MHz ({:?}ns)",
                            line1,
                            line2,
                            1000.0
                                / (self.avg_nanos_per_epoch / (CYCLES_PER_EPOCH * WINDOW_SIZE))
                                    as f32,
                            self.avg_nanos_per_epoch / (CYCLES_PER_EPOCH * WINDOW_SIZE)
                        )),
                        terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
                        cursor::MoveToColumn(0),
                        cursor::MoveUp(3)
                    )
                    .unwrap();
                }
            } else {
                if fps_refresh.load(Ordering::Acquire) {
                    fps_refresh.store(false, Ordering::Release);
                    execute!(
                        stdout,
                        Print(format!(
                            "\n> {:2.2?}MHz ({:?}ns)",
                            1000.0
                                / (self.avg_nanos_per_epoch / (CYCLES_PER_EPOCH * WINDOW_SIZE))
                                    as f32,
                            self.avg_nanos_per_epoch / (CYCLES_PER_EPOCH * WINDOW_SIZE)
                        )),
                        terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
                        cursor::MoveToColumn(0)
                    )
                    .unwrap();
                }
            }
        }

        execute!(stdout, cursor::Show, cursor::MoveDown(3)).unwrap();

        drop(cycle_schedule);
        drop(fps_schedule);
        key_exit.store(true, Ordering::Release)
    }

    pub fn run(&mut self) {
        self.run_n(1);
    }

    pub fn list_breakpoints(&self) {
        self.breakpoints.iter().enumerate().for_each(|(ix, bp)| {
            let sym = if let Some(s) = self.addr2sym.get(&bp) {
                s.clone()
            } else {
                format!("${:04x}", bp)
            };

            println!("{}: {}", ix, sym);
        });
    }

    pub fn add_breakpoint(&mut self, sym_or_addr: &str) {
        let addr = if let Some(&a) = self.sym2addr.get(sym_or_addr) {
            a
        } else {
            u16::from_str_radix(sym_or_addr, 16).unwrap()
        };

        match self.breakpoints.iter().position(|&bp| bp == addr) {
            Some(ix) => {
                println!("{}", ix);
            }
            None => {
                self.breakpoints.push(addr);
                println!("{}", self.breakpoints.len() - 1);
            }
        }
    }

    pub fn remove_breakpoint(&mut self, ix: usize) {
        self.breakpoints.remove(ix);
    }

    pub fn show_cpu(&mut self) {
        let cpu = self.sys.get_cpu();
        print!(
            "<{}> {:04x}: ",
            get_flag_string(cpu.p),
            cpu.pc.wrapping_sub(1)
        );

        self.show_instruction();

        let cpu = self.sys.get_cpu();
        println!();
        println!(
            "A:{:02x}       X:{:02x}       Y:{:02x}          S:{:02x}",
            cpu.a, cpu.x, cpu.y, cpu.s
        );
    }

    pub fn show_zp(&self) {
        let slice = &self.sys.get_ram().mem[0..0x100];
        show_bytes(slice, 0);
    }

    pub fn show_stack(&self) {
        let slice = &self.sys.get_ram().mem[0x100..0x200];
        show_bytes(slice, 0x100);
    }

    pub fn show_ram(&self) {
        let slice = &self.sys.get_ram().mem[0x200..];
        show_bytes(slice, 0x200);
    }

    pub fn show_dsp(&mut self) {
        if let Some(dsp) = self.sys.get_display() {
            let (line1, line2) = dsp.get_output();

            println!("┌────────────────┐");
            println!("│{}│", line1);
            println!("│{}│", line2);
            println!("└────────────────┘");
        }
    }

    pub fn show_per(&self) {
        if let Some(per) = self.sys.get_peripheral_controller() {
            println!(
                "PA:{:02x}[{:02x}]  PB:{:02x}[{:02x}]  T1:{:04x}/{:04x}  I:{:02x}[{:02x}]",
                per.ora,
                per.ddra,
                per.orb,
                per.ddrb,
                per.t1c,
                per.t1l,
                per.ifr.get(),
                per.ier
            );
        }
    }

    fn start_timer(&mut self) -> timer::Guard {
        *self.cycle_gate.1.lock().unwrap() = 0;

        let cycle_gate = self.cycle_gate.clone();
        self.timer
            .schedule_repeating(Duration::nanoseconds(CYCLE_NANOSECONDS as i64), move || {
                let (cond, mutex) = &*cycle_gate.clone();
                *mutex.lock().unwrap() += 1;
                cond.notify_all();
            })
    }

    fn step_next(&mut self) {
        self.cycle();
        while self.sys.get_cpu().tcu != 1 {
            self.cycle();
        }
    }

    fn cycle(&mut self) {
        self.cycle_count = self.cycle_count.wrapping_add(1);
        if self.cycle_count % CYCLES_PER_EPOCH == 0 {
            let now = Instant::now();
            self.avg_nanos_per_epoch = ((self.avg_nanos_per_epoch * (WINDOW_SIZE - 1))
                / WINDOW_SIZE)
                + (now.duration_since(self.epoch_start).as_nanos() as u64);
            self.epoch_start = now;
        }

        if !self.bench {
            let (cond, mutex) = &*self.cycle_gate;
            let _ = cond
                .wait_while(mutex.lock().unwrap(), |c| {
                    if *c == 0 {
                        true
                    } else {
                        *c -= 1;
                        false
                    }
                })
                .unwrap();
        }

        self.sys.cycle();
    }

    pub fn read_symbols(&mut self, path: &str) {
        for line in std::fs::read_to_string(path).unwrap().lines() {
            let mut words = line.split_ascii_whitespace();
            words.next().unwrap();
            let addr = u16::from_str_radix(words.next().unwrap(), 16).unwrap();
            let mut sym = words.next().unwrap().to_string();
            sym.remove(0);
            self.sym2addr.insert(sym.clone(), addr);
            self.addr2sym.insert(addr, sym);
        }
    }

    fn show_instruction(&mut self) {
        let cpu = self.sys.get_cpu();

        let (opcode, address_mode) = cpu.ir;
        let pc = cpu.pc;

        let arg8 = self.sys.peek(pc);
        let arg16 = (arg8 as u16) | ((self.sys.peek(pc + 1) as u16) << 8);

        let sym = if let Some(s) = self.addr2sym.get(&arg16) {
            s.clone()
        } else {
            format!("${:04x}", arg16)
        };

        print!("{:?}", opcode);

        match address_mode {
            mos6502::AddressMode::Absolute => print!(" {}", sym),
            mos6502::AddressMode::AbsoluteIndexedIndirect => print!(" ({},x)", sym),
            mos6502::AddressMode::AbsoluteIndexedWithX => print!(" {},x", sym),
            mos6502::AddressMode::AbsoluteIndexedWithY => print!(" {},y", sym),
            mos6502::AddressMode::AbsoluteIndirect => print!(" ({})", sym),
            mos6502::AddressMode::Accumulator => {}
            mos6502::AddressMode::ImmediateAddressing => print!(" #${:02x}", arg8),
            mos6502::AddressMode::Implied => {}
            mos6502::AddressMode::ProgramCounterRelative => print!(" #${:02x}", arg8),
            mos6502::AddressMode::Stack => {}
            mos6502::AddressMode::ZeroPage => print!(" ${:02x}", arg8),
            mos6502::AddressMode::ZeroPageIndexedIndirect => print!(" (${:02x},x)", arg8),
            mos6502::AddressMode::ZeroPageIndexedWithX => print!(" ${:02x},x", arg8),
            mos6502::AddressMode::ZeroPageIndexedWithY => print!(" ${:02x},y", arg8),
            mos6502::AddressMode::ZeroPageIndirect => print!(" (${:02x})", arg8),
            mos6502::AddressMode::ZeroPageIndirectIndexedWithY => print!(" (${:02x},y)", arg8),
        }
    }
}

fn get_flag_string(flags: u8) -> String {
    let names = ['C', 'Z', 'I', 'D', 'B', '-', 'O', 'N'];
    (0..8)
        .rev()
        .map(|i| if (flags >> i) & 1 == 1 { names[i] } else { '-' })
        .collect()
}

fn show_bytes(source: &[u8], offset: usize) {
    let mut eliding = false;

    let chunks = source.as_ref().chunks(16);

    for (i, row) in chunks.enumerate() {
        if row.iter().all(|x| *x == 255) {
            if !eliding {
                println!("*");
                eliding = true;
            }
            continue;
        } else {
            eliding = false;
        }

        print!("{:04x}:   ", (i * 16) + offset);

        for (i, x) in row.as_ref().iter().enumerate() {
            print!(
                "{:02x}{}",
                x,
                match i + 1 {
                    n if n == row.as_ref().len() => "",
                    n if n % 4 == 0 => "  ",
                    _ => " ",
                }
            );
        }

        let pad = 16 - row.len();
        let pad = pad * 3 + pad / 4;
        for _ in 0..pad {
            print!(" ");
        }
        print!("   ");

        for x in row {
            print!(
                "{}",
                if x.is_ascii() && !x.is_ascii_control() {
                    (*x).into()
                } else {
                    '.'
                }
            );
        }

        println!("");
    }
}
