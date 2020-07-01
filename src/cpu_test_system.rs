use crate::components::*;
use crate::system::System;

pub struct CPUTestSystem {
    cpu: W65C02S<SystemBus>,
}

impl CPUTestSystem {
    pub fn new(ram_path: &str, entry_address: u16) -> CPUTestSystem {
        CPUTestSystem {
            cpu: W65C02S::new(SystemBus::new(ram_path, entry_address)),
        }
    }
}

impl System for CPUTestSystem {
    type BusType = SystemBus;
    type PortsType = NullPorts;

    fn is_halted(&self) -> bool {
        self.cpu.is_halted()
    }

    fn get_cpu(&self) -> &W65C02S<Self::BusType> {
        &self.cpu
    }

    fn get_display(&mut self) -> Option<&mut HD44780U> {
        None
    }

    fn get_ram(&self) -> &RAM {
        &self.cpu.bus.ram
    }

    fn get_controller(&mut self) -> Option<&mut SNESController> {
        None
    }

    fn get_peripheral_controller(&self) -> Option<&W65C22<Self::PortsType>> {
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
    pub ram: RAM
}

impl SystemBus {
    pub fn new(ram_path: &str, entry_address: u16) -> SystemBus {
        let mut ram = RAM::load(ram_path);
        ram.mem[0xfffc] = (entry_address & 0xff) as u8;
        ram.mem[0xfffd] = ((entry_address >> 8) & 0xff) as u8;
        SystemBus {
            ram: ram,
        }
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

pub struct NullPorts {
}

impl Ports for NullPorts {
    fn peek(&self, _port: Port) -> u8 { unimplemented!(); }
    fn read(&mut self, _port: Port) -> u8 { unimplemented!();}
    fn write(&mut self, _port: Port, _val: u8) {unimplemented!();}
}