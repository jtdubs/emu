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
        &mut self.bus.pers.dsp
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
        self.bus.pers.dsp.cycle();
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

pub struct Peripherals {
    pub dsp: HD44780U,
    pub con: SNESController,
    pub a_cache: u8,
    pub b_cache: u8,
}

const RS: u8 = 0x20;
const RW: u8 = 0x40;
const E: u8 = 0x80;

impl Peripherals {
    pub fn new() -> Peripherals {
        Peripherals {
            dsp: HD44780U::new(),
            con: SNESController::new(),
            a_cache: 0,
            b_cache: 0,
        }
    }

    fn get_control(&self, a: u8) -> (RegisterSelector, bool, bool) {
        (
            if a & RS == RS {
                RegisterSelector::Data
            } else {
                RegisterSelector::Instruction
            },
            a & RW == RW,
            a & E == E,
        )
    }
}

impl PortArbiter for Peripherals {
    fn port(&mut self, op: PortOperation) -> u8 {
        match op {
            PortOperation::Read(port) => {
                match port {
                    Port::A => {
                        (0u8 & 0xE0) | (self.con.read() & 0x07)
                    }
                    Port::B => {
                        self.dsp.read(self.get_control(self.a_cache).0)
                    }
                }
            }
            PortOperation::Peek(port) => {
                match port {
                    Port::A => {
                        (0u8 & 0xE0) | (self.con.peek() & 0x07)
                    }
                    Port::B => {
                        self.dsp.peek(self.get_control(self.a_cache).0)
                    }
                }
            }
            PortOperation::Write(port, val) => {
                match port {
                    Port::A => {
                        let dsp_val = val & 0xE0;
                        match (self.get_control(self.a_cache).2, self.get_control(dsp_val).2) {
                            (true, false) => {
                                self.dsp.write(self.get_control(dsp_val).0, self.b_cache);
                            }
                            _ => {}
                        }
                        self.a_cache = dsp_val;
                        self.con.write(val & 0x07);
                    }
                    Port::B => {
                        self.b_cache = val;
                    }
                }
                val
            }
        }
    }
}