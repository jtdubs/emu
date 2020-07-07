use log::debug;
use num_enum::*;
use std::cell::Cell;
use std::fmt;
use std::convert::TryFrom;

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[allow(dead_code, non_camel_case_types)]
#[repr(u16)]
enum RegisterSelector {
    PB = 0x00,     // Port B Data
    PA_HS = 0x01,  // Port A Data (w/ Handshake)
    DDRB = 0x02,   // Port B Direction
    DDRA = 0x03,   // Port A Direction
    T1C_L = 0x04,  // Timer 1 Counter (Low)
    T1C_H = 0x05,  // Timer 1 Counter (High)
    T1L_L = 0x06,  // Timer 1 Latch (Low)
    T1L_H = 0x07,  // Timer 1 Latch (High)
    T2C_L = 0x08,  // Timer 2 Counter (Low)
    T2C_H = 0x09,  // Timer 2 Counter (High)
    SR = 0x0A,     // Shift Register
    ACR = 0x0B,    // Auxiliiary Control
    PCR = 0x0C,    // Peripheral Control
    IFR = 0x0D,    // Interrupt Flags
    IER = 0x0E,    // Interrupt Enable
    PA = 0x0F,     // Port A Data (w/o Handshake)
}

#[derive(Debug)]
#[allow(dead_code)]
enum Interrupts {
    CA2 = 0x01,
    CA1 = 0x02,
    SR = 0x04,
    CB2 = 0x08,
    CB1 = 0x10,
    T2 = 0x20,
    T1 = 0x40,
    IRQ = 0x80,
}

#[derive(Debug)]
#[allow(dead_code)]
enum TimerMode {
    OneShot = 0,
    FreeRunning = 1,
}

#[derive(Debug)]
#[allow(dead_code)]
enum TimerOutputMode {
    None = 0,
    PortB = 1,
}

#[derive(Debug)]
#[allow(dead_code)]
enum SRMode {
    Disabled = 0,
    InputTimer2 = 1,
    InputSystemClock = 2,
    InputExternalClock = 3,
    FreeRunning = 4,
    OutputTimer2 = 5,
    OutputSystemClock = 6,
    OutputExternalClock = 7,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[allow(dead_code)]
#[repr(u8)]
enum CAMode {
    InputModeFalling = 0,
    InterruptModeFalling = 1,
    InputModeRising = 2,
    InterruptModeRising = 3,
    HandshakeMode = 4,
    PulseMoutputMode = 5,
    Low = 6,
    High = 7,
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[allow(dead_code)]
#[repr(u8)]
enum CAEdge {
    Negative = 0,
    Positive = 1,
}

pub trait Ports {
    fn peek_a(&self) -> u8;
    fn read_a(&mut self) -> u8;
    fn write_a(&mut self, val: u8);

    fn peek_b(&self) -> u8;
    fn read_b(&mut self) -> u8;
    fn write_b(&mut self, val: u8);
}

pub struct MOS6522<PortsType: Ports> {
    // interrupt control
    pub ifr: Cell<u8>,  // interrupt flag register
    pub ier: u8,        // interrupt enable register

    // function control
    pub acr: u8,    // auxilliary control register
    pub pcr: u8,    // peripheral control register

    // port a
    pub ira: u8,    // port a input register
    pub ora: u8,    // port a output register
    pub ddra: u8,   // port a direction register

    // port b
    pub irb: u8,    // port b input register
    pub orb: u8,    // port b output register
    pub ddrb: u8,   // port b direction register

    // timer 1
    pub t1c: u16,   // timer 1 counter
    pub t1l: u16,   // timer 1 latch

    // timer 2
    pub t2c: u16,   // timer 2 counter
    pub t2l: u16,   // timer 2 latch

    // serial data shift register
    pub sr: u8,     // shift register

    // handshake control
    pub ca1: bool,
    pub ca2: bool,
    pub cb1: bool,
    pub cb2: bool,

    // interface for interrogating ports in input mode
    pub ports: PortsType,
}

impl<PortsType: Ports> fmt::Debug for MOS6522<PortsType> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MOS6522")
            .finish()
    }
}

impl<PortsType: Ports> MOS6522<PortsType> {
    pub fn new(ports: PortsType) -> MOS6522<PortsType> {
        MOS6522 {
            ifr: Cell::new(0),
            ier: 0,
            acr: 0,
            pcr: 0,
            ira: 0,
            ora: 0,
            ddra: 0,
            irb: 0,
            orb: 0,
            ddrb: 0,
            t1c: 0,
            t1l: 0,
            t2c: 0,
            t2l: 0,
            sr: 0,
            ca1: false,
            ca2: false,
            cb1: false,
            cb2: false,
            ports: ports,
        }
    }

    fn set_interrupt(&self, i: Interrupts) {
        debug!("Set interrupt: {:?}", i);
        let mut ifr = self.ifr.get();
        ifr |= i as u8;
        ifr |= 0x80;
        self.ifr.replace(ifr);
    }

    fn clear_interrupt(&self, i: Interrupts) {
        debug!("Clear interrupt: {:?}", i);
        let mut ifr = self.ifr.get();
        ifr &= !(i as u8);
        if ifr == 0x80 {
            ifr = 0;
        }
        self.ifr.replace(ifr);
    }

    pub fn peek(&self, addr: u16) -> u8 {
        match RegisterSelector::try_from(addr).unwrap() {
            RegisterSelector::PB => {
                (self.orb & self.ddrb) | (self.ports.peek_b() & !self.ddrb)
            }
            RegisterSelector::PA_HS => {
                unimplemented!();
            }
            RegisterSelector::DDRB => {
                self.ddrb
            }
            RegisterSelector::DDRA => {
                self.ddra
            }
            RegisterSelector::T1C_L => {
                (self.t1c & 0x00ff) as u8
            }
            RegisterSelector::T1C_H => {
                (self.t1c >> 8) as u8
            }
            RegisterSelector::T1L_L => {
                (self.t1l & 0x00ff) as u8
            }
            RegisterSelector::T1L_H => {
                (self.t1l >> 8) as u8
            }
            RegisterSelector::T2C_L => {
                unimplemented!("MOS6522 - Read T2C_L");
            }
            RegisterSelector::T2C_H => {
                unimplemented!("MOS6522 - Read T2C_H");
            }
            RegisterSelector::SR => {
                unimplemented!("MOS6522 - Read SR");
            }
            RegisterSelector::ACR => {
                unimplemented!("MOS6522 - Read ACR");
            }
            RegisterSelector::PCR => {
                unimplemented!("MOS6522 - Read PCR");
            }
            RegisterSelector::IFR => {
                self.ifr.get()
            }
            RegisterSelector::IER => {
                self.ier
            }
            RegisterSelector::PA => {
                (self.ora & self.ddra) | (self.ports.peek_a() & !self.ddra)
            }
        }
    }

    pub fn set_cb1(&mut self, val: bool) {
        // if port b latching enabled
        if self.acr & 0x02 == 0x02 {
            // check cb1 input mode and latch if condition met
            match CAEdge::try_from(self.pcr >> 4 & 1).unwrap() {
                CAEdge::Positive => {
                    if !self.cb1 && val {
                        self.irb = self.ports.read_b();
                    }
                }
                CAEdge::Negative => {
                    if self.cb1 && !val {
                        self.irb = self.ports.read_b();
                    }
                }
            }
        }

        self.cb1 = val;
    }

    pub fn set_cb2(&mut self, val: bool) {
        // if port b latching enabled
        if self.acr & 0x02 == 0x02 {
            // check cb2 input mode and latch if condition met
            match CAMode::try_from(self.pcr >> 5).unwrap() {
                CAMode::InputModeFalling | CAMode::InterruptModeFalling => {
                    if self.cb2 && !val {
                        self.irb = self.ports.read_b();
                    }
                }
                CAMode::InputModeRising | CAMode::InterruptModeRising => {
                    if !self.cb2 && val {
                        self.irb = self.ports.read_b();
                    }
                }
                _ => {}
            }
        }

        self.cb2 = val;
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        let data = match RegisterSelector::try_from(addr).unwrap() {
            RegisterSelector::PB => {
                // if port b latching disabled, then 
                if self.acr & 0x02 != 0x02 {
                    self.irb = self.ports.read_b();
                }
                self.irb
            }
            RegisterSelector::PA_HS => {
                unimplemented!();
            }
            RegisterSelector::DDRB => {
                self.ddrb
            }
            RegisterSelector::DDRA => {
                self.ddra
            }
            RegisterSelector::T1C_L => {
                self.clear_interrupt(Interrupts::T1);
                (self.t1c & 0x00ff) as u8
            }
            RegisterSelector::T1C_H => {
                (self.t1c >> 8) as u8
            }
            RegisterSelector::T1L_L => {
                self.clear_interrupt(Interrupts::T1);
                (self.t1l & 0x00ff) as u8
            }
            RegisterSelector::T1L_H => {
                (self.t1l >> 8) as u8
            }
            RegisterSelector::T2C_L => {
                unimplemented!("MOS6522 - Read T2C_L");
            }
            RegisterSelector::T2C_H => {
                unimplemented!("MOS6522 - Read T2C_H");
            }
            RegisterSelector::SR => {
                unimplemented!("MOS6522 - Read SR");
            }
            RegisterSelector::ACR => {
                unimplemented!("MOS6522 - Read ACR");
            }
            RegisterSelector::PCR => {
                unimplemented!("MOS6522 - Read PCR");
            }
            RegisterSelector::IFR => {
                self.ifr.get()
            }
            RegisterSelector::IER => {
                self.ier
            }
            RegisterSelector::PA => {
                (self.ora & self.ddra) | (self.ports.peek_a() & !self.ddra)
            }
        };
        debug!("R @ {:04x} = {:02x}", addr, data);
        data
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        debug!("W @ {:04x} = {:02x}", addr, data);
        match RegisterSelector::try_from(addr).unwrap() {
            RegisterSelector::PB => {
                self.orb = data & self.ddrb;
                self.ports.write_b(self.orb);
            }
            RegisterSelector::PA_HS => {
                unimplemented!("MOS6522 - Access to ORA w/ handshake");
            }
            RegisterSelector::DDRB => {
                self.ddrb = data;
            }
            RegisterSelector::DDRA => {
                self.ddra = data;
            }
            RegisterSelector::T1C_L => {
                self.t1l = (self.t1l & 0xff00) | (data as u16);
            }
            RegisterSelector::T1C_H => {
                self.t1l = (self.t1l & 0x00ff) | ((data as u16) << 8);
                self.t1c = self.t1l;
                self.clear_interrupt(Interrupts::T1);
            }
            RegisterSelector::T1L_L => {
                self.t1l = (self.t1l & 0xff00) | (data as u16);
            }
            RegisterSelector::T1L_H => {
                self.t1l = (self.t1l & 0x00ff) | ((data as u16) << 8);
                self.clear_interrupt(Interrupts::T1);
            }
            RegisterSelector::T2C_L => {
                unimplemented!("MOS6522 - Write T2C_H");
            }
            RegisterSelector::T2C_H => {
                unimplemented!("MOS6522 - Write T2C_H");
            }
            RegisterSelector::SR => {
                unimplemented!("MOS6522 - Write SR");
            }
            RegisterSelector::ACR => {
                self.acr = data;
            }
            RegisterSelector::PCR => {
                unimplemented!("MOS6522 - Write PCR");
            }
            RegisterSelector::IFR => {
                *self.ifr.get_mut() &= !data;
            }
            RegisterSelector::IER => {
                self.ier = data;
            }
            RegisterSelector::PA => {
                self.ora = data & self.ddra;
                self.ports.write_a(self.ora);
            }
        };
    }

    pub fn cycle(&mut self) -> bool {
        if self.t1c > 0 {
            self.t1c -= 1;
        } else {
            self.set_interrupt(Interrupts::T1);
            match self.acr >> 6 {
                0 => {
                    // single-shot mode
                    // nothing to do
                }
                1 => {
                    // free-run mode
                    // reset timer to latched value
                    self.t1c = self.t1l;
                }
                2 => {
                    unimplemented!("timer1 - one shot pb7 mode");
                }
                3 => {
                    unimplemented!("timer1 - square wave mode");
                }
                _ => {
                    panic!("impossible value for self.acr");
                }
            }

            // TODO: if T1 is set in IER, raise the interrupt
        }

        (self.ifr.get() & self.ier) != 0
    }

}
