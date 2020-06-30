use crate::components::*;

pub struct System {
    cpu: W65C02S,
    bus: BusMembers,
}

impl System {
    pub fn new(rom_path: &str) -> System {
        System {
            cpu: W65C02S::new(),
            bus: BusMembers::new(rom_path),
        }
    }

    pub fn is_halted(&self) -> bool {
        self.cpu.is_halted()
    }

    pub fn get_cpu(&self) -> &W65C02S {
        &self.cpu
    }

    pub fn get_display(&mut self) -> &mut HD44780U {
        &mut self.bus.pers.ada.dsp
    }

    pub fn get_ram(&self) -> &RAM {
        &self.bus.ram
    }

    pub fn get_controller(&mut self) -> &mut SNESController {
        &mut self.bus.pers.con
    }

    pub fn get_peripheral_controller(&self) -> &W65C22 {
        &self.bus.per
    }

    pub fn peek(&mut self, addr: u16) -> u8 {
        self.bus.bus(BusOperation::Peek(addr))
    }

    pub fn cycle(&mut self) {
        self.cpu.cycle(&mut self.bus);
        self.cpu.set_interrupt(self.bus.per.cycle());
        self.bus.pers.ada.cycle();
    }
}

pub struct Peripherals {
    pub ada: HD44780UAdapter,
    pub con: SNESController,
}

impl Peripherals {
    pub fn new() -> Peripherals {
        Peripherals {
            ada: HD44780UAdapter::new(),
            con: SNESController::new(),
        }
    }
}

impl PortArbiter for Peripherals {
    fn port(&mut self, op: PortOperation) -> u8 {
        match op {
            PortOperation::Read(port) => {
                match port {
                    Port::A => {
                        (self.ada.peek(Port::A) & 0xE0) | (self.con.peek(Port::A) & 0x07)
                    }
                    Port::B => {
                        self.ada.peek(Port::B)
                    }
                }
            }
            PortOperation::Peek(port) => {
                match port {
                    Port::A => {
                        (self.ada.read(Port::A) & 0xE0) | (self.con.read(Port::A) & 0x07)
                    }
                    Port::B => {
                        self.ada.read(Port::B)
                    }
                }
            }
            PortOperation::Write(port, val) => {
                match port {
                    Port::A => {
                        self.ada.write(Port::A, val & 0xE0);
                        self.con.write(Port::A, val & 0x07);
                    }
                    Port::B => {
                        self.ada.write(Port::B, val);
                    }
                }
                val
            }
        }
    }
}

pub struct BusMembers {
    pub per: W65C22,
    pub ram: RAM,
    pub rom: ROM,
    pub pers: Peripherals,
}

impl BusMembers {
    pub fn new(rom_path: &str) -> BusMembers {
        BusMembers {
            rom: ROM::load(rom_path),
            ram: RAM::new(0x4000),
            per: W65C22::new(),
            pers: Peripherals::new(),
        }
    }
}

impl BusArbiter for BusMembers {
    fn bus(&mut self, op: BusOperation) -> u8 {
        match op {
            BusOperation::Read(addr) => {
                if addr & 0x8000 == 0x8000 {
                    self.rom.read(addr & !0x8000)
                } else if addr & 0xC000 == 0x0000 {
                    self.ram.read(addr & !0xC000)
                } else if addr & 0xFFF0 == 0x6000 {
                    self.per.read(addr & !0xFFF0, &mut self.pers)
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
                    self.per.write(addr & !0xFFF0, val, &mut self.pers);
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
                    self.per.peek(addr & !0xFFF0, &mut self.pers)
                } else {
                    panic!("peek at unmapped address: {:02x}", addr);
                }
            }
        }
    }
}
