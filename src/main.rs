mod bus;
mod ram;
mod rom;
mod w65c02s;
mod w65c22;

use bus::Bus;
use ram::RAM;
use rom::ROM;
use w65c02s::W65C02S;
use w65c22::W65C22;

fn main() {
    env_logger::init();

    let mut rom = ROM::new(0x8000);
    rom.map(0x0000, "rom.bin");

    let ram = RAM::new(0x4000);
    let periph = W65C22::new();

    let mut bus = Bus::new();
    bus.add_member(0xC000, 0x0000, Box::new(ram));
    bus.add_member(0x8000, 0x8000, Box::new(rom));
    bus.add_member(0xFFF0, 0x6000, Box::new(periph));

    let mut cpu = W65C02S::new(bus);
    while !cpu.is_halted() {
        cpu.step();
    }
}
