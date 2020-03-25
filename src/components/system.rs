use std::rc::Rc;
use std::sync::Mutex;

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

    pub fn show(&self) {
        println!("CPU: {:x?}", self.cpu.lock().unwrap());
        println!("PER: {:x?}", self.per.lock().unwrap());
        println!("DSP: {:x?}", self.dsp.lock().unwrap());
        println!("CON: {:x?}", self.con.lock().unwrap());
    }
}
