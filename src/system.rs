use std::rc::Rc;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use signal_hook;

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
    sigterm: Arc<AtomicBool>,
    pub breakpoints: Vec<u16>
}

impl System {
    pub fn new() -> System {
        let mut sys = System {
            clk: Clock::new(),
            cpu: Rc::new(Mutex::new(W65C02S::new())),
            ram: Rc::new(Mutex::new(RAM::new(0x4000))),
            rom: Rc::new(Mutex::new(ROM::load("rom.bin"))),
            per: Rc::new(Mutex::new(W65C22::new())),
            ada: Rc::new(Mutex::new(HD44780UAdapter::new())),
            dsp: Rc::new(Mutex::new(HD44780U::new())),
            con: Rc::new(Mutex::new(SNESController::new())),
            sigterm: Arc::new(AtomicBool::new(false)),
            breakpoints: Vec::new(),
        };

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
            self.breakpoints.push(self.cpu.lock().unwrap().pc + 3);
            self.run();
            self.breakpoints.pop();
        } else {
            self.step();
        }
    }

    pub fn run(&mut self) {
        self.sigterm.store(false, Ordering::Relaxed);

        loop {
            self.step();

            if self.sigterm.load(Ordering::Relaxed) {
                println!();
                break;
            }

            if self.cpu.lock().unwrap().is_halted() {
                break;
            }

            if self.breakpoints.contains(&self.cpu.lock().unwrap().pc) {
                break;
            }
        }
    }

    pub fn list_breakpoints(&self) {
        self.breakpoints.iter().enumerate().for_each(|(ix, bp)| {
            println!("{}: {:04x}", ix, bp);
        });
    }

    pub fn add_breakpoint(&mut self, addr : u16) {
        match self.breakpoints.iter().position(|&bp| bp == addr) {
            Some(ix) => { println!("{}", ix); }
            None => {
                self.breakpoints.push(addr);
                println!("{}", self.breakpoints.len()-1);
            }
        }
    }

    pub fn remove_breakpoint(&mut self, ix : usize) {
        self.breakpoints.remove(ix);
    }

    pub fn show_cpu(&self) {
        let cpu = self.cpu.lock().unwrap();
        print!(
            "<{}> {:04x}: ",
            get_flag_string(cpu.p),
            cpu.pc
        );
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

        println!("S:{:?} A:{:02x}", dsp.state, dsp.addr);
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

    fn show_instruction(&self, cpu: &W65C02S) {
        let (opcode, address_mode) = &cpu.ir;

        let arg8 = cpu.peek(cpu.pc);
        let arg16 = (arg8 as u16) | ((cpu.peek(cpu.pc+1) as u16) << 8);

        print!("{:?}", opcode);

        match address_mode {
            cpu::AddressMode::Absolute => print!(" ${:04x}", arg16),
            cpu::AddressMode::AbsoluteIndexedIndirect => print!(" (${:04x},x)", arg16),
            cpu::AddressMode::AbsoluteIndexedWithX => print!(" ${:04x},x", arg16),
            cpu::AddressMode::AbsoluteIndexedWithY => print!(" ${:04x},y", arg16),
            cpu::AddressMode::AbsoluteIndirect => print!(" (${:04x})", arg16),
            cpu::AddressMode::Accumulator => {}
            cpu::AddressMode::ImmediateAddressing => print!(" #${:02x}", arg8),
            cpu::AddressMode::Implied => {}
            cpu::AddressMode::ProgramCounterRelative => {}
            cpu::AddressMode::Stack => {}
            cpu::AddressMode::ZeroPage => print!("  ${:02x}", arg8),
            cpu::AddressMode::ZeroPageIndexedIndirect => print!("  (${:02x},x)", arg8),
            cpu::AddressMode::ZeroPageIndexedWithX => print!("  ${:02x},x", arg8),
            cpu::AddressMode::ZeroPageIndexedWithY => print!("  ${:02x},y", arg8),
            cpu::AddressMode::ZeroPageIndirect => print!("  (${:02x})", arg8),
            cpu::AddressMode::ZeroPageIndirectIndexedWithY => print!("  (${:02x},y)", arg8),
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
