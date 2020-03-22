mod bus;
mod ram;
mod rom;
mod w65c02s;
mod w65c22;

use ram::RAM;
use rom::ROM;
use w65c02s::W65C02S;
use w65c22::W65C22;

fn main() {
    env_logger::init();

    let periph = W65C22::new();

    let mut cpu = W65C02S::new();
    cpu.attach(0xC000, 0x0000, Box::new(RAM::new(0x4000)));
    cpu.attach(0x8000, 0x8000, Box::new(ROM::load("rom.bin")));
    cpu.attach(0xFFF0, 0x6000, Box::new(periph));

    while !cpu.is_halted() {
        cpu.step();
    }
}
