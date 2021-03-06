use log::{debug, info};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum CPUState {
    Init(u8),
    Run,
    Wait,
    Halt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressMode {
    Absolute,                     // a
    AbsoluteIndexedIndirect,      // (a,x)
    AbsoluteIndexedWithX,         // a,x
    AbsoluteIndexedWithY,         // a,y
    AbsoluteIndirect,             // (a)
    Accumulator,                  // A
    ImmediateAddressing,          // #
    Implied,                      // i
    ProgramCounterRelative,       // r
    Stack,                        // s
    ZeroPage,                     // zp
    ZeroPageIndexedIndirect,      // (zp,x)
    ZeroPageIndexedWithX,         // zp,x
    ZeroPageIndexedWithY,         // zp,y
    ZeroPageIndirect,             // (zp)
    ZeroPageIndirectIndexedWithY, // (zp),y
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    NOP(u8, u8),
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
    WAI,
}

pub type Opcode = (Instruction, AddressMode);

fn decode(val: u8) -> Opcode {
    match val {
        0x6D => (Instruction::ADC, AddressMode::Absolute),
        0x7D => (Instruction::ADC, AddressMode::AbsoluteIndexedWithX),
        0x79 => (Instruction::ADC, AddressMode::AbsoluteIndexedWithY),
        0x69 => (Instruction::ADC, AddressMode::ImmediateAddressing),
        0x65 => (Instruction::ADC, AddressMode::ZeroPage),
        0x61 => (Instruction::ADC, AddressMode::ZeroPageIndexedIndirect),
        0x75 => (Instruction::ADC, AddressMode::ZeroPageIndexedWithX),
        0x72 => (Instruction::ADC, AddressMode::ZeroPageIndirect),
        0x71 => (Instruction::ADC, AddressMode::ZeroPageIndirectIndexedWithY),

        0x2D => (Instruction::AND, AddressMode::Absolute),
        0x3D => (Instruction::AND, AddressMode::AbsoluteIndexedWithX),
        0x39 => (Instruction::AND, AddressMode::AbsoluteIndexedWithY),
        0x29 => (Instruction::AND, AddressMode::ImmediateAddressing),
        0x25 => (Instruction::AND, AddressMode::ZeroPage),
        0x21 => (Instruction::AND, AddressMode::ZeroPageIndexedIndirect),
        0x35 => (Instruction::AND, AddressMode::ZeroPageIndexedWithX),
        0x32 => (Instruction::AND, AddressMode::ZeroPageIndirect),
        0x31 => (Instruction::AND, AddressMode::ZeroPageIndirectIndexedWithY),

        0x0E => (Instruction::ASL, AddressMode::Absolute),
        0x1E => (Instruction::ASL, AddressMode::AbsoluteIndexedWithX),
        0x0A => (Instruction::ASL, AddressMode::Accumulator),
        0x06 => (Instruction::ASL, AddressMode::ZeroPage),
        0x16 => (Instruction::ASL, AddressMode::ZeroPageIndexedWithX),

        0x0F => (Instruction::BBR(0), AddressMode::ProgramCounterRelative),
        0x1F => (Instruction::BBR(1), AddressMode::ProgramCounterRelative),
        0x2F => (Instruction::BBR(2), AddressMode::ProgramCounterRelative),
        0x3F => (Instruction::BBR(3), AddressMode::ProgramCounterRelative),
        0x4F => (Instruction::BBR(4), AddressMode::ProgramCounterRelative),
        0x5F => (Instruction::BBR(5), AddressMode::ProgramCounterRelative),
        0x6F => (Instruction::BBR(6), AddressMode::ProgramCounterRelative),
        0x7F => (Instruction::BBR(7), AddressMode::ProgramCounterRelative),

        0x8F => (Instruction::BBS(0), AddressMode::ProgramCounterRelative),
        0x9F => (Instruction::BBS(1), AddressMode::ProgramCounterRelative),
        0xAF => (Instruction::BBS(2), AddressMode::ProgramCounterRelative),
        0xBF => (Instruction::BBS(3), AddressMode::ProgramCounterRelative),
        0xCF => (Instruction::BBS(4), AddressMode::ProgramCounterRelative),
        0xDF => (Instruction::BBS(5), AddressMode::ProgramCounterRelative),
        0xEF => (Instruction::BBS(6), AddressMode::ProgramCounterRelative),
        0xFF => (Instruction::BBS(7), AddressMode::ProgramCounterRelative),

        0x90 => (Instruction::BCC, AddressMode::ProgramCounterRelative),
        0xB0 => (Instruction::BCS, AddressMode::ProgramCounterRelative),
        0xF0 => (Instruction::BEQ, AddressMode::ProgramCounterRelative),

        0x2C => (Instruction::BIT, AddressMode::Absolute),
        0x3C => (Instruction::BIT, AddressMode::AbsoluteIndexedWithX),
        0x89 => (Instruction::BIT, AddressMode::ImmediateAddressing),
        0x24 => (Instruction::BIT, AddressMode::ZeroPage),
        0x34 => (Instruction::BIT, AddressMode::ZeroPageIndexedWithX),

        0x30 => (Instruction::BMI, AddressMode::ProgramCounterRelative),
        0xD0 => (Instruction::BNE, AddressMode::ProgramCounterRelative),
        0x10 => (Instruction::BPL, AddressMode::ProgramCounterRelative),
        0x80 => (Instruction::BRA, AddressMode::ProgramCounterRelative),

        0x00 => (Instruction::BRK, AddressMode::Stack),

        0x50 => (Instruction::BVC, AddressMode::ProgramCounterRelative),
        0x70 => (Instruction::BVS, AddressMode::ProgramCounterRelative),

        0x18 => (Instruction::CLC, AddressMode::Implied),
        0xD8 => (Instruction::CLD, AddressMode::Implied),
        0x58 => (Instruction::CLI, AddressMode::Implied),
        0xB8 => (Instruction::CLV, AddressMode::Implied),

        0xCD => (Instruction::CMP, AddressMode::Absolute),
        0xDD => (Instruction::CMP, AddressMode::AbsoluteIndexedWithX),
        0xD9 => (Instruction::CMP, AddressMode::AbsoluteIndexedWithY),
        0xC9 => (Instruction::CMP, AddressMode::ImmediateAddressing),
        0xC5 => (Instruction::CMP, AddressMode::ZeroPage),
        0xC1 => (Instruction::CMP, AddressMode::ZeroPageIndexedIndirect),
        0xD5 => (Instruction::CMP, AddressMode::ZeroPageIndexedWithX),
        0xD2 => (Instruction::CMP, AddressMode::ZeroPageIndirect),
        0xD1 => (Instruction::CMP, AddressMode::ZeroPageIndirectIndexedWithY),

        0xEC => (Instruction::CPX, AddressMode::Absolute),
        0xE0 => (Instruction::CPX, AddressMode::ImmediateAddressing),
        0xE4 => (Instruction::CPX, AddressMode::ZeroPage),

        0xCC => (Instruction::CPY, AddressMode::Absolute),
        0xC0 => (Instruction::CPY, AddressMode::ImmediateAddressing),
        0xC4 => (Instruction::CPY, AddressMode::ZeroPage),

        0xCE => (Instruction::DEC, AddressMode::Absolute),
        0xDE => (Instruction::DEC, AddressMode::AbsoluteIndexedWithX),
        0x3A => (Instruction::DEC, AddressMode::Accumulator),
        0xC6 => (Instruction::DEC, AddressMode::ZeroPage),
        0xD6 => (Instruction::DEC, AddressMode::ZeroPageIndexedWithX),

        0xCA => (Instruction::DEX, AddressMode::Implied),
        0x88 => (Instruction::DEY, AddressMode::Implied),

        0x4D => (Instruction::EOR, AddressMode::Absolute),
        0x5D => (Instruction::EOR, AddressMode::AbsoluteIndexedWithX),
        0x59 => (Instruction::EOR, AddressMode::AbsoluteIndexedWithY),
        0x49 => (Instruction::EOR, AddressMode::ImmediateAddressing),
        0x45 => (Instruction::EOR, AddressMode::ZeroPage),
        0x41 => (Instruction::EOR, AddressMode::ZeroPageIndexedIndirect),
        0x55 => (Instruction::EOR, AddressMode::ZeroPageIndexedWithX),
        0x52 => (Instruction::EOR, AddressMode::ZeroPageIndirect),
        0x51 => (Instruction::EOR, AddressMode::ZeroPageIndirectIndexedWithY),

        0xEE => (Instruction::INC, AddressMode::Absolute),
        0xFE => (Instruction::INC, AddressMode::AbsoluteIndexedWithX),
        0x1A => (Instruction::INC, AddressMode::Accumulator),
        0xE6 => (Instruction::INC, AddressMode::ZeroPage),
        0xF6 => (Instruction::INC, AddressMode::ZeroPageIndexedWithX),

        0xE8 => (Instruction::INX, AddressMode::Implied),
        0xC8 => (Instruction::INY, AddressMode::Implied),

        0x4C => (Instruction::JMP, AddressMode::Absolute),
        0x7C => (Instruction::JMP, AddressMode::AbsoluteIndexedIndirect),
        0x6C => (Instruction::JMP, AddressMode::AbsoluteIndirect),

        0x20 => (Instruction::JSR, AddressMode::Absolute),

        0xAD => (Instruction::LDA, AddressMode::Absolute),
        0xBD => (Instruction::LDA, AddressMode::AbsoluteIndexedWithX),
        0xB9 => (Instruction::LDA, AddressMode::AbsoluteIndexedWithY),
        0xA9 => (Instruction::LDA, AddressMode::ImmediateAddressing),
        0xA5 => (Instruction::LDA, AddressMode::ZeroPage),
        0xA1 => (Instruction::LDA, AddressMode::ZeroPageIndexedIndirect),
        0xB5 => (Instruction::LDA, AddressMode::ZeroPageIndexedWithX),
        0xB2 => (Instruction::LDA, AddressMode::ZeroPageIndirect),
        0xB1 => (Instruction::LDA, AddressMode::ZeroPageIndirectIndexedWithY),

        0xAE => (Instruction::LDX, AddressMode::Absolute),
        0xBE => (Instruction::LDX, AddressMode::AbsoluteIndexedWithY),
        0xA2 => (Instruction::LDX, AddressMode::ImmediateAddressing),
        0xA6 => (Instruction::LDX, AddressMode::ZeroPage),
        0xB6 => (Instruction::LDX, AddressMode::ZeroPageIndexedWithY),

        0xAC => (Instruction::LDY, AddressMode::Absolute),
        0xBC => (Instruction::LDY, AddressMode::AbsoluteIndexedWithX),
        0xA0 => (Instruction::LDY, AddressMode::ImmediateAddressing),
        0xA4 => (Instruction::LDY, AddressMode::ZeroPage),
        0xB4 => (Instruction::LDY, AddressMode::ZeroPageIndexedWithX),

        0x4E => (Instruction::LSR, AddressMode::Absolute),
        0x5E => (Instruction::LSR, AddressMode::AbsoluteIndexedWithX),
        0x4A => (Instruction::LSR, AddressMode::Accumulator),
        0x46 => (Instruction::LSR, AddressMode::ZeroPage),
        0x56 => (Instruction::LSR, AddressMode::ZeroPageIndexedWithX),

        0xEA => (Instruction::NOP(1, 2), AddressMode::Implied),
        
        0x02 => (Instruction::NOP(2, 2), AddressMode::Implied),
        0x22 => (Instruction::NOP(2, 2), AddressMode::Implied),
        0x42 => (Instruction::NOP(2, 2), AddressMode::Implied),
        0x62 => (Instruction::NOP(2, 2), AddressMode::Implied),
        0x82 => (Instruction::NOP(2, 2), AddressMode::Implied),
        0xC2 => (Instruction::NOP(2, 2), AddressMode::Implied),
        0xE2 => (Instruction::NOP(2, 2), AddressMode::Implied),
        
        0x03 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x13 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x23 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x33 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x43 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x53 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x63 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x73 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x83 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x93 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0xA3 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0xB3 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0xC3 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0xD3 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0xE3 => (Instruction::NOP(1, 1), AddressMode::Implied),
        0xF3 => (Instruction::NOP(1, 1), AddressMode::Implied),

        0x44 => (Instruction::NOP(2, 3), AddressMode::Implied),
        0x54 => (Instruction::NOP(2, 4), AddressMode::Implied),
        0xD4 => (Instruction::NOP(2, 4), AddressMode::Implied),
        0xF4 => (Instruction::NOP(2, 4), AddressMode::Implied),

        0x0B => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x1B => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x2B => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x3B => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x4B => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x5B => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x6B => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x7B => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x8B => (Instruction::NOP(1, 1), AddressMode::Implied),
        0x9B => (Instruction::NOP(1, 1), AddressMode::Implied),
        0xAB => (Instruction::NOP(1, 1), AddressMode::Implied),
        0xBB => (Instruction::NOP(1, 1), AddressMode::Implied),
        0xEB => (Instruction::NOP(1, 1), AddressMode::Implied),
        0xFB => (Instruction::NOP(1, 1), AddressMode::Implied),
        
        0x5C => (Instruction::NOP(3, 8), AddressMode::Implied),
        0xDC => (Instruction::NOP(3, 4), AddressMode::Implied),
        0xFC => (Instruction::NOP(3, 4), AddressMode::Implied),

        0x0D => (Instruction::ORA, AddressMode::Absolute),
        0x1D => (Instruction::ORA, AddressMode::AbsoluteIndexedWithX),
        0x19 => (Instruction::ORA, AddressMode::AbsoluteIndexedWithY),
        0x09 => (Instruction::ORA, AddressMode::ImmediateAddressing),
        0x05 => (Instruction::ORA, AddressMode::ZeroPage),
        0x01 => (Instruction::ORA, AddressMode::ZeroPageIndexedIndirect),
        0x15 => (Instruction::ORA, AddressMode::ZeroPageIndexedWithX),
        0x12 => (Instruction::ORA, AddressMode::ZeroPageIndirect),
        0x11 => (Instruction::ORA, AddressMode::ZeroPageIndirectIndexedWithY),

        0x48 => (Instruction::PHA, AddressMode::Stack),
        0x08 => (Instruction::PHP, AddressMode::Stack),
        0xDA => (Instruction::PHX, AddressMode::Stack),
        0x5A => (Instruction::PHY, AddressMode::Stack),
        0x68 => (Instruction::PLA, AddressMode::Stack),
        0x28 => (Instruction::PLP, AddressMode::Stack),
        0xFA => (Instruction::PLX, AddressMode::Stack),
        0x7A => (Instruction::PLY, AddressMode::Stack),

        0x07 => (Instruction::RMB(0), AddressMode::ZeroPage),
        0x17 => (Instruction::RMB(1), AddressMode::ZeroPage),
        0x27 => (Instruction::RMB(2), AddressMode::ZeroPage),
        0x37 => (Instruction::RMB(3), AddressMode::ZeroPage),
        0x47 => (Instruction::RMB(4), AddressMode::ZeroPage),
        0x57 => (Instruction::RMB(5), AddressMode::ZeroPage),
        0x67 => (Instruction::RMB(6), AddressMode::ZeroPage),
        0x77 => (Instruction::RMB(7), AddressMode::ZeroPage),

        0x2E => (Instruction::ROL, AddressMode::Absolute),
        0x3E => (Instruction::ROL, AddressMode::AbsoluteIndexedWithX),
        0x2A => (Instruction::ROL, AddressMode::Accumulator),
        0x26 => (Instruction::ROL, AddressMode::ZeroPage),
        0x36 => (Instruction::ROL, AddressMode::ZeroPageIndexedWithX),

        0x6E => (Instruction::ROR, AddressMode::Absolute),
        0x7E => (Instruction::ROR, AddressMode::AbsoluteIndexedWithX),
        0x6A => (Instruction::ROR, AddressMode::Accumulator),
        0x66 => (Instruction::ROR, AddressMode::ZeroPage),
        0x76 => (Instruction::ROR, AddressMode::ZeroPageIndexedWithX),

        0x40 => (Instruction::RTI, AddressMode::Stack),
        0x60 => (Instruction::RTS, AddressMode::Stack),

        0xED => (Instruction::SBC, AddressMode::Absolute),
        0xFD => (Instruction::SBC, AddressMode::AbsoluteIndexedWithX),
        0xF9 => (Instruction::SBC, AddressMode::AbsoluteIndexedWithY),
        0xE9 => (Instruction::SBC, AddressMode::ImmediateAddressing),
        0xE5 => (Instruction::SBC, AddressMode::ZeroPage),
        0xE1 => (Instruction::SBC, AddressMode::ZeroPageIndexedIndirect),
        0xF5 => (Instruction::SBC, AddressMode::ZeroPageIndexedWithX),
        0xF2 => (Instruction::SBC, AddressMode::ZeroPageIndirect),
        0xF1 => (Instruction::SBC, AddressMode::ZeroPageIndirectIndexedWithY),

        0x38 => (Instruction::SEC, AddressMode::Implied),
        0xF8 => (Instruction::SED, AddressMode::Implied),
        0x78 => (Instruction::SEI, AddressMode::Implied),

        0x87 => (Instruction::SMB(0), AddressMode::ZeroPage),
        0x97 => (Instruction::SMB(1), AddressMode::ZeroPage),
        0xA7 => (Instruction::SMB(2), AddressMode::ZeroPage),
        0xB7 => (Instruction::SMB(3), AddressMode::ZeroPage),
        0xC7 => (Instruction::SMB(4), AddressMode::ZeroPage),
        0xD7 => (Instruction::SMB(5), AddressMode::ZeroPage),
        0xE7 => (Instruction::SMB(6), AddressMode::ZeroPage),
        0xF7 => (Instruction::SMB(7), AddressMode::ZeroPage),

        0x8D => (Instruction::STA, AddressMode::Absolute),
        0x9D => (Instruction::STA, AddressMode::AbsoluteIndexedWithX),
        0x99 => (Instruction::STA, AddressMode::AbsoluteIndexedWithY),
        0x85 => (Instruction::STA, AddressMode::ZeroPage),
        0x81 => (Instruction::STA, AddressMode::ZeroPageIndexedIndirect),
        0x95 => (Instruction::STA, AddressMode::ZeroPageIndexedWithX),
        0x92 => (Instruction::STA, AddressMode::ZeroPageIndirect),
        0x91 => (Instruction::STA, AddressMode::ZeroPageIndirectIndexedWithY),

        0xDB => (Instruction::STP, AddressMode::Implied),

        0x8E => (Instruction::STX, AddressMode::Absolute),
        0x86 => (Instruction::STX, AddressMode::ZeroPage),
        0x96 => (Instruction::STX, AddressMode::ZeroPageIndexedWithY),

        0x8C => (Instruction::STY, AddressMode::Absolute),
        0x84 => (Instruction::STY, AddressMode::ZeroPage),
        0x94 => (Instruction::STY, AddressMode::ZeroPageIndexedWithX),

        0x9C => (Instruction::STZ, AddressMode::Absolute),
        0x9E => (Instruction::STZ, AddressMode::AbsoluteIndexedWithX),
        0x64 => (Instruction::STZ, AddressMode::ZeroPage),
        0x74 => (Instruction::STZ, AddressMode::ZeroPageIndexedWithX),

        0xAA => (Instruction::TAX, AddressMode::Implied),
        0xA8 => (Instruction::TAY, AddressMode::Implied),

        0x1C => (Instruction::TRB, AddressMode::Absolute),
        0x14 => (Instruction::TRB, AddressMode::ZeroPage),

        0x0C => (Instruction::TSB, AddressMode::Absolute),
        0x04 => (Instruction::TSB, AddressMode::ZeroPage),

        0xBA => (Instruction::TSX, AddressMode::Implied),
        0x8A => (Instruction::TXA, AddressMode::Implied),
        0x9A => (Instruction::TXS, AddressMode::Implied),
        0x98 => (Instruction::TYA, AddressMode::Implied),

        0xCB => (Instruction::WAI, AddressMode::Implied),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum CPUFlag {
    Carry = 0x01,
    Zero = 0x02,
    IRQB = 0x04,
    Decimal = 0x08,
    BRK = 0x10,
    User = 0x20,
    Overflow = 0x40,
    Negative = 0x80,
}

pub trait Bus {
    fn peek(&self, addr: u16) -> u8;
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

pub struct W65C02S<BusType: Bus> {
    pub state: CPUState, // cpu state
    pub ir: Opcode,      // instruction register
    pub tcu: u8,         // timing control unit
    pub a: u8,           // accumulator register
    pub x: u8,           // index register 'x'
    pub y: u8,           // index register 'y'
    pub p: u8,           // processor status register
    pub pc: u16,         // program counter register
    pub s: u8,           // stack pointer register
    pub temp8: u8,       // temporary storage
    pub temp16: u16,     // temporary storage
    pub interrupt: bool, // an interrupt is available
    pub bus: BusType,
}

impl<BusType: Bus> fmt::Debug for W65C02S<BusType> {
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

impl<BusType: Bus> W65C02S<BusType> {
    pub fn new(bus : BusType) -> W65C02S<BusType> {
        W65C02S {
            state: CPUState::Init(0),
            ir: (Instruction::NOP(0, 0), AddressMode::Implied),
            tcu: 0,
            a: 0,
            x: 0,
            y: 0,
            p: 0,
            pc: 0,
            s: 0,
            temp8: 0,
            temp16: 0,
            interrupt: false,
            bus: bus,
        }
    }

    pub fn is_halted(&self) -> bool {
        match self.state {
            CPUState::Halt => true,
            _ => false,
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        self.bus.read(addr)
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.bus.write(addr, val)
    }

    fn stack_push(&mut self, val: u8) {
        self.write(0x0100 + (self.s as u16), val);
        self.s = self.s.wrapping_sub(1);
    }

    fn stack_pop(&mut self) -> u8 {
        self.s = self.s.wrapping_add(1);
        self.read(0x0100 + (self.s as u16))
    }

    fn stack_peek(&mut self) -> u8 {
        self.read(0x0100 + (self.s as u16))
    }

    fn fetch(&mut self) -> u8 {
        let val = self.read(self.pc);
        self.pc += 1;
        val
    }

    fn update_zero_flag(&mut self, val: bool) {
        if val {
            self.p |= CPUFlag::Zero as u8;
        } else {
            self.p &= !(CPUFlag::Zero as u8);
        }
    }

    fn update_negative_flag(&mut self, val: u8) {
        if val & 0x80 == 0x80 {
            self.p |= CPUFlag::Negative as u8;
        } else {
            self.p &= !(CPUFlag::Negative as u8);
        }
    }

    fn update_decimal_flag(&mut self, val: bool) {
        if val {
            self.p |= CPUFlag::Decimal as u8;
        } else {
            self.p &= !(CPUFlag::Decimal as u8);
        }
    }

    fn update_overflow_flag(&mut self, val: bool) {
        if val {
            self.p |= CPUFlag::Overflow as u8;
        } else {
            self.p &= !(CPUFlag::Overflow as u8);
        }
    }

    fn update_carry_flag(&mut self, val: bool) {
        if val {
            self.p |= CPUFlag::Carry as u8;
        } else {
            self.p &= !(CPUFlag::Carry as u8);
        }
    }

    fn update_irqb_flag(&mut self, val: bool) {
        if val {
            self.p |= CPUFlag::IRQB as u8;
        } else {
            self.p &= !(CPUFlag::IRQB as u8);
        }
    }

    fn branch(&mut self, flag: CPUFlag, val: bool) {
        self.temp8 = self.fetch();
        let f = flag as u8;
        if self.p & f == (if val { f } else { 0 }) {
            self.tcu += 1;
        } else {
            self.tcu = 0;
        }
    }

    pub fn cycle(&mut self) {
        debug!("CPU: {:x?}", self);

        match self.state {
            CPUState::Init(c) => match c {
                5 => {
                    self.pc = self.read(0xFFFC) as u16;
                    self.state = CPUState::Init(c + 1)
                }
                6 => {
                    self.pc = self.pc | ((self.read(0xFFFD) as u16) << 8);
                    self.state = CPUState::Run;
                }
                _ => self.state = CPUState::Init(c + 1),
            },
            CPUState::Run => {
                match (self.ir, self.tcu) {
                    // First step is always to fetch the next instruction
                    (_, 0) => {
                        if (self.p & (CPUFlag::IRQB as u8) == 0) && self.interrupt {
                            debug!("Interrupt!");
                            self.ir = (Instruction::BRK, AddressMode::Implied);
                            self.tcu += 1;
                        } else {
                            self.ir = decode(self.fetch());
                            debug!("DECODE: {:x?}", self.ir);
                            
                            if self.ir.0 == Instruction::NOP(1, 1) {
                                self.tcu = 0;
                            } else {
                                self.tcu += 1;
                            }
                        }
                    }

                    //
                    // ADC
                    //
                    ((Instruction::ADC, AddressMode::ImmediateAddressing), 1)
                    | ((Instruction::ADC, AddressMode::ZeroPage), 2)
                    | ((Instruction::ADC, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::ADC, AddressMode::Absolute), 3)
                    | ((Instruction::ADC, AddressMode::AbsoluteIndexedWithX), 3)
                    | ((Instruction::ADC, AddressMode::AbsoluteIndexedWithY), 3)
                    | ((Instruction::ADC, AddressMode::ZeroPageIndexedIndirect), 5)
                    | ((Instruction::ADC, AddressMode::ZeroPageIndirectIndexedWithY), 4)
                    | ((Instruction::ADC, AddressMode::ZeroPageIndirect), 4) => {
                        let op1 = self.a as u16;
                        let op2 = if self.ir.1 == AddressMode::ImmediateAddressing {
                            self.fetch() as u16
                        } else {
                            self.read(self.temp16) as u16
                        };

                        if self.p & (CPUFlag::Decimal as u8) == 0 {
                            let sum = op1
                                .wrapping_add(op2)
                                .wrapping_add((self.p & (CPUFlag::Carry as u8)) as u16);
                            self.a = sum as u8;
                            
                            self.update_zero_flag(self.a == 0);
                            self.update_negative_flag(self.a);
                            self.update_carry_flag(sum & 0x100 == 0x100);
                            self.update_overflow_flag(((sum ^ op1) & (sum ^ op2)) & 0x80 == 0x80);
                            self.tcu = 0;
                        } else {
                            let carry_in = (self.p & (CPUFlag::Carry as u8)) as u16;

                            let mut sum = (op1 & 0xf) + (op2 & 0xf) + carry_in;
                            if sum > 9 {
                                sum += 0x06;
                            }

                            sum += (op1 & 0xf0) + (op2 & 0xf0);
                            if (sum >> 4) > 9 {
                                sum += 0x60;
                            }

                            self.a = sum as u8;

                            self.update_overflow_flag(((sum ^ op1) & (sum ^ op2)) & 0x80 == 0x80);
                            self.update_carry_flag(sum & 0xFF00 != 0);
                            self.update_zero_flag(self.a == 0);
                            self.update_negative_flag(self.a);
                                                        
                            self.tcu += 1;
                        }
                    }
                    ((Instruction::ADC, AddressMode::ImmediateAddressing), 2)
                    | ((Instruction::ADC, AddressMode::ZeroPage), 3)
                    | ((Instruction::ADC, AddressMode::ZeroPageIndexedWithX), 4)
                    | ((Instruction::ADC, AddressMode::Absolute), 4)
                    | ((Instruction::ADC, AddressMode::AbsoluteIndexedWithX), 4)
                    | ((Instruction::ADC, AddressMode::AbsoluteIndexedWithY), 4)
                    | ((Instruction::ADC, AddressMode::ZeroPageIndexedIndirect), 6)
                    | ((Instruction::ADC, AddressMode::ZeroPageIndirectIndexedWithY), 5)
                    | ((Instruction::ADC, AddressMode::ZeroPageIndirect), 5) => {
                        if self.p & (CPUFlag::Decimal as u8) == 0 {
                            panic!("ADC can only take an extra cycle in decimal mode!");
                        } else {                            
                            self.tcu = 0;
                        }
                    }

                    //
                    // AND
                    //
                    ((Instruction::AND, AddressMode::ImmediateAddressing), 1)
                    | ((Instruction::AND, AddressMode::ZeroPage), 2)
                    | ((Instruction::AND, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::AND, AddressMode::Absolute), 3)
                    | ((Instruction::AND, AddressMode::AbsoluteIndexedWithX), 3)
                    | ((Instruction::AND, AddressMode::AbsoluteIndexedWithY), 3)
                    | ((Instruction::AND, AddressMode::ZeroPageIndexedIndirect), 5)
                    | ((Instruction::AND, AddressMode::ZeroPageIndirectIndexedWithY), 4)
                    | ((Instruction::AND, AddressMode::ZeroPageIndirect), 4) => {
                        self.a &= if self.ir.1 == AddressMode::ImmediateAddressing {
                            self.fetch()
                        } else {
                            self.read(self.temp16)
                        };

                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    //
                    // ASL
                    //
                    ((Instruction::ASL, AddressMode::Accumulator), 1) => {
                        self.update_carry_flag(self.a & 0x80 == 0x80);
                        self.a <<= 1;
                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }
                    ((Instruction::ASL, AddressMode::ZeroPage), 2)
                    | ((Instruction::ASL, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::ASL, AddressMode::Absolute), 3)
                    | ((Instruction::ASL, AddressMode::AbsoluteIndexedWithX), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::ASL, AddressMode::ZeroPage), 3)
                    | ((Instruction::ASL, AddressMode::ZeroPageIndexedWithX), 4)
                    | ((Instruction::ASL, AddressMode::Absolute), 4)
                    | ((Instruction::ASL, AddressMode::AbsoluteIndexedWithX), 4) => {
                        self.update_carry_flag(self.temp8 & 0x80 == 0x80);
                        self.temp8 <<= 1;
                        self.update_zero_flag(self.temp8 == 0);
                        self.update_negative_flag(self.temp8);
                        self.tcu += 1;
                    }
                    ((Instruction::ASL, AddressMode::AbsoluteIndexedWithX), 5) => {
                        self.tcu += 1;
                    }
                    ((Instruction::ASL, AddressMode::ZeroPage), 4)
                    | ((Instruction::ASL, AddressMode::ZeroPageIndexedWithX), 5)
                    | ((Instruction::ASL, AddressMode::Absolute), 5)
                    | ((Instruction::ASL, AddressMode::AbsoluteIndexedWithX), 6) => {
                        self.write(self.temp16, self.temp8);
                        self.tcu = 0;
                    }

                    //
                    // BBR / BBS
                    //
                    ((Instruction::BBS(_), AddressMode::ProgramCounterRelative), 1)
                    | ((Instruction::BBR(_), AddressMode::ProgramCounterRelative), 1) => {
                        self.temp16 = self.fetch() as u16;
                        self.tcu += 1;
                    }
                    ((Instruction::BBS(_), AddressMode::ProgramCounterRelative), 2)
                    | ((Instruction::BBR(_), AddressMode::ProgramCounterRelative), 2) => {
                        self.temp8 = self.fetch();
                        self.tcu += 1;
                    }
                    ((Instruction::BBS(_), AddressMode::ProgramCounterRelative), 3)
                    | ((Instruction::BBR(_), AddressMode::ProgramCounterRelative), 3) => {
                        self.temp16 = self.read(self.temp16) as u16;
                        self.tcu += 1;
                    }
                    ((Instruction::BBS(n), AddressMode::ProgramCounterRelative), 4) => {
                        if (self.temp16 >> n) & 1 == 1 {
                            let offset = self.temp8 as i8;
                            if offset >= 0 {
                                self.pc += offset as u16;
                            } else {
                                self.pc -= offset.abs() as u16;
                            }
                        }
                        self.tcu = 0;
                    }
                    ((Instruction::BBR(n), AddressMode::ProgramCounterRelative), 4) => {
                        if (self.temp16 >> n) & 1 == 0 {
                            let offset = self.temp8 as i8;
                            if offset >= 0 {
                                self.pc += offset as u16;
                            } else {
                                self.pc -= offset.abs() as u16;
                            }
                        }
                        self.tcu = 0;
                    }

                    //
                    // BCC
                    //
                    ((Instruction::BCC, AddressMode::ProgramCounterRelative), 1) => {
                        self.branch(CPUFlag::Carry, false);
                    }

                    //
                    // BCS r
                    //
                    ((Instruction::BCS, AddressMode::ProgramCounterRelative), 1) => {
                        self.branch(CPUFlag::Carry, true);
                    }

                    //
                    // BEQ r
                    //
                    ((Instruction::BEQ, AddressMode::ProgramCounterRelative), 1) => {
                        self.branch(CPUFlag::Zero, true);
                    }

                    //
                    // BIT
                    //
                    ((Instruction::BIT, AddressMode::ImmediateAddressing), 1)
                    | ((Instruction::BIT, AddressMode::ZeroPage), 2)
                    | ((Instruction::BIT, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::BIT, AddressMode::Absolute), 3)
                    | ((Instruction::BIT, AddressMode::AbsoluteIndexedWithX), 3) => {
                        let operand = if self.ir.1 == AddressMode::ImmediateAddressing {
                            self.fetch()
                        } else {
                            self.read(self.temp16)
                        };

                        let val = self.a & operand;
                        self.update_zero_flag(val == 0);
                        if self.ir.1 != AddressMode::ImmediateAddressing {
                            self.update_overflow_flag(operand & 0x40 == 0x40);
                            self.update_negative_flag(operand);
                        }
                        self.tcu = 0;
                    }

                    //
                    // BMI r
                    //
                    ((Instruction::BMI, AddressMode::ProgramCounterRelative), 1) => {
                        self.branch(CPUFlag::Negative, true);
                    }

                    //
                    // BNE r
                    //
                    ((Instruction::BNE, AddressMode::ProgramCounterRelative), 1) => {
                        self.branch(CPUFlag::Zero, false);
                    }

                    //
                    // BPL r
                    //
                    ((Instruction::BPL, AddressMode::ProgramCounterRelative), 1) => {
                        self.branch(CPUFlag::Negative, false);
                    }

                    //
                    // BRA
                    //
                    ((Instruction::BRA, AddressMode::ProgramCounterRelative), 1) => {
                        self.temp8 = self.fetch();
                        self.tcu += 1;
                    }

                    //
                    // BRK
                    //
                    ((Instruction::BRK, AddressMode::Implied), 1) => {
                        self.tcu += 1;
                    }
                    ((Instruction::BRK, AddressMode::Stack), 1) => {
                        self.p |= CPUFlag::BRK as u8;
                        self.fetch();
                        self.tcu += 1;
                    }
                    ((Instruction::BRK, _), 2) => {
                        self.stack_push((self.pc >> 8) as u8);
                        self.tcu += 1;
                    }
                    ((Instruction::BRK, _), 3) => {
                        self.stack_push((self.pc & 0xff) as u8);
                        self.tcu += 1;
                    }
                    ((Instruction::BRK, _), 4) => {
                        self.stack_push(self.p | (CPUFlag::BRK as u8) | (CPUFlag::User as u8));
                        self.tcu += 1;
                    }
                    ((Instruction::BRK, _), 5) => {
                        self.p |= CPUFlag::IRQB as u8;
                        self.p &= !(CPUFlag::Decimal as u8);
                        self.pc = self.read(0xFFFE) as u16;
                        self.tcu += 1;
                    }
                    ((Instruction::BRK, _), 6) => {
                        self.pc = self.pc | ((self.read(0xFFFF) as u16) << 8);
                        self.tcu = 0;
                    }

                    //
                    // BVC r
                    //
                    ((Instruction::BVC, AddressMode::ProgramCounterRelative), 1) => {
                        self.branch(CPUFlag::Overflow, false);
                    }

                    //
                    // BVS r
                    //
                    ((Instruction::BVS, AddressMode::ProgramCounterRelative), 1) => {
                        self.branch(CPUFlag::Overflow, true);
                    }

                    //
                    // CLC i
                    //
                    ((Instruction::CLC, AddressMode::Implied), 1) => {
                        self.update_carry_flag(false);
                        self.tcu = 0;
                    }

                    //
                    // CLD i
                    //
                    ((Instruction::CLD, AddressMode::Implied), 1) => {
                        self.update_decimal_flag(false);
                        self.tcu = 0;
                    }

                    //
                    // CLI i
                    //
                    ((Instruction::CLI, AddressMode::Implied), 1) => {
                        self.update_irqb_flag(false);
                        self.tcu = 0;
                    }

                    //
                    // CLV i
                    //
                    ((Instruction::CLV, AddressMode::Implied), 1) => {
                        self.update_overflow_flag(false);
                        self.tcu = 0;
                    }

                    //
                    // CMP
                    //
                    ((Instruction::CMP, AddressMode::ImmediateAddressing), 1)
                    | ((Instruction::CMP, AddressMode::ZeroPage), 2)
                    | ((Instruction::CMP, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::CMP, AddressMode::Absolute), 3)
                    | ((Instruction::CMP, AddressMode::AbsoluteIndexedWithX), 3)
                    | ((Instruction::CMP, AddressMode::AbsoluteIndexedWithY), 3)
                    | ((Instruction::CMP, AddressMode::ZeroPageIndexedIndirect), 5)
                    | ((Instruction::CMP, AddressMode::ZeroPageIndirectIndexedWithY), 4)
                    | ((Instruction::CMP, AddressMode::ZeroPageIndirect), 4) => {
                        self.temp8 = if self.ir.1 == AddressMode::ImmediateAddressing {
                            self.fetch()
                        } else {
                            self.read(self.temp16)
                        };

                        self.update_carry_flag(self.a >= self.temp8);
                        let val = self.a.wrapping_sub(self.temp8);
                        self.update_zero_flag(val == 0);
                        self.update_negative_flag(val);
                        self.tcu = 0;
                    }

                    //
                    // CPX
                    //
                    ((Instruction::CPX, AddressMode::ImmediateAddressing), 1)
                    | ((Instruction::CPX, AddressMode::ZeroPage), 2)
                    | ((Instruction::CPX, AddressMode::Absolute), 3) => {
                        self.temp8 = if self.ir.1 == AddressMode::ImmediateAddressing {
                            self.fetch()
                        } else {
                            self.read(self.temp16)
                        };

                        self.update_carry_flag(self.x >= self.temp8);
                        let val = self.x.wrapping_sub(self.temp8);
                        self.update_zero_flag(val == 0);
                        self.update_negative_flag(val);
                        self.tcu = 0;
                    }

                    //
                    // CPY
                    //
                    ((Instruction::CPY, AddressMode::ImmediateAddressing), 1)
                    | ((Instruction::CPY, AddressMode::ZeroPage), 2)
                    | ((Instruction::CPY, AddressMode::Absolute), 3) => {
                        self.temp8 = if self.ir.1 == AddressMode::ImmediateAddressing {
                            self.fetch()
                        } else {
                            self.read(self.temp16)
                        };

                        self.update_carry_flag(self.y >= self.temp8);
                        let val = self.y.wrapping_sub(self.temp8);
                        self.update_zero_flag(val == 0);
                        self.update_negative_flag(val);
                        self.tcu = 0;
                    }

                    //
                    // DEC
                    //
                    ((Instruction::DEC, AddressMode::Accumulator), 1) => {
                        self.a = self.a.wrapping_sub(1);
                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }
                    ((Instruction::DEC, AddressMode::ZeroPage), 2)
                    | ((Instruction::DEC, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::DEC, AddressMode::Absolute), 3)
                    | ((Instruction::DEC, AddressMode::AbsoluteIndexedWithX), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::DEC, AddressMode::ZeroPage), 3)
                    | ((Instruction::DEC, AddressMode::ZeroPageIndexedWithX), 4)
                    | ((Instruction::DEC, AddressMode::Absolute), 4)
                    | ((Instruction::DEC, AddressMode::AbsoluteIndexedWithX), 4) => {
                        self.temp8 = self.temp8.wrapping_sub(1);
                        self.update_zero_flag(self.temp8 == 0);
                        self.update_negative_flag(self.temp8);
                        self.tcu += 1;
                    }
                    ((Instruction::DEC, AddressMode::ZeroPage), 4)
                    | ((Instruction::DEC, AddressMode::ZeroPageIndexedWithX), 5)
                    | ((Instruction::DEC, AddressMode::Absolute), 5)
                    | ((Instruction::DEC, AddressMode::AbsoluteIndexedWithX), 5) => {
                        self.write(self.temp16, self.temp8);
                        self.tcu = 0;
                    }

                    //
                    // DEX i
                    //
                    ((Instruction::DEX, AddressMode::Implied), 1) => {
                        self.x = self.x.wrapping_sub(1);
                        self.update_zero_flag(self.x == 0);
                        self.update_negative_flag(self.x);
                        self.tcu = 0;
                    }

                    //
                    // DEY i
                    //
                    ((Instruction::DEY, AddressMode::Implied), 1) => {
                        self.y = self.y.wrapping_sub(1);
                        self.update_zero_flag(self.y == 0);
                        self.update_negative_flag(self.y);
                        self.tcu = 0;
                    }

                    //
                    // EOR
                    //
                    ((Instruction::EOR, AddressMode::ImmediateAddressing), 1)
                    | ((Instruction::EOR, AddressMode::ZeroPage), 2)
                    | ((Instruction::EOR, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::EOR, AddressMode::Absolute), 3)
                    | ((Instruction::EOR, AddressMode::AbsoluteIndexedWithX), 3)
                    | ((Instruction::EOR, AddressMode::AbsoluteIndexedWithY), 3)
                    | ((Instruction::EOR, AddressMode::ZeroPageIndexedIndirect), 5)
                    | ((Instruction::EOR, AddressMode::ZeroPageIndirectIndexedWithY), 4)
                    | ((Instruction::EOR, AddressMode::ZeroPageIndirect), 4) => {
                        self.a ^= if self.ir.1 == AddressMode::ImmediateAddressing {
                            self.fetch()
                        } else {
                            self.read(self.temp16)
                        };

                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    //
                    // INC
                    //
                    ((Instruction::INC, AddressMode::Accumulator), 1) => {
                        self.a = self.a.wrapping_add(1);
                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }
                    ((Instruction::INC, AddressMode::ZeroPage), 2)
                    | ((Instruction::INC, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::INC, AddressMode::Absolute), 3)
                    | ((Instruction::INC, AddressMode::AbsoluteIndexedWithX), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::INC, AddressMode::ZeroPage), 3)
                    | ((Instruction::INC, AddressMode::ZeroPageIndexedWithX), 4)
                    | ((Instruction::INC, AddressMode::Absolute), 4)
                    | ((Instruction::INC, AddressMode::AbsoluteIndexedWithX), 4) => {
                        self.temp8 = self.temp8.wrapping_add(1);
                        self.update_zero_flag(self.temp8 == 0);
                        self.update_negative_flag(self.temp8);
                        self.tcu += 1;
                    }
                    ((Instruction::INC, AddressMode::ZeroPage), 4)
                    | ((Instruction::INC, AddressMode::ZeroPageIndexedWithX), 5)
                    | ((Instruction::INC, AddressMode::Absolute), 5)
                    | ((Instruction::INC, AddressMode::AbsoluteIndexedWithX), 5) => {
                        self.write(self.temp16, self.temp8);
                        self.tcu = 0;
                    }

                    //
                    // INX i
                    //
                    ((Instruction::INX, AddressMode::Implied), 1) => {
                        self.x = self.x.wrapping_add(1);
                        self.update_zero_flag(self.x == 0);
                        self.update_negative_flag(self.x);
                        self.tcu = 0;
                    }

                    //
                    // INY i
                    //
                    ((Instruction::INY, AddressMode::Implied), 1) => {
                        self.y = self.y.wrapping_add(1);
                        self.update_zero_flag(self.y == 0);
                        self.update_negative_flag(self.y);
                        self.tcu = 0;
                    }

                    //
                    // JMP
                    //
                    ((Instruction::JMP, AddressMode::Absolute), 2) => {
                        self.pc = self.temp16 | ((self.fetch() as u16) << 8);
                        self.tcu = 0;
                    }
                    ((Instruction::JMP, AddressMode::AbsoluteIndirect), 3)
                    | ((Instruction::JMP, AddressMode::AbsoluteIndexedIndirect), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::JMP, AddressMode::AbsoluteIndexedIndirect), 4) => {
                        self.tcu += 1;
                    }
                    ((Instruction::JMP, AddressMode::AbsoluteIndirect), 4)
                    | ((Instruction::JMP, AddressMode::AbsoluteIndexedIndirect), 5) => {
                        self.pc = self.temp8 as u16;
                        self.pc |= (self.read(self.temp16 + 1) as u16) << 8;
                        self.tcu = 0;
                    }

                    //
                    // JSR a
                    //
                    ((Instruction::JSR, AddressMode::Absolute), 1) => {
                        self.temp16 = self.fetch() as u16;
                        self.tcu += 1;
                    }
                    ((Instruction::JSR, AddressMode::Absolute), 2) => {
                        self.stack_peek();
                        self.tcu += 1;
                    }
                    ((Instruction::JSR, AddressMode::Absolute), 3) => {
                        self.stack_push((self.pc >> 8) as u8);
                        self.tcu += 1;
                    }
                    ((Instruction::JSR, AddressMode::Absolute), 4) => {
                        self.stack_push((self.pc & 0xFF) as u8);
                        self.tcu += 1;
                    }
                    ((Instruction::JSR, AddressMode::Absolute), 5) => {
                        self.temp16 = self.temp16 | ((self.fetch() as u16) << 8);
                        self.pc = self.temp16;
                        self.tcu = 0;
                    }

                    //
                    // LDA
                    //
                    ((Instruction::LDA, AddressMode::ImmediateAddressing), 1)
                    | ((Instruction::LDA, AddressMode::ZeroPage), 2)
                    | ((Instruction::LDA, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::LDA, AddressMode::Absolute), 3)
                    | ((Instruction::LDA, AddressMode::AbsoluteIndexedWithX), 3)
                    | ((Instruction::LDA, AddressMode::AbsoluteIndexedWithY), 3)
                    | ((Instruction::LDA, AddressMode::ZeroPageIndexedIndirect), 5)
                    | ((Instruction::LDA, AddressMode::ZeroPageIndirectIndexedWithY), 4)
                    | ((Instruction::LDA, AddressMode::ZeroPageIndirect), 4) => {
                        self.a = if self.ir.1 == AddressMode::ImmediateAddressing {
                            self.fetch()
                        } else {
                            self.read(self.temp16)
                        };

                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    //
                    // LDX
                    //
                    ((Instruction::LDX, AddressMode::ImmediateAddressing), 1)
                    | ((Instruction::LDX, AddressMode::ZeroPage), 2)
                    | ((Instruction::LDX, AddressMode::ZeroPageIndexedWithY), 3)
                    | ((Instruction::LDX, AddressMode::Absolute), 3)
                    | ((Instruction::LDX, AddressMode::AbsoluteIndexedWithY), 3) => {
                        self.x = if self.ir.1 == AddressMode::ImmediateAddressing {
                            self.fetch()
                        } else {
                            self.read(self.temp16)
                        };

                        self.update_zero_flag(self.x == 0);
                        self.update_negative_flag(self.x);
                        self.tcu = 0;
                    }

                    //
                    // LDY
                    //
                    ((Instruction::LDY, AddressMode::ImmediateAddressing), 1)
                    | ((Instruction::LDY, AddressMode::ZeroPage), 2)
                    | ((Instruction::LDY, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::LDY, AddressMode::Absolute), 3)
                    | ((Instruction::LDY, AddressMode::AbsoluteIndexedWithX), 3) => {
                        self.y = if self.ir.1 == AddressMode::ImmediateAddressing {
                            self.fetch()
                        } else {
                            self.read(self.temp16)
                        };

                        self.update_zero_flag(self.y == 0);
                        self.update_negative_flag(self.y);
                        self.tcu = 0;
                    }

                    //
                    // LSR
                    //
                    ((Instruction::LSR, AddressMode::Accumulator), 1) => {
                        self.update_carry_flag(self.a & 0x01 == 0x01);
                        self.a >>= 1;
                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }
                    ((Instruction::LSR, AddressMode::ZeroPage), 2)
                    | ((Instruction::LSR, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::LSR, AddressMode::Absolute), 3)
                    | ((Instruction::LSR, AddressMode::AbsoluteIndexedWithX), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::LSR, AddressMode::ZeroPage), 3)
                    | ((Instruction::LSR, AddressMode::ZeroPageIndexedWithX), 4)
                    | ((Instruction::LSR, AddressMode::Absolute), 4)
                    | ((Instruction::LSR, AddressMode::AbsoluteIndexedWithX), 4) => {
                        self.update_carry_flag(self.temp8 & 0x01 == 0x01);
                        self.temp8 >>= 1;
                        self.update_zero_flag(self.temp8 == 0);
                        self.update_negative_flag(self.temp8);
                        self.tcu += 1;
                    }
                    ((Instruction::LSR, AddressMode::AbsoluteIndexedWithX), 5) => {
                        self.tcu += 1;
                    }
                    ((Instruction::LSR, AddressMode::ZeroPage), 4)
                    | ((Instruction::LSR, AddressMode::ZeroPageIndexedWithX), 5)
                    | ((Instruction::LSR, AddressMode::Absolute), 5)
                    | ((Instruction::LSR, AddressMode::AbsoluteIndexedWithX), 6) => {
                        self.write(self.temp16, self.temp8);
                        self.tcu = 0;
                    }

                    //
                    // NOP i
                    //
                    ((Instruction::NOP(bytes, cycles), AddressMode::Implied), tcu) => {
                        if tcu < bytes {
                            self.fetch();
                        }
                        self.tcu += 1;
                        if tcu == cycles {
                            self.tcu = 0;
                        }
                    }

                    //
                    // ORA
                    //
                    ((Instruction::ORA, AddressMode::ImmediateAddressing), 1)
                    | ((Instruction::ORA, AddressMode::ZeroPage), 2)
                    | ((Instruction::ORA, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::ORA, AddressMode::Absolute), 3)
                    | ((Instruction::ORA, AddressMode::AbsoluteIndexedWithX), 3)
                    | ((Instruction::ORA, AddressMode::AbsoluteIndexedWithY), 3)
                    | ((Instruction::ORA, AddressMode::ZeroPageIndexedIndirect), 5)
                    | ((Instruction::ORA, AddressMode::ZeroPageIndirectIndexedWithY), 4)
                    | ((Instruction::ORA, AddressMode::ZeroPageIndirect), 4) => {
                        self.a |= if self.ir.1 == AddressMode::ImmediateAddressing {
                            self.fetch()
                        } else {
                            self.read(self.temp16)
                        };

                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    //
                    // PHA s
                    //
                    ((Instruction::PHA, AddressMode::Stack), 1) => {
                        self.stack_push(self.a);
                        self.tcu += 1;
                    }
                    ((Instruction::PHA, AddressMode::Stack), 2) => {
                        self.tcu = 0;
                    }

                    //
                    // PHP s
                    //
                    ((Instruction::PHP, AddressMode::Stack), 1) => {
                        self.stack_push(self.p | (CPUFlag::BRK as u8) | (CPUFlag::User as u8));
                        self.tcu += 1;
                    }
                    ((Instruction::PHP, AddressMode::Stack), 2) => {
                        self.tcu = 0;
                    }

                    //
                    // PHX s
                    //
                    ((Instruction::PHX, AddressMode::Stack), 1) => {
                        self.stack_push(self.x);
                        self.tcu += 1;
                    }
                    ((Instruction::PHX, AddressMode::Stack), 2) => {
                        self.tcu = 0;
                    }

                    //
                    // PHY s
                    //
                    ((Instruction::PHY, AddressMode::Stack), 1) => {
                        self.stack_push(self.y);
                        self.tcu += 1;
                    }
                    ((Instruction::PHY, AddressMode::Stack), 2) => {
                        self.tcu = 0;
                    }

                    //
                    // PLA s
                    //
                    ((Instruction::PLA, AddressMode::Stack), 1) => {
                        self.a = self.stack_pop();
                        self.tcu += 1;
                    }
                    ((Instruction::PLA, AddressMode::Stack), 2) => {
                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu += 1;
                    }
                    ((Instruction::PLA, AddressMode::Stack), 3) => {
                        self.tcu = 0;
                    }

                    //
                    // PLP s
                    //
                    ((Instruction::PLP, AddressMode::Stack), 1) => {
                        self.p = self.stack_pop();
                        self.tcu += 1;
                    }
                    ((Instruction::PLP, AddressMode::Stack), 2) => {
                        self.tcu += 1;
                    }
                    ((Instruction::PLP, AddressMode::Stack), 3) => {
                        self.tcu = 0;
                    }

                    //
                    // PLX s
                    //
                    ((Instruction::PLX, AddressMode::Stack), 1) => {
                        self.x = self.stack_pop();
                        self.tcu += 1;
                    }
                    ((Instruction::PLX, AddressMode::Stack), 2) => {
                        self.update_zero_flag(self.x == 0);
                        self.update_negative_flag(self.x);
                        self.tcu += 1;
                    }
                    ((Instruction::PLX, AddressMode::Stack), 3) => {
                        self.tcu = 0;
                    }

                    //
                    // PLY s
                    //
                    ((Instruction::PLY, AddressMode::Stack), 1) => {
                        self.y = self.stack_pop();
                        self.tcu += 1;
                    }
                    ((Instruction::PLY, AddressMode::Stack), 2) => {
                        self.update_zero_flag(self.y == 0);
                        self.update_negative_flag(self.y);
                        self.tcu += 1;
                    }
                    ((Instruction::PLY, AddressMode::Stack), 3) => {
                        self.tcu = 0;
                    }

                    //
                    // RMB
                    //
                    ((Instruction::RMB(_), AddressMode::ZeroPage), 2) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::RMB(n), AddressMode::ZeroPage), 3) => {
                        self.temp8 &= !(1u8 << n);
                        self.tcu += 1;
                    }
                    ((Instruction::RMB(_), AddressMode::ZeroPage), 4) => {
                        self.write(self.temp16, self.temp8);
                        self.tcu = 0;
                    }

                    //
                    // ROL
                    //
                    ((Instruction::ROL, AddressMode::Accumulator), 1) => {
                        let c = self.p & 1;
                        self.update_carry_flag(self.a & 0x80 == 0x80);
                        self.a = (self.a << 1) | c;
                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }
                    ((Instruction::ROL, AddressMode::ZeroPage), 2)
                    | ((Instruction::ROL, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::ROL, AddressMode::Absolute), 3)
                    | ((Instruction::ROL, AddressMode::AbsoluteIndexedWithX), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::ROL, AddressMode::ZeroPage), 3)
                    | ((Instruction::ROL, AddressMode::ZeroPageIndexedWithX), 4)
                    | ((Instruction::ROL, AddressMode::Absolute), 4)
                    | ((Instruction::ROL, AddressMode::AbsoluteIndexedWithX), 4) => {
                        let c = self.p & 1;
                        self.update_carry_flag(self.temp8 & 0x80 == 0x80);
                        self.temp8 = (self.temp8 << 1) | c;
                        self.update_zero_flag(self.temp8 == 0);
                        self.update_negative_flag(self.temp8);
                        self.tcu += 1;
                    }
                    ((Instruction::ROL, AddressMode::AbsoluteIndexedWithX), 5) => {
                        self.tcu += 1;
                    }
                    ((Instruction::ROL, AddressMode::ZeroPage), 4)
                    | ((Instruction::ROL, AddressMode::ZeroPageIndexedWithX), 5)
                    | ((Instruction::ROL, AddressMode::Absolute), 5)
                    | ((Instruction::ROL, AddressMode::AbsoluteIndexedWithX), 6) => {
                        self.write(self.temp16, self.temp8);
                        self.tcu = 0;
                    }

                    //
                    // ROR
                    //
                    ((Instruction::ROR, AddressMode::Accumulator), 1) => {
                        let c = self.p & 1;
                        self.update_carry_flag(self.a & 0x01 == 0x01);
                        self.a = (self.a >> 1) | (c << 7);
                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }
                    ((Instruction::ROR, AddressMode::ZeroPage), 2)
                    | ((Instruction::ROR, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::ROR, AddressMode::Absolute), 3)
                    | ((Instruction::ROR, AddressMode::AbsoluteIndexedWithX), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::ROR, AddressMode::ZeroPage), 3)
                    | ((Instruction::ROR, AddressMode::ZeroPageIndexedWithX), 4)
                    | ((Instruction::ROR, AddressMode::Absolute), 4)
                    | ((Instruction::ROR, AddressMode::AbsoluteIndexedWithX), 4) => {
                        let c = self.p & 1;
                        self.update_carry_flag(self.temp8 & 0x01 == 0x01);
                        self.temp8 = (self.temp8 >> 1) | (c << 7);
                        self.update_zero_flag(self.temp8 == 0);
                        self.update_negative_flag(self.temp8);
                        self.tcu += 1;
                    }
                    ((Instruction::ROR, AddressMode::AbsoluteIndexedWithX), 5) => {
                        self.tcu += 1;
                    }
                    ((Instruction::ROR, AddressMode::ZeroPage), 4)
                    | ((Instruction::ROR, AddressMode::ZeroPageIndexedWithX), 5)
                    | ((Instruction::ROR, AddressMode::Absolute), 5)
                    | ((Instruction::ROR, AddressMode::AbsoluteIndexedWithX), 6) => {
                        self.write(self.temp16, self.temp8);
                        self.tcu = 0;
                    }

                    //
                    // RTI s
                    //
                    ((Instruction::RTI, AddressMode::Stack), 1) => {
                        self.p = self.stack_pop();
                        self.tcu += 1;
                    }
                    ((Instruction::RTI, AddressMode::Stack), 2) => {
                        self.tcu += 1;
                    }
                    ((Instruction::RTI, AddressMode::Stack), 3) => {
                        self.pc = self.stack_pop() as u16;
                        self.tcu += 1;
                    }
                    ((Instruction::RTI, AddressMode::Stack), 4) => {
                        self.tcu += 1;
                    }
                    ((Instruction::RTI, AddressMode::Stack), 5) => {
                        self.pc |= (self.stack_pop() as u16) << 8;
                        self.tcu = 0;
                    }

                    //
                    // RTS s
                    //
                    ((Instruction::RTS, AddressMode::Stack), 1) => {
                        self.fetch();
                        self.tcu += 1;
                    }
                    ((Instruction::RTS, AddressMode::Stack), 2) => {
                        self.stack_peek();
                        self.tcu += 1;
                    }
                    ((Instruction::RTS, AddressMode::Stack), 3) => {
                        self.temp16 = self.stack_pop() as u16;
                        self.tcu += 1;
                    }
                    ((Instruction::RTS, AddressMode::Stack), 4) => {
                        self.temp16 |= (self.stack_pop() as u16) << 8;
                        self.tcu += 1;
                    }
                    ((Instruction::RTS, AddressMode::Stack), 5) => {
                        self.pc = self.temp16;
                        self.fetch();
                        self.tcu = 0;
                    }

                    //
                    // SBC
                    //
                    ((Instruction::SBC, AddressMode::ImmediateAddressing), 1)
                    | ((Instruction::SBC, AddressMode::ZeroPage), 2)
                    | ((Instruction::SBC, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::SBC, AddressMode::Absolute), 3)
                    | ((Instruction::SBC, AddressMode::AbsoluteIndexedWithX), 3)
                    | ((Instruction::SBC, AddressMode::AbsoluteIndexedWithY), 3)
                    | ((Instruction::SBC, AddressMode::ZeroPageIndexedIndirect), 5)
                    | ((Instruction::SBC, AddressMode::ZeroPageIndirectIndexedWithY), 4)
                    | ((Instruction::SBC, AddressMode::ZeroPageIndirect), 4) => {
                        let op1 = self.a as u16;
                        

                        if self.p & (CPUFlag::Decimal as u8) == 0 {
                            let op2 = if self.ir.1 == AddressMode::ImmediateAddressing {
                                !self.fetch() as u16
                            } else {
                                !self.read(self.temp16) as u16
                            };
                            
                            let sum = op1
                                .wrapping_add(op2)
                                .wrapping_add((self.p & (CPUFlag::Carry as u8)) as u16);
                            self.a = sum as u8;

                            self.update_zero_flag(self.a == 0);
                            self.update_negative_flag(self.a);
                            self.update_carry_flag(sum & 0x100 == 0x100);
                            self.update_overflow_flag(((sum ^ op1) & (sum ^ op2)) & 0x80 == 0x80);

                            self.tcu = 0;
                        } else {
                            let op2 = if self.ir.1 == AddressMode::ImmediateAddressing {
                                self.fetch() as u16
                            } else {
                                self.read(self.temp16) as u16
                            };
                            
                            // println!("{:04x?} - {:04x?} = ??", op1, op2);
                            // println!("flags: {:02x?}", self.p);


                            let mut nines_complement = 0x99u16.wrapping_sub(op2);
                            if nines_complement & 0x0f > 9 {
                                nines_complement = nines_complement.wrapping_add(0x06);
                            }
                            if (nines_complement >> 4) > 9 {
                                nines_complement = nines_complement.wrapping_add(0x60);
                            }
                            // println!("nines complement of 0x{:02x?} is 0x{:04x?}", op2, nines_complement);

                            // println!("flags: {:02x?}", self.p);

                            let carry_in = if (self.p & (CPUFlag::Carry as u8)) == 0 {
                                0x00
                            } else {
                                0x01
                            };

                            // println!("{:04x?} + {:04x?} + {:04x?} = ??", op1, nines_complement, carry_in);

                            let mut sum = (op1 & 0x0f).wrapping_add(nines_complement & 0x0f).wrapping_add(carry_in);

                            // println!("sum is 0x{:04x}", sum);

                            if sum > 9 {
                                sum = sum.wrapping_add(0x06);
                            }

                            sum = sum.wrapping_add(op1 & 0xf0).wrapping_add(nines_complement & 0xf0);
                            if (sum >> 4) > 9 {
                                sum = sum.wrapping_add(0x60);
                            }

                            // println!("sum is 0x{:04x}", sum);

                            self.a = (sum & 0xff) as u8;

                            // println!("a is 0x{:02x}", self.a);

                            self.update_overflow_flag(((sum ^ op1) & (sum ^ nines_complement)) & 0x80 == 0x80);
                            self.update_carry_flag(sum & 0xFF00 != 0);
                            self.update_zero_flag(self.a == 0);
                            self.update_negative_flag(self.a);
                                                        
                            self.tcu += 1;
                        }
                    }
                    ((Instruction::SBC, AddressMode::ImmediateAddressing), 2)
                    | ((Instruction::SBC, AddressMode::ZeroPage), 3)
                    | ((Instruction::SBC, AddressMode::ZeroPageIndexedWithX), 4)
                    | ((Instruction::SBC, AddressMode::Absolute), 4)
                    | ((Instruction::SBC, AddressMode::AbsoluteIndexedWithX), 4)
                    | ((Instruction::SBC, AddressMode::AbsoluteIndexedWithY), 4)
                    | ((Instruction::SBC, AddressMode::ZeroPageIndexedIndirect), 6)
                    | ((Instruction::SBC, AddressMode::ZeroPageIndirectIndexedWithY), 5)
                    | ((Instruction::SBC, AddressMode::ZeroPageIndirect), 5) => {
                        if self.p & (CPUFlag::Decimal as u8) == 0 {
                            panic!("SBC can only take an extra cycle in decimal mode!");
                        } else {                            
                            self.tcu = 0;
                        }
                    }

                    //
                    // SEC i
                    //
                    ((Instruction::SEC, AddressMode::Implied), 1) => {
                        self.update_carry_flag(true);
                        self.tcu = 0;
                    }

                    //
                    // SED i
                    //
                    ((Instruction::SED, AddressMode::Implied), 1) => {
                        self.update_decimal_flag(true);
                        self.tcu = 0;
                    }

                    //
                    // SEI i
                    //
                    ((Instruction::SEI, AddressMode::Implied), 1) => {
                        self.update_irqb_flag(true);
                        self.tcu = 0;
                    }

                    //
                    // SMB
                    //
                    ((Instruction::SMB(_), AddressMode::ZeroPage), 2) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::SMB(n), AddressMode::ZeroPage), 3) => {
                        self.temp8 |= 1u8 << n;
                        self.tcu += 1;
                    }
                    ((Instruction::SMB(_), AddressMode::ZeroPage), 4) => {
                        self.write(self.temp16, self.temp8);
                        self.tcu = 0;
                    }

                    //
                    // STA
                    //
                    ((Instruction::STA, AddressMode::AbsoluteIndexedWithX), 3)
                    | ((Instruction::STA, AddressMode::AbsoluteIndexedWithY), 3)
                    | ((Instruction::STA, AddressMode::ZeroPageIndirectIndexedWithY), 4) => {
                        self.tcu += 1;
                    }
                    ((Instruction::STA, AddressMode::ZeroPage), 2)
                    | ((Instruction::STA, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::STA, AddressMode::Absolute), 3)
                    | ((Instruction::STA, AddressMode::AbsoluteIndexedWithX), 4)
                    | ((Instruction::STA, AddressMode::AbsoluteIndexedWithY), 4)
                    | ((Instruction::STA, AddressMode::ZeroPageIndexedIndirect), 5)
                    | ((Instruction::STA, AddressMode::ZeroPageIndirectIndexedWithY), 5)
                    | ((Instruction::STA, AddressMode::ZeroPageIndirect), 4) => {
                        self.write(self.temp16, self.a);
                        self.tcu = 0;
                    }

                    //
                    // STP
                    //
                    ((Instruction::STP, AddressMode::Implied), 1) => {
                        self.tcu += 1;
                    }
                    ((Instruction::STP, AddressMode::Implied), 2) => {
                        self.state = CPUState::Halt;
                    }

                    //
                    // STX
                    //
                    ((Instruction::STX, AddressMode::ZeroPage), 2)
                    | ((Instruction::STX, AddressMode::ZeroPageIndexedWithY), 3)
                    | ((Instruction::STX, AddressMode::Absolute), 3) => {
                        self.write(self.temp16, self.x);
                        self.tcu = 0;
                    }

                    //
                    // STY
                    //
                    ((Instruction::STY, AddressMode::ZeroPage), 2)
                    | ((Instruction::STY, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::STY, AddressMode::Absolute), 3) => {
                        self.write(self.temp16, self.y);
                        self.tcu = 0;
                    }

                    //
                    // STZ
                    //
                    ((Instruction::STZ, AddressMode::ZeroPage), 2)
                    | ((Instruction::STZ, AddressMode::ZeroPageIndexedWithX), 3)
                    | ((Instruction::STZ, AddressMode::Absolute), 3)
                    | ((Instruction::STZ, AddressMode::AbsoluteIndexedWithX), 3) => {
                        self.write(self.temp16, 0);
                        self.tcu = 0;
                    }

                    // TAX i
                    ((Instruction::TAX, AddressMode::Implied), 1) => {
                        self.x = self.a;
                        self.update_zero_flag(self.x == 0);
                        self.update_negative_flag(self.x);
                        self.tcu = 0;
                    }

                    // TAY i
                    ((Instruction::TAY, AddressMode::Implied), 1) => {
                        self.y = self.a;
                        self.update_zero_flag(self.y == 0);
                        self.update_negative_flag(self.y);
                        self.tcu = 0;
                    }

                    //
                    // TRB
                    //
                    ((Instruction::TRB, AddressMode::ZeroPage), 2)
                    | ((Instruction::TRB, AddressMode::Absolute), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::TRB, AddressMode::ZeroPage), 3)
                    | ((Instruction::TRB, AddressMode::Absolute), 4) => {
                        self.update_zero_flag(self.temp8 & self.a == 0);
                        self.temp8 &= !self.a;
                        self.tcu += 1;
                    }
                    ((Instruction::TRB, AddressMode::ZeroPage), 4)
                    | ((Instruction::TRB, AddressMode::Absolute), 5) => {
                        self.write(self.temp16, self.temp8);
                        self.tcu = 0;
                    }

                    //
                    // TSB
                    //
                    ((Instruction::TSB, AddressMode::ZeroPage), 2)
                    | ((Instruction::TSB, AddressMode::Absolute), 3) => {
                        self.temp8 = self.read(self.temp16);
                        self.tcu += 1;
                    }
                    ((Instruction::TSB, AddressMode::ZeroPage), 3)
                    | ((Instruction::TSB, AddressMode::Absolute), 4) => {
                        self.update_zero_flag(self.temp8 & self.a == 0);
                        self.temp8 |= self.a;
                        self.tcu += 1;
                    }
                    ((Instruction::TSB, AddressMode::ZeroPage), 4)
                    | ((Instruction::TSB, AddressMode::Absolute), 5) => {
                        self.write(self.temp16, self.temp8);
                        self.tcu = 0;
                    }

                    //
                    // TSX i
                    //
                    ((Instruction::TSX, AddressMode::Implied), 1) => {
                        self.x = self.s;
                        self.update_zero_flag(self.x == 0);
                        self.update_negative_flag(self.x);
                        self.tcu = 0;
                    }

                    //
                    // TXA i
                    //
                    ((Instruction::TXA, AddressMode::Implied), 1) => {
                        self.a = self.x;
                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    //
                    // TXS i
                    //
                    ((Instruction::TXS, AddressMode::Implied), 1) => {
                        self.s = self.x;
                        self.tcu = 0;
                    }

                    //
                    // TYA i
                    //
                    ((Instruction::TYA, AddressMode::Implied), 1) => {
                        self.a = self.y;
                        self.update_zero_flag(self.a == 0);
                        self.update_negative_flag(self.a);
                        self.tcu = 0;
                    }

                    //
                    // WAI
                    //
                    ((Instruction::WAI, AddressMode::Implied), 1) => {
                        self.tcu += 1;
                    }
                    ((Instruction::WAI, AddressMode::Implied), 2) => {
                        self.p |= CPUFlag::BRK as u8;
                        self.state = CPUState::Wait;
                        self.tcu = 0;
                    }

                    //
                    // Defaults based on Address Mode
                    //

                    // Fetch temp16 low
                    ((_, AddressMode::Absolute), 1)
                    | ((_, AddressMode::AbsoluteIndexedIndirect), 1)
                    | ((_, AddressMode::AbsoluteIndexedWithX), 1)
                    | ((_, AddressMode::AbsoluteIndexedWithY), 1)
                    | ((_, AddressMode::AbsoluteIndirect), 1)
                    | ((_, AddressMode::ZeroPage), 1)
                    | ((_, AddressMode::ZeroPageIndexedWithX), 1)
                    | ((_, AddressMode::ZeroPageIndexedWithY), 1) => {
                        self.temp16 = self.fetch() as u16;
                        self.tcu += 1;
                    }

                    // Fetch temp16 high
                    ((_, AddressMode::Absolute), 2) | ((_, AddressMode::AbsoluteIndirect), 2) => {
                        self.temp16 = self.temp16 | ((self.fetch() as u16) << 8);
                        self.tcu += 1;
                    }

                    // Fetch temp16 high + x
                    ((_, AddressMode::AbsoluteIndexedWithX), 2)
                    | ((_, AddressMode::AbsoluteIndexedIndirect), 2) => {
                        self.temp16 = self.temp16 | ((self.fetch() as u16) << 8);
                        self.temp16 += self.x as u16;
                        self.tcu += 1;
                    }

                    // Fetch temp16 high + y
                    ((_, AddressMode::AbsoluteIndexedWithY), 2) => {
                        self.temp16 = self.temp16 | ((self.fetch() as u16) << 8);
                        self.temp16 += self.y as u16;
                        self.tcu += 1;
                    }

                    // Offset PC
                    ((_, AddressMode::ProgramCounterRelative), 2) => {
                        let offset = self.temp8 as i8;
                        if offset >= 0 {
                            self.pc += offset as u16;
                        } else {
                            self.pc -= (offset as i16).abs() as u16;
                        }
                        self.tcu = 0;
                    }

                    // Offset by x
                    ((_, AddressMode::ZeroPageIndexedWithX), 2) => {
                        self.temp16 = (self.temp16 + (self.x as u16)) % 0x100;
                        self.tcu += 1;
                    }

                    // Offset by y
                    ((_, AddressMode::ZeroPageIndexedWithY), 2) => {
                        self.temp16 = (self.temp16 + (self.y as u16)) % 0x100;
                        self.tcu += 1;
                    }

                    // Fetch temp8
                    ((_, AddressMode::ZeroPageIndexedIndirect), 1)
                    | ((_, AddressMode::ZeroPageIndirect), 1)
                    | ((_, AddressMode::ZeroPageIndirectIndexedWithY), 1) => {
                        self.temp8 = self.fetch();
                        self.tcu += 1;
                    }

                    // Offset temp8 by x
                    ((_, AddressMode::ZeroPageIndexedIndirect), 2) => {
                        self.temp8 = self.temp8.wrapping_add(self.x);
                        self.tcu += 1;
                    }

                    // Read temp16 low
                    ((_, AddressMode::ZeroPageIndexedIndirect), 3)
                    | ((_, AddressMode::ZeroPageIndirect), 2)
                    | ((_, AddressMode::ZeroPageIndirectIndexedWithY), 2) => {
                        self.temp16 = self.read(self.temp8 as u16) as u16;
                        self.tcu += 1;
                    }

                    // Read temp16 high
                    ((_, AddressMode::ZeroPageIndexedIndirect), 4)
                    | ((_, AddressMode::ZeroPageIndirect), 3) => {
                        self.temp16 |= (self.read((self.temp8 + 1) as u16) as u16) << 8;
                        self.tcu += 1;
                    }

                    // Read temp16 high; offset by y
                    ((_, AddressMode::ZeroPageIndirectIndexedWithY), 3) => {
                        self.temp16 |= (self.read((self.temp8 + 1) as u16) as u16) << 8;
                        self.temp16 += self.y as u16;
                        self.tcu += 1;
                    }

                    // Unimplemented
                    _ => {
                        self.state = CPUState::Halt;
                        info!("CPU: {:x?}", self);
                        unimplemented!("Unimplemented opcode: PC={:?}, IR={:?}, TCU={:?}", self.pc, self.ir, self.tcu);
                    }
                }
            }
            CPUState::Wait => {
                if (self.p & (CPUFlag::IRQB as u8) == 0) && self.interrupt {
                    debug!("Interrupt!");
                    self.ir = (Instruction::BRK, AddressMode::Implied);
                    self.tcu = 1;
                    self.state = CPUState::Run;
                }
            }
            CPUState::Halt => {}
        }
    }

    pub fn set_interrupt(&mut self, val: bool) {
        self.interrupt = val;
    }
}
