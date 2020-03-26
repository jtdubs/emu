use signal_hook;
use std::collections::HashMap;
use std::io::{stdout, Read, Write};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use termion::raw::IntoRawMode;
//use termion::event::{Key, Event, MouseEvent};
// use termion::input::{TermRead};

use crate::components::*;

pub struct System {
    pub clk: Clock,
    pub cpu: Rc<Mutex<W65C02S>>,
    pub ram: Rc<Mutex<RAM>>,
    pub rom: Rc<Mutex<ROM>>,
    pub per: Rc<Mutex<W65C22>>,
    pub ada: Rc<Mutex<HD44780UAdapter>>,
    pub dsp: Rc<Mutex<HD44780U>>,
    pub con: Rc<Mutex<SNESController>>,
    pub breakpoints: Vec<u16>,
    sigterm: Arc<AtomicBool>,
    sym2addr: HashMap<String, u16>,
    addr2sym: HashMap<u16, String>,
}

impl System {
    pub fn new(rom_path: &str, sym_path: &str) -> System {
        let mut sys = System {
            clk: Clock::new(),
            cpu: Rc::new(Mutex::new(W65C02S::new())),
            ram: Rc::new(Mutex::new(RAM::new(0x4000))),
            rom: Rc::new(Mutex::new(ROM::load(rom_path))),
            per: Rc::new(Mutex::new(W65C22::new())),
            ada: Rc::new(Mutex::new(HD44780UAdapter::new())),
            dsp: Rc::new(Mutex::new(HD44780U::new())),
            con: Rc::new(Mutex::new(SNESController::new())),
            breakpoints: Vec::new(),
            sigterm: Arc::new(AtomicBool::new(false)),
            sym2addr: HashMap::new(),
            addr2sym: HashMap::new(),
        };

        sys.read_symbols(sym_path);

        signal_hook::flag::register(signal_hook::SIGINT, Arc::clone(&sys.sigterm)).unwrap();

        sys.clk.attach(sys.cpu.clone());
        sys.clk.attach(sys.per.clone());
        sys.clk.attach(sys.dsp.clone());

        {
            let mut c = sys.cpu.lock().unwrap();
            c.attach(0xC000, 0x0000, sys.ram.clone());
            c.attach(0x8000, 0x8000, sys.rom.clone());
            c.attach(0xFFF0, 0x6000, sys.per.clone());
        }

        {
            let mut p = sys.per.lock().unwrap();
            p.attach_a(0xE0, sys.ada.clone());
            p.attach_a(0x07, sys.con.clone());
            p.attach_b(0xFF, sys.ada.clone());
        }

        {
            let mut a = sys.ada.lock().unwrap();
            a.attach(sys.dsp.clone());
        }

        sys
    }

    pub fn step(&mut self) {
        self.clk.cycle();
        while self.cpu.lock().unwrap().tcu != 1 {
            self.clk.cycle();
        }
    }

    pub fn step_over(&mut self) {
        if self.cpu.lock().unwrap().ir.0 == cpu::Instruction::JSR {
            self.breakpoints.push(self.cpu.lock().unwrap().pc + 2);
            self.run();
            self.breakpoints.pop();
        } else {
            self.step();
        }
    }

    pub fn step_out(&mut self) {
        let mut depth: i32 = 0;

        self.sigterm.store(false, Ordering::Relaxed);

        loop {
            match self.cpu.lock().unwrap().ir.0 {
                cpu::Instruction::JSR => depth += 1,
                cpu::Instruction::RTS => depth -= 1,
                _ => {}
            }

            self.step();

            if self.sigterm.load(Ordering::Relaxed) {
                println!();
                break;
            }

            if self.cpu.lock().unwrap().is_halted() {
                break;
            }

            if depth < 0 {
                break;
            }
        }
    }

    pub fn run(&mut self) {
        let mut i: u32 = 0;
        self.sigterm.store(false, Ordering::Relaxed);

        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut stdin = termion::async_stdin();
        let mut buffer = [0u8; 8];

        write!(stdout, "{}", termion::cursor::Hide).unwrap();

        {
            let dsp = self.dsp.lock().unwrap();
            let (line1, line2) = dsp.get_output();
            write!(
                stdout,
                "┌────────────────┐\r\n│{}│\r\n│{}│\r\n└────────────────┘\r{}",
                line1, line2, termion::cursor::Up(1)
            )
            .unwrap();
            stdout.flush().unwrap();
        }

        'run_loop: loop {
            self.step();

            if self.sigterm.load(Ordering::Relaxed) {
                break;
            }

            if let Ok(x) = stdin.read(&mut buffer) {
                for &c in buffer[0..x].iter() {
                    match c as char {
                        '\x03' => { break 'run_loop; }
                        '\x1B' => { break 'run_loop; }
                        'w' => { write!(stdout, "\nUP─────────────\r{}", termion::cursor::Up(1)).unwrap(); }
                        'a' => { write!(stdout, "\nLEFT───────────\r{}", termion::cursor::Up(1)).unwrap(); }
                        's' => { write!(stdout, "\nDOWN───────────\r{}", termion::cursor::Up(1)).unwrap(); }
                        'd' => { write!(stdout, "\nRIGHT──────────\r{}", termion::cursor::Up(1)).unwrap(); }
                        x =>   { write!(stdout, "\nUNKNOWN: {:02x}\r{}", x as u8, termion::cursor::Up(1)).unwrap(); }
                        _ => {}
                    }
                }
            }

            if self.cpu.lock().unwrap().is_halted() {
                break;
            }

            if self
                .breakpoints
                .contains(&(self.cpu.lock().unwrap().pc - 1))
            {
                break;
            }

            i = i.wrapping_add(0);
            if i % 40 == 0 {
                let mut dsp = self.dsp.lock().unwrap();
                if dsp.get_updated() {
                    let (line1, line2) = dsp.get_output();
                    write!(
                        stdout,
                        "{}│{}│\r\n│{}│\r",
                        termion::cursor::Up(1),
                        line1,
                        line2
                    )
                    .unwrap();
                }
            }
        }

        write!(
            stdout,
            "{}{}\n",
            termion::cursor::Show,
            termion::cursor::Down(3)
        )
        .unwrap();
        stdout.flush().unwrap();
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

    pub fn show_cpu(&self) {
        let cpu = self.cpu.lock().unwrap();
        print!("<{}> {:04x}: ", get_flag_string(cpu.p), cpu.pc);
        self.show_instruction(&cpu);
        println!();
        println!(
            "A:{:02x}       X:{:02x}       Y:{:02x}          S:{:02x}",
            cpu.a, cpu.x, cpu.y, cpu.s
        );
    }

    pub fn show_zp(&self) {
        let ram = self.ram.lock().unwrap();
        let slice = &ram.mem[0..0x100];
        show_bytes(slice, 0);
    }

    pub fn show_stack(&self) {
        let ram = self.ram.lock().unwrap();
        let slice = &ram.mem[0x100..0x200];
        show_bytes(slice, 0x100);
    }

    pub fn show_ram(&self) {
        let ram = self.ram.lock().unwrap();
        let slice = &ram.mem[0x200..];
        show_bytes(slice, 0x200);
    }

    pub fn show_dsp(&self) {
        let dsp = self.dsp.lock().unwrap();
        let (line1, line2) = dsp.get_output();

        println!("┌────────────────┐");
        println!("│{}│", line1);
        println!("│{}│", line2);
        println!("└────────────────┘");
    }

    pub fn show_per(&self) {
        let per = self.per.lock().unwrap();
        println!(
            "PA:{:02x}[{:02x}]  PB:{:02x}[{:02x}]  T1:{:04x}/{:04x}  I:{:02x}[{:02x}]",
            per.ora, per.ddra, per.orb, per.ddrb, per.t1c, per.t1l, per.ifr, per.ier
        );
    }

    fn read_symbols(&mut self, path: &str) {
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

    fn show_instruction(&self, cpu: &W65C02S) {
        let (opcode, address_mode) = &cpu.ir;

        let arg8 = cpu.peek(cpu.pc);
        let arg16 = (arg8 as u16) | ((cpu.peek(cpu.pc + 1) as u16) << 8);

        let sym = if let Some(s) = self.addr2sym.get(&arg16) {
            s.clone()
        } else {
            format!("${:04x}", arg16)
        };

        print!("{:?}", opcode);

        match address_mode {
            cpu::AddressMode::Absolute => print!(" {}", sym),
            cpu::AddressMode::AbsoluteIndexedIndirect => print!(" ({},x)", sym),
            cpu::AddressMode::AbsoluteIndexedWithX => print!(" {},x", sym),
            cpu::AddressMode::AbsoluteIndexedWithY => print!(" {},y", sym),
            cpu::AddressMode::AbsoluteIndirect => print!(" ({})", sym),
            cpu::AddressMode::Accumulator => {}
            cpu::AddressMode::ImmediateAddressing => print!(" #${:02x}", arg8),
            cpu::AddressMode::Implied => {}
            cpu::AddressMode::ProgramCounterRelative => print!(" #${:02x}", arg8),
            cpu::AddressMode::Stack => {}
            cpu::AddressMode::ZeroPage => print!(" ${:02x}", arg8),
            cpu::AddressMode::ZeroPageIndexedIndirect => print!(" (${:02x},x)", arg8),
            cpu::AddressMode::ZeroPageIndexedWithX => print!(" ${:02x},x", arg8),
            cpu::AddressMode::ZeroPageIndexedWithY => print!(" ${:02x},y", arg8),
            cpu::AddressMode::ZeroPageIndirect => print!(" (${:02x})", arg8),
            cpu::AddressMode::ZeroPageIndirectIndexedWithY => print!(" (${:02x},y)", arg8),
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
        if row.iter().all(|x| *x == 0) {
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
