mod memory;
mod w65c02s;

use memory::Memory;
use w65c02s::W65C02S;

fn main() {
    env_logger::init();

    let mut m = Memory::new();
    m.map(0x8000, "rom.bin");

    let mut cpu = W65C02S::new();
    while !cpu.is_halted() {
        cpu.step(&mut m);
    }
}
