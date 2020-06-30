use crate::components::*;

pub struct System {
    pub cpu: W65C02S,
    pub bus: BusMembers,
}

impl System {
    pub fn new(rom_path: &str) -> System {
        System {
            cpu: W65C02S::new(),
            bus: BusMembers::new(rom_path),
        }
    }

    /*
    pub fn get_pc(&self) -> u16 {
        self.cpu.pc
    }
    
    pub fn get_current_instruction(&self) -> cpu::Instruction {
        self.cpu.ir.0
    }

    pub fn peek(&mut self, addr : u16) -> u8 {
        self.bus.bus(BusOperation::Peek(addr))
    }
    */

    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.bus);
        if self.bus.per.cycle() {
            self.cpu.interrupt();
        }
    }
}

pub struct BusMembers {
    pub per: W65C22,
    pub ram: RAM,
    pub rom: ROM,
}

impl BusMembers {
    pub fn new(rom_path : &str) -> BusMembers {
        BusMembers {
            rom: ROM::load(rom_path),
            ram: RAM::new(0x4000),
            per: W65C22::new(),
        }
    }
}

impl BusArbiter for BusMembers {
    fn bus(&mut self, op : BusOperation) -> u8 {
        match op {
            BusOperation::Read(addr) => {
                if addr & 0x8000 == 0x8000 {
                    self.rom.read(addr & !0x8000)
                } else if addr & 0xC000 == 0x0000 {
                    self.ram.read(addr & !0xC000)
                } else if addr & 0xFFF0 == 0x6000 {
                    self.per.read(addr & !0xFFF0)
                } else {
                    panic!("read at unmapped address: {:02x}", addr);
                }
            }
            BusOperation::Write(addr, val) => {
                if addr & 0x8000 == 0x8000 {
                    self.rom.write(addr & !0x8000, val);
                } else if addr & 0xC000 == 0x0000 {
                    self.ram.write(addr & !0xC000, val);
                } else if addr & 0xFFF0 == 0x6000 {
                    self.per.write(addr & !0xFFF0, val);
                } else {
                    panic!("write at unmapped address: {:02x}", addr);
                }
                val
            }
            BusOperation::Peek(addr) => {
                if addr & 0x8000 == 0x8000 {
                    self.rom.peek(addr & !0x8000)
                } else if addr & 0xC000 == 0x0000 {
                    self.ram.peek(addr & !0xC000)
                } else if addr & 0xFFF0 == 0x6000 {
                    self.per.peek(addr & !0xFFF0)
                } else {
                    panic!("peek at unmapped address: {:02x}", addr);
                }
            }
        }
    }
}