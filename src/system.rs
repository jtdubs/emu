use crate::components::*;

pub struct System {
    cpu: W65C02S<SystemBus>,
}

impl System {
    pub fn new(rom_path: &str) -> System {
        System {
            cpu: W65C02S::new(SystemBus::new(rom_path)),
        }
    }

    pub fn is_halted(&self) -> bool {
        self.cpu.is_halted()
    }

    pub fn get_cpu(&self) -> &W65C02S<SystemBus> {
        &self.cpu
    }

    pub fn get_display(&mut self) -> &mut HD44780U {
        &mut self.cpu.bus.per.ports.dsp
    }

    pub fn get_ram(&self) -> &RAM {
        &self.cpu.bus.ram
    }

    pub fn get_controller(&mut self) -> &mut SNESController {
        &mut self.cpu.bus.per.ports.con
    }

    pub fn get_peripheral_controller(&self) -> &W65C22<Peripherals> {
        &self.cpu.bus.per
    }

    pub fn peek(&mut self, addr: u16) -> u8 {
        self.cpu.bus.peek(addr)
    }

    pub fn cycle(&mut self) {
        self.cpu.cycle();
        let per_int = self.cpu.bus.per.cycle();
        self.cpu.set_interrupt(per_int);
        self.cpu.bus.per.ports.dsp.cycle();
    }
}

pub struct SystemBus {
    pub per: W65C22<Peripherals>,
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
            per: W65C22::new(Peripherals::new()),
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