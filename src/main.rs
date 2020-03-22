use log::debug;

pub struct Memory {
    pub mem : [u8; 65536]
}

impl Memory {
    pub fn new() -> Memory {
        Memory { mem: [0; 65536] }
    }

    pub fn read(&self, addr : u16) -> u8 {
        let data = self.mem[addr as usize];
        debug!("MEM READ @ {:x} = {:x}", addr, data);
        data
    }

    pub fn write(&mut self, addr : u16, data : u8) {
        debug!("MEM WRITE @ {:x} = {:x}", addr, data);
        self.mem[addr as usize] = data;
    }
}


#[derive(Debug)]
pub enum CPUState {
    Init { cycle : u8 },
    Fetch,
    Execute
}

#[derive(Debug)]
pub struct W65C02S {
    pub state : CPUState,
    pub ir    : u8,   // instruction register
    pub tcu   : u8,   // timing control unit
    pub a     : u8,   // accumulator register
    pub x     : u8,   // index register 'x'
    pub y     : u8,   // index register 'y'
    pub pc    : u16,  // program counter register
    pub s     : u8,   // stack pointer register
}

impl W65C02S {
    pub fn new() -> W65C02S {
        W65C02S {
            state: CPUState::Init { cycle: 0 },
            ir: 0,
            tcu: 0,
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0,
        }
    }

    pub fn step(&mut self, bus : &mut Memory) {
        debug!("step: {:x?}", self);
        match self.state {
            CPUState::Init { cycle: c } => {
                match c {
                    5 => {
                        self.pc = bus.read(0xFFFE) as u16;
                        self.state = CPUState::Init { cycle: c+1 }
                    }
                    6 => {
                        self.pc = self.pc | ((bus.read(0xFFFF) as u16) << 8);
                        self.state = CPUState::Fetch;
                    }
                    _ => {
                        self.state = CPUState::Init { cycle: c+1 }
                    }
                }
            }
            CPUState::Fetch => {
                self.ir = bus.read(self.pc);
                self.pc = self.pc + 1;
            }
            CPUState::Execute => {
            }
        }
    }
}

fn main() {
    env_logger::init();

    let mut m = Memory::new();
    m.write(0xFFFE, 0xAA);
    m.write(0xFFFF, 0xBB);
    m.write(0xBBAA, 0xFF);

    let mut cpu = W65C02S::new();
    for i in 0..10 {
        cpu.step(&mut m);
    }
}
