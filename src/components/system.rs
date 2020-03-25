use std::rc::Rc;
use std::sync::Mutex;
use pretty_hex::*;

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
        };

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

    pub fn show_cpu(&self) {
        let cpu = self.cpu.lock().unwrap();
        println!("{} {:04X}> {:?} [{:?}]", get_flag_string(cpu.p), cpu.pc, cpu.ir.0, cpu.ir.1);
        println!("A:{:02X}  X:{:02X}  Y:{:02X}  S:{:02X}", cpu.a, cpu.x, cpu.y, cpu.s);
    }

    pub fn show_zp(&self) {
        let ram = self.ram.lock().unwrap();
        let slice = &ram.mem[0..0x100];
        println!("{:?}", slice.hex_dump())
    }

    pub fn show_stack(&self) {
        let ram = self.ram.lock().unwrap();
        let slice = &ram.mem[0x100..0x200];
        println!("{:?}", slice.hex_dump())
    }

    pub fn show_dsp(&self) {
        let dsp = self.dsp.lock().unwrap();
        let (line1, line2) = dsp.get_output();

        println!("S:{:?} A:{:02X}", dsp.state, dsp.addr);
        println!("┌────────────────┐");
        println!("│{}│", line1);
        println!("│{}│", line2);
        println!("└────────────────┘");
    }

        // println!("PER: {:x?}", self.per.lock().unwrap());
}

pub fn get_flag_string(flags : u8) -> String {
    let names = ['C', 'Z', 'I', 'D', 'B', '-', 'O', 'N'];
    (0..8).rev().map(|i| if (flags >> i) & 1 == 1 { names[i] } else { '-' }).collect()
}
