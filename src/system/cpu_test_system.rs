use crate::component::hd44780::HD44780;
use crate::component::mos6502::{Bus, MOS6502};
use crate::component::mos6522::{Ports, MOS6522};
use crate::component::snes_controller::SNESController;
use crate::component::RAM;
use crate::system::System;

pub struct CPUTestSystem {
    cpu: MOS6502<SystemBus>,
}

impl CPUTestSystem {
    pub fn new(ram_path: &str, entry_address: u16) -> CPUTestSystem {
        CPUTestSystem {
            cpu: MOS6502::new(SystemBus::new(ram_path, entry_address)),
        }
    }
}

impl System for CPUTestSystem {
    type BusType = SystemBus;
    type PortsType = NullPorts;

    fn is_halted(&self) -> bool {
        self.cpu.is_halted()
    }

    fn get_cpu(&self) -> &MOS6502<Self::BusType> {
        &self.cpu
    }

    fn get_display(&mut self) -> Option<&mut HD44780> {
        None
    }

    fn get_ram(&self) -> &RAM {
        &self.cpu.bus.ram
    }

    fn get_controller(&mut self) -> Option<&mut SNESController> {
        None
    }

    fn get_peripheral_controller(&self) -> Option<&MOS6522<Self::PortsType>> {
        None
    }

    fn peek(&mut self, addr: u16) -> u8 {
        self.cpu.bus.peek(addr)
    }

    fn cycle(&mut self) {
        self.cpu.cycle();
    }
}

pub struct SystemBus {
    pub ram: RAM,
}

impl SystemBus {
    pub fn new(ram_path: &str, entry_address: u16) -> SystemBus {
        let mut ram = RAM::load(ram_path);
        ram.mem[0xfffc] = (entry_address & 0xff) as u8;
        ram.mem[0xfffd] = ((entry_address >> 8) & 0xff) as u8;
        SystemBus { ram: ram }
    }
}

impl Bus for SystemBus {
    fn peek(&self, addr: u16) -> u8 {
        self.ram.peek(addr)
    }

    fn read(&mut self, addr: u16) -> u8 {
        self.ram.read(addr)
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.ram.write(addr, val);
    }
}

pub struct NullPorts {}

impl Ports for NullPorts {
    fn peek_a(&self) -> u8 {
        unimplemented!();
    }
    fn read_a(&mut self) -> u8 {
        unimplemented!();
    }
    fn write_a(&mut self, _val: u8) {
        unimplemented!();
    }

    fn peek_b(&self) -> u8 {
        unimplemented!();
    }
    fn read_b(&mut self) -> u8 {
        unimplemented!();
    }
    fn write_b(&mut self, _val: u8) {
        unimplemented!();
    }
}
