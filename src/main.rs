mod cpu;
mod ram;
mod rom;
mod periph;
mod display;

fn main() {
    env_logger::init();

    let mut periph = periph::W65C22::new();
    periph.attach_b(Box::new(display::HD44780U::new()));

    let mut cpu = cpu::W65C02S::new();
    cpu.attach(0xC000, 0x0000, Box::new(ram::RAM::new(0x4000)));
    cpu.attach(0x8000, 0x8000, Box::new(rom::ROM::load("rom.bin")));
    cpu.attach(0xFFF0, 0x6000, Box::new(periph));

    while !cpu.is_halted() {
        cpu.step();
    }
}
