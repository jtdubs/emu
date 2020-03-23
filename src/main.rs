use std::rc::Rc;
use std::sync::Mutex;

mod components;

use components::*;

fn main() {
    env_logger::init();

    let mut clk = Clock::new();
    let cpu = Rc::new(Mutex::new(W65C02S::new()));
    let ram = Rc::new(Mutex::new(RAM::new(0x4000)));
    let rom = Rc::new(Mutex::new(ROM::load("rom.bin")));
    let per = Rc::new(Mutex::new(W65C22::new()));
    let dsp = Rc::new(Mutex::new(HD44780U::new()));

    clk.attach(cpu.clone());
    clk.attach(per.clone());
    clk.attach(dsp.clone());

    let mut c = cpu.lock().unwrap();
    c.attach(0xC000, 0x0000, ram);
    c.attach(0x8000, 0x8000, rom);
    c.attach(0xFFF0, 0x6000, per.clone());

    let mut p = per.lock().unwrap();
    p.attach_b(dsp);

    while !cpu.lock().unwrap().is_halted() {
        clk.step();
    }
}
