use std::collections::HashMap;
use std::io::{stdout, Read, Write};
use std::time::Instant;
use termion::raw::IntoRawMode;

use crate::components::*;

const CYCLES_PER_EPOCH: u64 = 1000000;
const WINDOW_SIZE: u64 = 20;

pub struct System {
    pub cpu: W65C02S,
    pub breakpoints: Vec<u16>,
    pub sym2addr: HashMap<String, u16>,
    pub addr2sym: HashMap<u16, String>,
    pub cycle_count: u64,
    pub epoch_start: Instant,
    pub avg_nanos_per_epoch: u64,
}

impl System {
    pub fn new(rom_path: &str, sym_path: &str) -> System {
        let mut sys = System {
            cpu: W65C02S::new(rom_path),
            breakpoints: Vec::new(),
            sym2addr: HashMap::new(),
            addr2sym: HashMap::new(),
            cycle_count: 0,
            epoch_start: Instant::now(),
            avg_nanos_per_epoch: CYCLES_PER_EPOCH * WINDOW_SIZE,
        };

        sys.read_symbols(sym_path);

        sys
    }

    pub fn step(&mut self) {
        self.cycle();
        while self.cpu.tcu != 1 {
            self.cycle();
        }
    }

    pub fn cycle(&mut self) {
        self.cycle_count = self.cycle_count.wrapping_add(1);
        if self.cycle_count % CYCLES_PER_EPOCH == 0 {
            let now = Instant::now();
            self.avg_nanos_per_epoch = ((self.avg_nanos_per_epoch * (WINDOW_SIZE - 1)) / WINDOW_SIZE) + (now.duration_since(self.epoch_start).as_nanos() as u64);
            self.epoch_start = now;
        }

        self.cpu.cycle();
    }

    pub fn step_over(&mut self) {
        if self.cpu.ir.0 == cpu::Instruction::JSR {
            self.breakpoints.push(self.cpu.pc + 2);
            self.run();
            self.breakpoints.pop();
        } else {
            self.step();
        }
    }

    pub fn step_out(&mut self) {
        let mut depth: i32 = 0;

        loop {
            match self.cpu.ir.0 {
                cpu::Instruction::JSR => depth += 1,
                cpu::Instruction::RTS => depth -= 1,
                _ => {}
            }

            self.step();

            if self.cpu.is_halted() {
                break;
            }

            if depth < 0 {
                break;
            }
        }
    }

    pub fn run_headless(&mut self) {
        loop {
            self.step();

            {
                if self.cpu.is_halted() {
                    break;
                }

                if self
                    .breakpoints
                    .contains(&(self.cpu.pc - 1))
                {
                    break;
                }
            }
        }
    }

    pub fn run(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let mut stdin = termion::async_stdin_until(0x1B);
        let mut buffer = [0u8; 8];

        write!(stdout, "{}", termion::cursor::Hide).unwrap();

        {
            let (line1, line2) = self.cpu.per.ada.dsp.get_output();
            write!(
                stdout,
                "┌────────────────┐\r\n│{}│\r\n│{}│\r\n└────────────────┘\r\n>\r{}",
                line1,
                line2,
                termion::cursor::Up(3)
            )
            .unwrap();
            stdout.flush().unwrap();
        }

        self.epoch_start = Instant::now();
        loop {
            self.step();

            match stdin.read(&mut buffer) {
                Ok(1) => {
                    if buffer[0] == 0x1B {
                        break;
                    } else {
                        self.cpu.per.con.on_key(buffer[0] as char);
                    }
                }
                Ok(_) => {}
                Err(_) => {
                    break;
                }
            }

            if self.cpu.is_halted() {
                break;
            }

            if self
                .breakpoints
                .contains(&(self.cpu.pc - 1))
            {
                break;
            }

            let dsp = &mut self.cpu.per.ada.dsp;
            if dsp.get_updated() {
                let (line1, line2) = dsp.get_output();
                write!(
                    stdout,
                    "│{}│\r\n│{}│\r\n\n> {:2.2?}MHz ({:?}ns){}\r{}",
                    line1,
                    line2,
                    1000.0 / (self.avg_nanos_per_epoch / (CYCLES_PER_EPOCH * WINDOW_SIZE)) as f32,
                    self.avg_nanos_per_epoch / (CYCLES_PER_EPOCH * WINDOW_SIZE),
                    termion::clear::AfterCursor,
                    termion::cursor::Up(3),
                )
                .unwrap();
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
        print!("<{}> {:04x}: ", get_flag_string(self.cpu.p), self.cpu.pc);
        self.show_instruction(&self.cpu);
        println!();
        println!(
            "A:{:02x}       X:{:02x}       Y:{:02x}          S:{:02x}",
            self.cpu.a, self.cpu.x, self.cpu.y, self.cpu.s
        );
    }

    pub fn show_zp(&self) {
        let slice = &self.cpu.ram.mem[0..0x100];
        show_bytes(slice, 0);
    }

    pub fn show_stack(&self) {
        let slice = &self.cpu.ram.mem[0x100..0x200];
        show_bytes(slice, 0x100);
    }

    pub fn show_ram(&self) {
        let slice = &self.cpu.ram.mem[0x200..];
        show_bytes(slice, 0x200);
    }

    pub fn show_dsp(&self) {
        let (line1, line2) = self.cpu.per.ada.dsp.get_output();

        println!("┌────────────────┐");
        println!("│{}│", line1);
        println!("│{}│", line2);
        println!("└────────────────┘");
    }

    pub fn show_per(&self) {
        let per = &self.cpu.per;
        println!(
            "PA:{:02x}[{:02x}]  PB:{:02x}[{:02x}]  T1:{:04x}/{:04x}  I:{:02x}[{:02x}]",
            per.ora, per.ddra, per.orb, per.ddrb, per.t1c, per.t1l, per.ifr.get(), per.ier
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
