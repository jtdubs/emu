use crate::component::hd44780::{RegisterSelector, HD44780};
use crate::component::mos6502::{Bus, MOS6502};
use crate::component::mos6522::{Ports, MOS6522};
use crate::component::snes_controller::SNESController;
use crate::component::{RAM, ROM};
use crate::system::System;

pub struct BreadboardSystem {
    cpu: MOS6502<SystemBus>,
}

impl BreadboardSystem {
    pub fn new(rom_path: &str) -> BreadboardSystem {
        BreadboardSystem {
            cpu: MOS6502::new(SystemBus::new(rom_path)),
        }
    }
}

impl System for BreadboardSystem {
    type BusType = SystemBus;
    type PortsType = Peripherals;

    fn is_halted(&self) -> bool {
        self.cpu.is_halted()
    }

    fn get_cpu(&self) -> &MOS6502<Self::BusType> {
        &self.cpu
    }

    fn get_display(&mut self) -> Option<&mut HD44780> {
        Some(&mut self.cpu.bus.per.ports.dsp)
    }

    fn get_ram(&self) -> &RAM {
        &self.cpu.bus.ram
    }

    fn get_controller(&mut self) -> Option<&mut SNESController> {
        Some(&mut self.cpu.bus.per.ports.con)
    }

    fn get_peripheral_controller(&self) -> Option<&MOS6522<Self::PortsType>> {
        Some(&self.cpu.bus.per)
    }

    fn peek(&mut self, addr: u16) -> u8 {
        self.cpu.bus.peek(addr)
    }

    fn cycle(&mut self) {
        self.cpu.cycle();
        let per_int = self.cpu.bus.per.cycle();
        self.cpu.set_interrupt(per_int);
        self.cpu.bus.per.ports.dsp.cycle();
    }
}

pub struct SystemBus {
    pub per: MOS6522<Peripherals>,
    pub ram: RAM,
    pub rom: ROM,
}

impl SystemBus {
    const ROM_SELECTOR: (u16, u16) = (0x8000, 0x8000);
    const RAM_SELECTOR: (u16, u16) = (0xC000, 0x0000);
    const PER_SELECTOR: (u16, u16) = (0xFFF0, 0x6000);

    pub fn new(rom_path: &str) -> SystemBus {
        SystemBus {
            rom: ROM::load(rom_path),
            ram: RAM::new(0x4000),
            per: MOS6522::new(Peripherals::new()),
        }
    }
}

impl Bus for SystemBus {
    fn peek(&self, addr: u16) -> u8 {
        if addr & Self::ROM_SELECTOR.0 == Self::ROM_SELECTOR.1 {
            self.rom.peek(addr & !Self::ROM_SELECTOR.0)
        } else if addr & Self::RAM_SELECTOR.0 == Self::RAM_SELECTOR.1 {
            self.ram.peek(addr & !Self::RAM_SELECTOR.0)
        } else if addr & Self::PER_SELECTOR.0 == Self::PER_SELECTOR.1 {
            self.per.peek(addr & !Self::PER_SELECTOR.0)
        } else {
            panic!("peek at unmapped address: {:02x}", addr);
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        if addr & Self::ROM_SELECTOR.0 == Self::ROM_SELECTOR.1 {
            self.rom.read(addr & !Self::ROM_SELECTOR.0)
        } else if addr & Self::RAM_SELECTOR.0 == Self::RAM_SELECTOR.1 {
            self.ram.read(addr & !Self::RAM_SELECTOR.0)
        } else if addr & Self::PER_SELECTOR.0 == Self::PER_SELECTOR.1 {
            self.per.read(addr & !Self::PER_SELECTOR.0)
        } else {
            panic!("read at unmapped address: {:02x}", addr);
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        if addr & Self::ROM_SELECTOR.0 == Self::ROM_SELECTOR.1 {
            self.rom.write(addr & !Self::ROM_SELECTOR.1, val);
        } else if addr & Self::RAM_SELECTOR.0 == Self::RAM_SELECTOR.1 {
            self.ram.write(addr & !Self::RAM_SELECTOR.1, val);
        } else if addr & Self::PER_SELECTOR.0 == Self::PER_SELECTOR.1 {
            self.per.write(addr & !Self::PER_SELECTOR.0, val);
        } else {
            panic!("write at unmapped address: {:02x}", addr);
        }
    }
}

pub struct Peripherals {
    pub dsp: HD44780,
    pub con: SNESController,
    a_cache: u8,
    b_cache: u8,
}

impl Peripherals {
    pub fn new() -> Peripherals {
        Peripherals {
            dsp: HD44780::new(),
            con: SNESController::new(),
            a_cache: 0,
            b_cache: 0,
        }
    }

    fn get_dsp_pins(&self) -> (RegisterSelector, bool, bool) {
        const RS: u8 = 0x20;
        const RW: u8 = 0x40;
        const E: u8 = 0x80;

        (
            if self.a_cache & RS == RS {
                RegisterSelector::Data
            } else {
                RegisterSelector::Instruction
            },
            self.a_cache & RW == RW,
            self.a_cache & E == E,
        )
    }

    fn get_con_pins(&self) -> (bool, bool) {
        const LATCH: u8 = 0x02;
        const CLK: u8 = 0x04;

        (self.a_cache & LATCH == LATCH, self.a_cache & CLK == CLK)
    }

    fn do_write(&mut self) {
        let (rs, rw, e) = self.get_dsp_pins();
        self.dsp.write(rs, rw, e, self.b_cache);

        let (latch, clk) = self.get_con_pins();
        self.con.write(latch, clk);
    }
}

impl Ports for Peripherals {
    fn peek_a(&self) -> u8 {
        self.con.peek() & 0x07
    }

    fn read_a(&mut self) -> u8 {
        self.con.read() & 0x07
    }

    fn write_a(&mut self, val: u8) {
        self.a_cache = val;
        self.do_write();
    }

    fn peek_b(&self) -> u8 {
        let (rs, rw, e) = self.get_dsp_pins();
        self.dsp.peek(rs, rw, e)
    }

    fn read_b(&mut self) -> u8 {
        let (rs, rw, e) = self.get_dsp_pins();
        self.dsp.read(rs, rw, e)
    }

    fn write_b(&mut self, val: u8) {
        self.b_cache = val;
        self.do_write();
    }
}
