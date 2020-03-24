use log::debug;

use crate::components::cpu;

pub struct RAM {
    mem: Vec<u8>,
}

impl RAM {
    pub fn new(size: usize) -> RAM {
        let mut storage = Vec::new();
        storage.resize(size, 0u8);
        RAM { mem: storage }
    }
}

impl cpu::Attachment for RAM {
    fn read(&mut self, addr: u16) -> u8 {
        let data = self.mem[addr as usize];
        debug!("R @ {:04x} = {:02x}", addr, data);
        data
    }

    fn write(&mut self, addr: u16, data: u8) {
        debug!("W @ {:04x} = {:02x}", addr, data);
        self.mem[addr as usize] = data;
    }

    fn has_interrupt(&self) -> bool {
        false
    }
}
