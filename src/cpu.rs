use std::fmt;
use log::debug;

#[derive(Debug)]
pub enum CPUState {
    Init(u8),
    Run,
    Halt
}

#[derive(Debug)]
pub enum AddressMode {
    Absolute,                       // a
    AbsoluteIndexedIndirect,        // (a,x)
    AbsoluteIndexedWithX,           // a,x
    AbsoluteIndexedWithY,           // a,y
    AbsoluteIndirect,               // (a)
    Accumulator,                    // A
    ImmediateAddressing,            // #
    Implied,                        // i
    ProgramCounterRelative,         // r
    Stack,                          // s
    ZeroPage,                       // zp
    ZeroPageIndexedIndirect,        // (zp,x)
    ZeroPageIndexedWithX,           // zp,x
    ZeroPageIndexedWithY,           // zp,y
    ZeroPageIndirect,               // (zp)
    ZeroPageIndirectIndexedWithY    // (zp),y
}

#[derive(Debug)]
pub enum Instruction {
    ADC,
    AND,
    ASL,
    BBR(u8),
    BBS(u8),
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRA,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PHX,
    PHY,
    PLA,
    PLP,
    PLX,
    PLY,
    RMB(u8),
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    SMB(u8),
    STA,
    STP,
    STX,
    STY,
    STZ,
    TAX,
    TAY,
    TRB,
    TSB,
    TSX,
    TXA,
    TXS,
    TYA,
    WAI
}

pub type Opcode = (Instruction, AddressMode);

fn decode(val : u8) -> Option<Opcode> {
    match val {
        0x6D => Some((Instruction::ADC, AddressMode::Absolute)),
        0x7D => Some((Instruction::ADC, AddressMode::AbsoluteIndexedWithX)),
        0x79 => Some((Instruction::ADC, AddressMode::AbsoluteIndexedWithY)),
        0x69 => Some((Instruction::ADC, AddressMode::ImmediateAddressing)),
        0x65 => Some((Instruction::ADC, AddressMode::ZeroPage)),
        0x61 => Some((Instruction::ADC, AddressMode::ZeroPageIndexedIndirect)),
        0x75 => Some((Instruction::ADC, AddressMode::ZeroPageIndexedWithX)),
        0x72 => Some((Instruction::ADC, AddressMode::ZeroPageIndirect)),
        0x71 => Some((Instruction::ADC, AddressMode::ZeroPageIndirectIndexedWithY)),

        0x2D => Some((Instruction::AND, AddressMode::Absolute)),
        0x3D => Some((Instruction::AND, AddressMode::AbsoluteIndexedWithX)),
        0x39 => Some((Instruction::AND, AddressMode::AbsoluteIndexedWithY)),
        0x29 => Some((Instruction::AND, AddressMode::ImmediateAddressing)),
        0x25 => Some((Instruction::AND, AddressMode::ZeroPage)),
        0x21 => Some((Instruction::AND, AddressMode::ZeroPageIndexedIndirect)),
        0x35 => Some((Instruction::AND, AddressMode::ZeroPageIndexedWithX)),
        0x32 => Some((Instruction::AND, AddressMode::ZeroPageIndirect)),
        0x31 => Some((Instruction::AND, AddressMode::ZeroPageIndirectIndexedWithY)),

        0x0E => Some((Instruction::ASL, AddressMode::Absolute)),
        0x1E => Some((Instruction::ASL, AddressMode::AbsoluteIndexedWithX)),
        0x0A => Some((Instruction::ASL, AddressMode::Accumulator)),
        0x06 => Some((Instruction::ASL, AddressMode::ZeroPage)),
        0x16 => Some((Instruction::ASL, AddressMode::ZeroPageIndexedWithX)),

        0x0F => Some((Instruction::BBR(0), AddressMode::ProgramCounterRelative)),
        0x1F => Some((Instruction::BBR(1), AddressMode::ProgramCounterRelative)),
        0x2F => Some((Instruction::BBR(2), AddressMode::ProgramCounterRelative)),
        0x3F => Some((Instruction::BBR(3), AddressMode::ProgramCounterRelative)),
        0x4F => Some((Instruction::BBR(4), AddressMode::ProgramCounterRelative)),
        0x5F => Some((Instruction::BBR(5), AddressMode::ProgramCounterRelative)),
        0x6F => Some((Instruction::BBR(6), AddressMode::ProgramCounterRelative)),
        0x7F => Some((Instruction::BBR(7), AddressMode::ProgramCounterRelative)),

        0x8F => Some((Instruction::BBS(0), AddressMode::ProgramCounterRelative)),
        0x9F => Some((Instruction::BBS(1), AddressMode::ProgramCounterRelative)),
        0xAF => Some((Instruction::BBS(2), AddressMode::ProgramCounterRelative)),
        0xBF => Some((Instruction::BBS(3), AddressMode::ProgramCounterRelative)),
        0xCF => Some((Instruction::BBS(4), AddressMode::ProgramCounterRelative)),
        0xDF => Some((Instruction::BBS(5), AddressMode::ProgramCounterRelative)),
        0xEF => Some((Instruction::BBS(6), AddressMode::ProgramCounterRelative)),
        0xFF => Some((Instruction::BBS(7), AddressMode::ProgramCounterRelative)),

        0x90 => Some((Instruction::BCC, AddressMode::ProgramCounterRelative)),
        0xB0 => Some((Instruction::BCS, AddressMode::ProgramCounterRelative)),
        0xF0 => Some((Instruction::BEQ, AddressMode::ProgramCounterRelative)),

        0x2C => Some((Instruction::BIT, AddressMode::Absolute)),
        0x3C => Some((Instruction::BIT, AddressMode::AbsoluteIndexedWithX)),
        0x89 => Some((Instruction::BIT, AddressMode::ImmediateAddressing)),
        0x24 => Some((Instruction::BIT, AddressMode::ZeroPage)),
        0x34 => Some((Instruction::BIT, AddressMode::ZeroPageIndexedWithX)),


        0x30 => Some((Instruction::BMI, AddressMode::ProgramCounterRelative)),
        0xD0 => Some((Instruction::BNE, AddressMode::ProgramCounterRelative)),
        0x10 => Some((Instruction::BPL, AddressMode::ProgramCounterRelative)),
        0x80 => Some((Instruction::BRA, AddressMode::ProgramCounterRelative)),

        0x00 => Some((Instruction::BRK, AddressMode::Stack)),

        0x50 => Some((Instruction::BVC, AddressMode::ProgramCounterRelative)),
        0x70 => Some((Instruction::BVS, AddressMode::ProgramCounterRelative)),

        0x18 => Some((Instruction::CLC, AddressMode::Implied)),
        0xD8 => Some((Instruction::CLD, AddressMode::Implied)),
        0x58 => Some((Instruction::CLI, AddressMode::Implied)),
        0xB8 => Some((Instruction::CLV, AddressMode::Implied)),

        0xCD => Some((Instruction::CMP, AddressMode::Absolute)),
        0xDD => Some((Instruction::CMP, AddressMode::AbsoluteIndexedWithX)),
        0xD9 => Some((Instruction::CMP, AddressMode::AbsoluteIndexedWithY)),
        0xC9 => Some((Instruction::CMP, AddressMode::ImmediateAddressing)),
        0xC5 => Some((Instruction::CMP, AddressMode::ZeroPage)),
        0xC1 => Some((Instruction::CMP, AddressMode::ZeroPageIndexedIndirect)),
        0xD5 => Some((Instruction::CMP, AddressMode::ZeroPageIndexedWithX)),
        0xD2 => Some((Instruction::CMP, AddressMode::ZeroPageIndirect)),
        0xD1 => Some((Instruction::CMP, AddressMode::ZeroPageIndirectIndexedWithY)),

        0xEC => Some((Instruction::CPX, AddressMode::Absolute)),
        0xE0 => Some((Instruction::CPX, AddressMode::ImmediateAddressing)),
        0xE4 => Some((Instruction::CPX, AddressMode::ZeroPage)),

        0xCC => Some((Instruction::CPY, AddressMode::Absolute)),
        0xC0 => Some((Instruction::CPY, AddressMode::ImmediateAddressing)),
        0xC4 => Some((Instruction::CPY, AddressMode::ZeroPage)),

        0xCE => Some((Instruction::DEC, AddressMode::Absolute)),
        0xDE => Some((Instruction::DEC, AddressMode::AbsoluteIndexedWithX)),
        0x3A => Some((Instruction::DEC, AddressMode::Accumulator)),
        0xC6 => Some((Instruction::DEC, AddressMode::ZeroPage)),
        0xD6 => Some((Instruction::DEC, AddressMode::ZeroPageIndexedWithX)),

        0xCA => Some((Instruction::DEX, AddressMode::Implied)),
        0x88 => Some((Instruction::DEY, AddressMode::Implied)),

        0x4D => Some((Instruction::EOR, AddressMode::Absolute)),
        0x5D => Some((Instruction::EOR, AddressMode::AbsoluteIndexedWithX)),
        0x59 => Some((Instruction::EOR, AddressMode::AbsoluteIndexedWithY)),
        0x49 => Some((Instruction::EOR, AddressMode::ImmediateAddressing)),
        0x45 => Some((Instruction::EOR, AddressMode::ZeroPage)),
        0x41 => Some((Instruction::EOR, AddressMode::ZeroPageIndexedIndirect)),
        0x55 => Some((Instruction::EOR, AddressMode::ZeroPageIndexedWithX)),
        0x52 => Some((Instruction::EOR, AddressMode::ZeroPageIndirect)),
        0x51 => Some((Instruction::EOR, AddressMode::ZeroPageIndirectIndexedWithY)),

        0xEE => Some((Instruction::INC, AddressMode::Absolute)),
        0xFE => Some((Instruction::INC, AddressMode::AbsoluteIndexedWithX)),
        0x1A => Some((Instruction::INC, AddressMode::Accumulator)),
        0xE6 => Some((Instruction::INC, AddressMode::ZeroPage)),
        0xF6 => Some((Instruction::INC, AddressMode::ZeroPageIndexedWithX)),

        0xE8 => Some((Instruction::INX, AddressMode::Implied)),
        0xC8 => Some((Instruction::INY, AddressMode::Implied)),

        0x4C => Some((Instruction::JMP, AddressMode::Absolute)),
        0x7C => Some((Instruction::JMP, AddressMode::AbsoluteIndexedIndirect)),
        0x6C => Some((Instruction::JMP, AddressMode::AbsoluteIndirect)),

        0x20 => Some((Instruction::JSR, AddressMode::Absolute)),

        0xAD => Some((Instruction::LDA, AddressMode::Absolute)),
        0xBD => Some((Instruction::LDA, AddressMode::AbsoluteIndexedWithX)),
        0xB9 => Some((Instruction::LDA, AddressMode::AbsoluteIndexedWithY)),
        0xA9 => Some((Instruction::LDA, AddressMode::ImmediateAddressing)),
        0xA5 => Some((Instruction::LDA, AddressMode::ZeroPage)),
        0xA1 => Some((Instruction::LDA, AddressMode::ZeroPageIndexedIndirect)),
        0xB5 => Some((Instruction::LDA, AddressMode::ZeroPageIndexedWithX)),
        0xB2 => Some((Instruction::LDA, AddressMode::ZeroPageIndirect)),
        0xB1 => Some((Instruction::LDA, AddressMode::ZeroPageIndirectIndexedWithY)),

        0xAE => Some((Instruction::LDX, AddressMode::Absolute)),
        0xBE => Some((Instruction::LDX, AddressMode::AbsoluteIndexedWithY)),
        0xA2 => Some((Instruction::LDX, AddressMode::ImmediateAddressing)),
        0xA6 => Some((Instruction::LDX, AddressMode::ZeroPage)),
        0xB6 => Some((Instruction::LDX, AddressMode::ZeroPageIndexedWithY)),

        0xAC => Some((Instruction::LDY, AddressMode::Absolute)),
        0xBC => Some((Instruction::LDY, AddressMode::AbsoluteIndexedWithX)),
        0xA0 => Some((Instruction::LDY, AddressMode::ImmediateAddressing)),
        0xA4 => Some((Instruction::LDY, AddressMode::ZeroPage)),
        0xB4 => Some((Instruction::LDY, AddressMode::ZeroPageIndexedWithX)),

        0x4E => Some((Instruction::LSR, AddressMode::Absolute)),
        0x5E => Some((Instruction::LSR, AddressMode::AbsoluteIndexedWithX)),
        0x4A => Some((Instruction::LSR, AddressMode::Accumulator)),
        0x46 => Some((Instruction::LSR, AddressMode::ZeroPage)),
        0x56 => Some((Instruction::LSR, AddressMode::ZeroPageIndexedWithX)),

        0xEA => Some((Instruction::NOP, AddressMode::Implied)),

        0x0D => Some((Instruction::ORA, AddressMode::Absolute)),
        0x1D => Some((Instruction::ORA, AddressMode::AbsoluteIndexedWithX)),
        0x19 => Some((Instruction::ORA, AddressMode::AbsoluteIndexedWithY)),
        0x09 => Some((Instruction::ORA, AddressMode::ImmediateAddressing)),
        0x05 => Some((Instruction::ORA, AddressMode::ZeroPage)),
        0x01 => Some((Instruction::ORA, AddressMode::ZeroPageIndexedIndirect)),
        0x15 => Some((Instruction::ORA, AddressMode::ZeroPageIndexedWithX)),
        0x12 => Some((Instruction::ORA, AddressMode::ZeroPageIndirect)),
        0x11 => Some((Instruction::ORA, AddressMode::ZeroPageIndirectIndexedWithY)),

        0x48 => Some((Instruction::PHA, AddressMode::Stack)),
        0x08 => Some((Instruction::PHP, AddressMode::Stack)),
        0xDA => Some((Instruction::PHX, AddressMode::Stack)),
        0x5A => Some((Instruction::PHY, AddressMode::Stack)),
        0x68 => Some((Instruction::PLA, AddressMode::Stack)),
        0x28 => Some((Instruction::PLP, AddressMode::Stack)),
        0xFA => Some((Instruction::PLX, AddressMode::Stack)),
        0x7A => Some((Instruction::PLY, AddressMode::Stack)),

        0x07 => Some((Instruction::RMB(0), AddressMode::ZeroPage)),
        0x17 => Some((Instruction::RMB(1), AddressMode::ZeroPage)),
        0x27 => Some((Instruction::RMB(2), AddressMode::ZeroPage)),
        0x37 => Some((Instruction::RMB(3), AddressMode::ZeroPage)),
        0x47 => Some((Instruction::RMB(4), AddressMode::ZeroPage)),
        0x57 => Some((Instruction::RMB(5), AddressMode::ZeroPage)),
        0x67 => Some((Instruction::RMB(6), AddressMode::ZeroPage)),
        0x77 => Some((Instruction::RMB(7), AddressMode::ZeroPage)),

        0x2E => Some((Instruction::ROL, AddressMode::Absolute)),
        0x3E => Some((Instruction::ROL, AddressMode::AbsoluteIndexedWithX)),
        0x2A => Some((Instruction::ROL, AddressMode::Accumulator)),
        0x26 => Some((Instruction::ROL, AddressMode::ZeroPage)),
        0x36 => Some((Instruction::ROL, AddressMode::ZeroPageIndexedWithX)),

        0x6E => Some((Instruction::ROR, AddressMode::Absolute)),
        0x7E => Some((Instruction::ROR, AddressMode::AbsoluteIndexedWithX)),
        0x6A => Some((Instruction::ROR, AddressMode::Accumulator)),
        0x66 => Some((Instruction::ROR, AddressMode::ZeroPage)),
        0x76 => Some((Instruction::ROR, AddressMode::ZeroPageIndexedWithX)),

        0x40 => Some((Instruction::RTI, AddressMode::Stack)),
        0x60 => Some((Instruction::RTS, AddressMode::Stack)),

        0xED => Some((Instruction::SBC, AddressMode::Absolute)),
        0xFD => Some((Instruction::SBC, AddressMode::AbsoluteIndexedWithX)),
        0xF9 => Some((Instruction::SBC, AddressMode::AbsoluteIndexedWithY)),
        0xE9 => Some((Instruction::SBC, AddressMode::ImmediateAddressing)),
        0xE5 => Some((Instruction::SBC, AddressMode::ZeroPage)),
        0xE1 => Some((Instruction::SBC, AddressMode::ZeroPageIndexedIndirect)),
        0xF5 => Some((Instruction::SBC, AddressMode::ZeroPageIndexedWithX)),
        0xF2 => Some((Instruction::SBC, AddressMode::ZeroPageIndirect)),
        0xF1 => Some((Instruction::SBC, AddressMode::ZeroPageIndirectIndexedWithY)),

        0x38 => Some((Instruction::SEC, AddressMode::Implied)),
        0xF8 => Some((Instruction::SED, AddressMode::Implied)),
        0x78 => Some((Instruction::SEI, AddressMode::Implied)),

        0x87 => Some((Instruction::SMB(0), AddressMode::ZeroPage)),
        0x97 => Some((Instruction::SMB(1), AddressMode::ZeroPage)),
        0xA7 => Some((Instruction::SMB(2), AddressMode::ZeroPage)),
        0xB7 => Some((Instruction::SMB(3), AddressMode::ZeroPage)),
        0xC7 => Some((Instruction::SMB(4), AddressMode::ZeroPage)),
        0xD7 => Some((Instruction::SMB(5), AddressMode::ZeroPage)),
        0xE7 => Some((Instruction::SMB(6), AddressMode::ZeroPage)),
        0xF7 => Some((Instruction::SMB(7), AddressMode::ZeroPage)),

        0x8D => Some((Instruction::STA, AddressMode::Absolute)),
        0x9D => Some((Instruction::STA, AddressMode::AbsoluteIndexedWithX)),
        0x99 => Some((Instruction::STA, AddressMode::AbsoluteIndexedWithY)),
        0x85 => Some((Instruction::STA, AddressMode::ZeroPage)),
        0x81 => Some((Instruction::STA, AddressMode::ZeroPageIndexedIndirect)),
        0x95 => Some((Instruction::STA, AddressMode::ZeroPageIndexedWithX)),
        0x92 => Some((Instruction::STA, AddressMode::ZeroPageIndirect)),
        0x91 => Some((Instruction::STA, AddressMode::ZeroPageIndirectIndexedWithY)),

        0xDB => Some((Instruction::STP, AddressMode::Implied)),

        0x8E => Some((Instruction::STX, AddressMode::Absolute)),
        0x86 => Some((Instruction::STX, AddressMode::ZeroPage)),
        0x96 => Some((Instruction::STX, AddressMode::ZeroPageIndexedWithX)),

        0x8C => Some((Instruction::STY, AddressMode::Absolute)),
        0x84 => Some((Instruction::STY, AddressMode::ZeroPage)),
        0x94 => Some((Instruction::STY, AddressMode::ZeroPageIndexedWithX)),

        0x9C => Some((Instruction::STZ, AddressMode::Absolute)),
        0x9E => Some((Instruction::STZ, AddressMode::AbsoluteIndexedWithX)),
        0x64 => Some((Instruction::STZ, AddressMode::ZeroPage)),
        0x74 => Some((Instruction::STZ, AddressMode::ZeroPageIndexedWithX)),

        0xAA => Some((Instruction::TAX, AddressMode::Implied)),
        0xA8 => Some((Instruction::TAY, AddressMode::Implied)),

        0x1C => Some((Instruction::TRB, AddressMode::Absolute)),
        0x14 => Some((Instruction::TRB, AddressMode::ZeroPage)),

        0x0C => Some((Instruction::TSB, AddressMode::Absolute)),
        0x04 => Some((Instruction::TSB, AddressMode::ZeroPage)),

        0xBA => Some((Instruction::TSX, AddressMode::Implied)),
        0x8A => Some((Instruction::TXA, AddressMode::Implied)),
        0x9A => Some((Instruction::TXS, AddressMode::Implied)),
        0x98 => Some((Instruction::TYA, AddressMode::Implied)),

        0xCB => Some((Instruction::WAI, AddressMode::Implied)),

        _ => None
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum CPUFlag {
    Carry    = 0x01,
    Zero     = 0x02,
    IRQB     = 0x04,
    Decimal  = 0x08,
    BRK      = 0x10,
    User     = 0x20,
    Overflow = 0x40,
    Negative = 0x80,
}

pub trait Attachment {
    // clock cycle
    fn step(&mut self);

    // data & address bus
    fn read(&self, addr : u16) -> u8;
    fn write(&mut self, addr : u16, data : u8);
}

pub struct W65C02S {
    pub attachments : Vec<(u16, u16, Box<dyn Attachment>)>,
    pub state       : CPUState, // cpu state
    pub ir          : Opcode,   // instruction register
    pub tcu         : u8,       // timing control unit
    pub a           : u8,       // accumulator register
    pub x           : u8,       // index register 'x'
    pub y           : u8,       // index register 'y'
    pub p           : u8,       // processor status register
    pub pc          : u16,      // program counter register
    pub s           : u8,       // stack pointer register
    pub temp8       : u8,      // temporary storage
    pub temp16      : u16,      // temporary storage
}

impl fmt::Debug for W65C02S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("W65C02S")
         .field("state", &self.state)
         .field("ir", &self.ir)
         .field("tcu", &self.tcu)
         .field("a", &self.a)
         .field("x", &self.x)
         .field("y", &self.y)
         .field("p", &self.p)
         .field("pc", &self.pc)
         .field("s", &self.s)
         .field("temp8", &self.temp8)
         .field("temp16", &self.temp16)
         .finish()
    }
}

impl W65C02S {
    pub fn new() -> W65C02S {
        W65C02S {
            attachments: Vec::new(),
            state:       CPUState::Init(0),
            ir:          (Instruction::NOP, AddressMode::Implied),
            tcu:         0,
            a:           0,
            x:           0,
            y:           0,
            p:           0,
            pc:          0,
            s:           0,
            temp8:       0,
            temp16:      0,
        }
    }

    pub fn is_halted(&self) -> bool {
        match self.state {
            CPUState::Halt => true,
            _ => false
        }
    }

    pub fn attach(&mut self, addr_mask: u16, addr_val: u16, member: Box<dyn Attachment>) {
        self.attachments.push((addr_mask, addr_val, member));
    }

    fn read(&self, addr : u16) -> u8 {
        debug!("R @ {:04x}", addr);

        let mut selected_members = self.attachments.iter().filter(move |&(mask, val, _)| { (addr & mask) == *val });
        match selected_members.next() {
            None => { panic!("no bus member responded to addr: {:04x}", addr); }
            Some((mask, _, member)) => {
                match selected_members.next() {
                    None => {
                        let data = member.read(addr & !mask);
                        data
                    }
                    _    => { panic!("multiple bus members responded to addr: {:04x}", addr); }
                }
            }
        }
    }

    fn write(&mut self, addr : u16, data : u8) {
        debug!("W @ {:04x} = {:02x}", addr, data);

        let mut selected_members = self.attachments.iter_mut().filter(|(mask, val, _)| { (addr & mask) == *val });
        match selected_members.next() {
            None => { panic!("no bus member responded to addr: {:04x}", addr); }
            Some((mask, _, member)) => {
                match selected_members.next() {
                    None => { member.write(addr & !*mask, data) }
                    _    => { panic!("multiple bus members responded to addr: {:04x}", addr); }
                }
            }
        }
    }

    fn push(&mut self, val : u8) {
        self.write(0x0100 + (self.s as u16), val);
        self.s = self.s.wrapping_sub(1);
    }

    fn pop(&mut self) -> u8 {
        self.s = self.s.wrapping_add(1);
        self.read(0x0100 + (self.s as u16))
    }

    fn peek(&mut self) -> u8 {
        self.read(0x0100 + (self.s as u16))
    }

    fn fetch(&mut self) -> u8 {
        let val = self.read(self.pc);
        self.pc += 1;
        val
    }

    fn update_zero_flag(&mut self, val : u8) {
        if val == 0 {
            self.p |= CPUFlag::Zero as u8;
        } else {
            self.p &= !(CPUFlag::Zero as u8);
        }
    }

    fn update_negative_flag(&mut self, val : u8) {
        if val & 0x80 == 0x80 {
            self.p |= CPUFlag::Negative as u8;
        } else {
            self.p &= !(CPUFlag::Negative as u8);
        }
    }

    fn update_carry_flag(&mut self, val : bool) {
        if val {
            self.p |= CPUFlag::Carry as u8;
        } else {
            self.p &= !(CPUFlag::Carry as u8);
        }
    }

    fn update_irqb_flag(&mut self, val : bool) {
        if val {
            self.p |= CPUFlag::IRQB as u8;
        } else {
            self.p &= !(CPUFlag::IRQB as u8);
        }
    }

    pub fn step(&mut self) {
        debug!("CPU: {:x?}", self);

        match self.state {
            CPUState::Init(c) => {
                match c {
                    5 => {
                        self.pc = self.read(0xFFFC) as u16;
                        self.state = CPUState::Init(c+1)
                    }
                    6 => {
                        self.pc = self.pc | ((self.read(0xFFFD) as u16) << 8);
                        self.state = CPUState::Run;
                    }
                    _ => {
                        self.state = CPUState::Init(c+1)
                    }
                }
            }
            CPUState::Run => {
                match (&self.ir, &self.tcu) {
                    // First step is always to fetch the next instruction
                    (_, 0) => {
                        let val = self.fetch();
                        match decode(val) {
                            Some(opcode) => {
                                debug!("DECODE: {:x?}", opcode);
                                self.ir = opcode;
                                self.tcu += 1;
                            }
                            None => {
                                debug!("FAILED DECODE: {:x?}", val);
                                self.state = CPUState::Halt;
                            }
                        }
                    }

                    // AND #
                    ((Instruction::AND, AddressMode::ImmediateAddressing), 1) => {
                        self.a &= self.fetch();
                        self.update_zero_flag(self.a);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    // AND a
                    ((Instruction::AND, AddressMode::Absolute), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.a &= self.read(self.temp16);
                        self.update_zero_flag(self.a);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    // ASL A
                    ((Instruction::ASL, AddressMode::Accumulator), 1) => {
                        self.update_carry_flag(self.a & 0x80 == 0);
                        self.a = self.a << 1;
                        self.update_zero_flag(self.a);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    // ASL a
                    ((Instruction::ASL, AddressMode::Absolute), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::ASL, AddressMode::Absolute), 4) => {
                        self.update_carry_flag(self.temp8 & 0x80 == 0);
                        self.temp8 <<= 1;
                        self.update_zero_flag(self.a);
                        self.update_negative_flag(self.a);
                        self.tcu += 1;
                    }
                    ((Instruction::ASL, AddressMode::Absolute), 5) => {
                        self.write(self.temp16, self.temp8);
                        self.tcu = 0;
                    }

                    // BCS r
                    ((Instruction::BCS, AddressMode::ProgramCounterRelative), 1) => {
                        self.temp8 = self.fetch();
                        if self.p & (CPUFlag::Carry as u8) == 0 {
                            self.tcu = 0;
                        } else {
                            self.tcu += 1;
                        }
                    }

                    // BEQ r
                    ((Instruction::BEQ, AddressMode::ProgramCounterRelative), 1) => {
                        self.temp8 = self.fetch();
                        if self.p & (CPUFlag::Zero as u8) == 0 {
                            self.tcu = 0;
                        } else {
                            self.tcu += 1;
                        }
                    }

                    // BMI r
                    ((Instruction::BMI, AddressMode::ProgramCounterRelative), 1) => {
                        self.temp8 = self.fetch();
                        if self.p & (CPUFlag::Negative as u8) == 0 {
                            self.tcu = 0;
                        } else {
                            self.tcu += 1;
                        }
                    }

                    // BNE r
                    ((Instruction::BNE, AddressMode::ProgramCounterRelative), 1) => {
                        self.temp8 = self.fetch();
                        if self.p & (CPUFlag::Zero as u8) == 0 {
                            self.tcu += 1;
                        } else {
                            self.tcu = 0;
                        }
                    }

                    // BPL r
                    ((Instruction::BPL, AddressMode::ProgramCounterRelative), 1) => {
                        self.temp8 = self.fetch();
                        if self.p & (CPUFlag::Negative as u8) == 0 {
                            self.tcu += 1;
                        } else {
                            self.tcu = 0;
                        }
                    }

                    // CLI i
                    ((Instruction::CLI, AddressMode::Implied), 1) => {
                        self.update_irqb_flag(false);
                        self.tcu = 0;
                    }

                    // CMP #
                    ((Instruction::CMP, AddressMode::ImmediateAddressing), 1) => {
                        let val = self.fetch();
                        self.update_carry_flag(self.a >= val);
                        let val = self.a.wrapping_sub(val);
                        self.update_zero_flag(val);
                        self.update_negative_flag(val);
                        self.tcu = 0;
                    }

                    // CPX #
                    ((Instruction::CPX, AddressMode::ImmediateAddressing), 1) => {
                        let val = self.fetch();
                        self.update_carry_flag(self.x >= val);
                        let val = self.x.wrapping_sub(val);
                        self.update_zero_flag(val);
                        self.update_negative_flag(val);
                        self.tcu = 0;
                    }

                    // CPY #
                    ((Instruction::CPY, AddressMode::ImmediateAddressing), 1) => {
                        let val = self.fetch();
                        self.update_carry_flag(self.y >= val);
                        let val = self.y.wrapping_sub(val);
                        self.update_zero_flag(val);
                        self.update_negative_flag(val);
                        self.tcu = 0;
                    }

                    // DEC A
                    ((Instruction::DEC, AddressMode::Accumulator), 1) => {
                        self.a = self.a.wrapping_sub(1);
                        self.update_zero_flag(self.x);
                        self.update_negative_flag(self.x);
                        self.tcu = 0;
                    }

                    // DEX i
                    ((Instruction::DEX, AddressMode::Implied), 1) => {
                        self.x = self.x.wrapping_sub(1);
                        self.update_zero_flag(self.x);
                        self.update_negative_flag(self.x);
                        self.tcu = 0;
                    }

                    // DEY i
                    ((Instruction::DEY, AddressMode::Implied), 1) => {
                        self.y = self.y.wrapping_sub(1);
                        self.update_zero_flag(self.y);
                        self.update_negative_flag(self.y);
                        self.tcu = 0;
                    }

                    // EOR a
                    ((Instruction::EOR, AddressMode::Absolute), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.a ^= self.read(self.temp16);
                        self.update_zero_flag(self.a);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    // INC a
                    ((Instruction::INC, AddressMode::Absolute), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::INC, AddressMode::Absolute), 4) => {
                        self.temp8 += 1;
                        self.update_zero_flag(self.temp8);
                        self.update_negative_flag(self.temp8);
                        self.tcu += 1;
                    }
                    ((Instruction::INC, AddressMode::Absolute), 5) => {
                        self.write(self.temp16, self.temp8);
                        self.tcu = 0;
                    }

                    // INX i
                    ((Instruction::INX, AddressMode::Implied), 1) => {
                        self.x = self.x.wrapping_add(1);
                        self.update_zero_flag(self.x);
                        self.update_negative_flag(self.x);
                        self.tcu = 0;
                    }

                    // INY i
                    ((Instruction::INY, AddressMode::Implied), 1) => {
                        self.y = self.y.wrapping_add(1);
                        self.update_zero_flag(self.y);
                        self.update_negative_flag(self.y);
                        self.tcu = 0;
                    }

                    // JMP a
                    ((Instruction::JMP, AddressMode::Absolute), 3) => {
                        self.pc = self.temp16;
                        self.tcu = 0;
                    }

                    // JSR a
                    ((Instruction::JSR, AddressMode::Absolute), 1) => {
                        self.temp16 = self.fetch() as u16;
                        self.tcu += 1;
                    }
                    ((Instruction::JSR, AddressMode::Absolute), 2) => {
                        self.peek();
                        self.tcu += 1;
                    }
                    ((Instruction::JSR, AddressMode::Absolute), 3) => {
                        self.push((self.pc >> 8) as u8);
                        self.tcu += 1;
                    }
                    ((Instruction::JSR, AddressMode::Absolute), 4) => {
                        self.push((self.pc & 0xFF) as u8);
                        self.tcu += 1;
                    }
                    ((Instruction::JSR, AddressMode::Absolute), 5) => {
                        self.temp16 = self.temp16 | ((self.fetch() as u16) << 8);
                        self.pc = self.temp16;
                        self.tcu = 0;
                    }

                    // LDA #
                    ((Instruction::LDA, AddressMode::ImmediateAddressing), 1) => {
                        self.a = self.fetch();
                        self.update_zero_flag(self.a);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    // LDA a
                    ((Instruction::LDA, AddressMode::Absolute), 3) => {
                        self.a = self.read(self.temp16);
                        self.update_zero_flag(self.a);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    // LDA (zp),y
                    ((Instruction::LDA, AddressMode::ZeroPageIndirectIndexedWithY), 4) => {
                        self.a = self.read(self.temp16 + (self.y as u16));
                        self.update_zero_flag(self.a);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    // LDX #
                    ((Instruction::LDX, AddressMode::ImmediateAddressing), 1) => {
                        self.x = self.fetch();
                        self.update_zero_flag(self.x);
                        self.update_negative_flag(self.x);
                        self.tcu = 0;
                    }

                    // LDX a
                    ((Instruction::LDX, AddressMode::Absolute), 3) => {
                        self.x = self.read(self.temp16);
                        self.update_zero_flag(self.x);
                        self.update_negative_flag(self.x);
                        self.tcu = 0;
                    }

                    // LDX a,y
                    ((Instruction::LDX, AddressMode::AbsoluteIndexedWithY), 3) => {
                        self.x = self.read(self.temp16);
                        self.update_zero_flag(self.x);
                        self.update_negative_flag(self.x);
                        self.tcu = 0;
                    }

                    // LDY #
                    ((Instruction::LDY, AddressMode::ImmediateAddressing), 1) => {
                        self.y = self.fetch();
                        self.update_zero_flag(self.y);
                        self.update_negative_flag(self.y);
                        self.tcu = 0;
                    }

                    // LDY a
                    ((Instruction::LDY, AddressMode::Absolute), 3) => {
                        self.y = self.read(self.temp16);
                        self.update_zero_flag(self.y);
                        self.update_negative_flag(self.y);
                        self.tcu = 0;
                    }

                    // NOP i
                    ((Instruction::NOP, AddressMode::Implied), 1) => {
                        self.tcu = 0;
                    }

                    // RTS s
                    ((Instruction::RTS, AddressMode::Stack), 1) => {
                        self.fetch();
                        self.tcu += 1;
                    }
                    ((Instruction::RTS, AddressMode::Stack), 2) => {
                        self.peek();
                        self.tcu += 1;
                    }
                    ((Instruction::RTS, AddressMode::Stack), 3) => {
                        self.temp16 = self.pop() as u16;
                        self.tcu += 1;
                    }
                    ((Instruction::RTS, AddressMode::Stack), 4) => {
                        self.temp16 |= (self.pop() as u16) << 8;
                        self.tcu += 1;
                    }
                    ((Instruction::RTS, AddressMode::Stack), 5) => {
                        self.pc = self.temp16;
                        self.fetch();
                        self.tcu = 0;
                    }

                    // SEI i
                    ((Instruction::SEI, AddressMode::Implied), 1) => {
                        self.update_irqb_flag(true);
                        self.tcu = 0;
                    }

                    // STA zp
                    ((Instruction::STA, AddressMode::ZeroPage), 2) => {
                        self.write(self.temp16, self.a);
                        self.tcu = 0;
                    }

                    // STA a
                    ((Instruction::STA, AddressMode::Absolute), 3) => {
                        self.write(self.temp16, self.a);
                        self.tcu = 0;
                    }

                    // STA a,y
                    ((Instruction::STA, AddressMode::AbsoluteIndexedWithY), 4) => {
                        self.write(self.temp16, self.a);
                        self.tcu = 0;
                    }

                    // STX a
                    ((Instruction::STX, AddressMode::Absolute), 3) => {
                        self.write(self.temp16, self.x);
                        self.tcu = 0;
                    }

                    // STY a
                    ((Instruction::STY, AddressMode::Absolute), 3) => {
                        self.write(self.temp16, self.y);
                        self.tcu = 0;
                    }

                    // STZ a
                    ((Instruction::STZ, AddressMode::Absolute), 3) => {
                        self.write(self.temp16, 0);
                        self.tcu = 0;
                    }

                    // TXS i
                    ((Instruction::TXS, AddressMode::Implied), 1) => {
                        self.s = self.x;
                        self.tcu = 0;
                    }

                    // Default for Absolute
                    ((_, AddressMode::Absolute), 1) => {
                        self.temp16 = self.fetch() as u16;
                        self.tcu += 1;
                    }
                    ((_, AddressMode::Absolute), 2) => {
                        self.temp16 = self.temp16 | ((self.fetch() as u16) << 8);
                        self.tcu += 1;
                    }

                    // Default for Absolute Indexed With X
                    ((_, AddressMode::AbsoluteIndexedWithX), 1) => {
                        self.temp16 = self.fetch() as u16;
                        self.tcu += 1;
                    }
                    ((_, AddressMode::AbsoluteIndexedWithX), 2) => {
                        self.temp16 = self.temp16 | ((self.fetch() as u16) << 8);
                        self.tcu += 1;
                    }
                    ((_, AddressMode::AbsoluteIndexedWithX), 3) => {
                        self.temp16 += self.x as u16;
                        self.tcu += 1;
                    }

                    // Default for Absolute Indexed With Y
                    ((_, AddressMode::AbsoluteIndexedWithY), 1) => {
                        self.temp16 = self.fetch() as u16;
                        self.tcu += 1;
                    }
                    ((_, AddressMode::AbsoluteIndexedWithY), 2) => {
                        self.temp16 = self.temp16 | ((self.fetch() as u16) << 8);
                        self.tcu += 1;
                    }
                    ((_, AddressMode::AbsoluteIndexedWithY), 3) => {
                        self.temp16 += self.y as u16;
                        self.tcu += 1;
                    }

                    // Default for Program Counter Relative
                    ((_, AddressMode::ProgramCounterRelative), 2) => {
                        let offset = self.temp8 as i8;
                        if offset >= 0 {
                            self.pc += offset as u16;
                        } else {
                            self.pc -= offset.abs() as u16;
                        }
                        self.tcu = 0;
                    }

                    // Default for Zero Page
                    ((_, AddressMode::ZeroPage), 1) => {
                        self.temp16 = self.fetch() as u16;
                        self.tcu += 1;
                    }

                    // Default for Zero Page Indirect Indexed With Y
                    ((_, AddressMode::ZeroPageIndirectIndexedWithY), 1) => {
                        self.temp8 = self.fetch();
                        self.tcu += 1;
                    }
                    ((_, AddressMode::ZeroPageIndirectIndexedWithY), 2) => {
                        self.temp16 = self.read(self.temp8 as u16) as u16;
                        self.tcu += 1;
                    }
                    ((_, AddressMode::ZeroPageIndirectIndexedWithY), 3) => {
                        self.temp16 = (self.read((self.temp8 + 1) as u16) as u16) << 8;
                        self.tcu += 1;
                    }

                    // Unimplemented
                    _ => {
                        self.state = CPUState::Halt;
                    }
                }
            }
            CPUState::Halt => {
            }
        }

        self.attachments.iter_mut().for_each(|(_, _, m)| { m.step(); });
    }
}
