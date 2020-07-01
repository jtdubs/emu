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
        self.bus.peek(addr)
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
    const ROM_SELECTOR: (u16, u16) = (0x8000, 0x8000);
    const RAM_SELECTOR: (u16, u16) = (0xC000, 0x0000);
    const PER_SELECTOR: (u16, u16) = (0xFFF0, 0x6000);

    pub fn new(rom_path: &str) -> BusMembers {
        BusMembers {
            rom: ROM::load(rom_path),
            ram: RAM::new(0x4000),
            per: W65C22::new(),
            pers: Peripherals::new(),
        }
    }
}

impl Bus for BusMembers {
    fn peek(&self, addr: u16) -> u8 {
        if addr & BusMembers::ROM_SELECTOR.0 == BusMembers::ROM_SELECTOR.1 {
            self.rom.peek(addr & !BusMembers::ROM_SELECTOR.0)
        } else if addr & BusMembers::RAM_SELECTOR.0 == BusMembers::RAM_SELECTOR.1 {
            self.ram.peek(addr & !BusMembers::RAM_SELECTOR.0)
        } else if addr & BusMembers::PER_SELECTOR.0 == BusMembers::PER_SELECTOR.1 {
            self.per.peek(addr & !BusMembers::PER_SELECTOR.0, &self.pers)
        } else {
            panic!("peek at unmapped address: {:02x}", addr);
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        if addr & BusMembers::ROM_SELECTOR.0 == BusMembers::ROM_SELECTOR.1 {
            self.rom.read(addr & !BusMembers::ROM_SELECTOR.0)
        } else if addr & BusMembers::RAM_SELECTOR.0 == BusMembers::RAM_SELECTOR.1 {
            self.ram.read(addr & !BusMembers::RAM_SELECTOR.0)
        } else if addr & BusMembers::PER_SELECTOR.0 == BusMembers::PER_SELECTOR.1 {
            self.per.read(addr & !BusMembers::PER_SELECTOR.0, &mut self.pers)
        } else {
            panic!("read at unmapped address: {:02x}", addr);
        }
    }

    fn write(&mut self, addr: u16, val: u8) {
        if addr & BusMembers::ROM_SELECTOR.0 == BusMembers::ROM_SELECTOR.1 {
            self.rom.write(addr & !BusMembers::ROM_SELECTOR.1, val);
        } else if addr & BusMembers::RAM_SELECTOR.0 == BusMembers::RAM_SELECTOR.1 {
            self.ram.write(addr & !BusMembers::RAM_SELECTOR.1, val);
        } else if addr & BusMembers::PER_SELECTOR.0 == BusMembers::PER_SELECTOR.1 {
            self.per.write(addr & !BusMembers::PER_SELECTOR.0, val, &mut self.pers);
        } else {
            panic!("write at unmapped address: {:02x}", addr);
        }
    }
}

pub struct Peripherals {
    pub dsp: HD44780U,
    pub con: SNESController,
    pub a_cache: u8,
    pub b_cache: u8,
}

impl Peripherals {
    pub fn new() -> Peripherals {
        Peripherals {
            dsp: HD44780U::new(),
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

        (
            self.a_cache & LATCH == LATCH,
            self.a_cache & CLK == CLK,
        )
    }
}

impl Ports for Peripherals {
    fn peek(&self, port: Port) -> u8 {
        match port {
            Port::A => {
                self.con.peek() & 0x07
            }
            Port::B => {
                let (rs, rw, e) = self.get_dsp_pins();
                self.dsp.peek(rs, rw, e)
            }
        }
    }

    fn read(&mut self, port: Port) -> u8 {
        match port {
            Port::A => {
                self.con.read() & 0x07
            }
            Port::B => {
                let (rs, rw, e) = self.get_dsp_pins();
                self.dsp.read(rs, rw, e)
            }
        }
    }

    fn write(&mut self, port: Port, val: u8) {
        match port {
            Port::A => {
                self.a_cache = val;
            }
            Port::B => {
                self.b_cache = val;
            }
        }

        let (rs, rw, e) = self.get_dsp_pins();
        self.dsp.write(rs, rw, e, self.b_cache);

        
        let (latch, clk) = self.get_con_pins();
        self.con.write(latch, clk);
    }
}