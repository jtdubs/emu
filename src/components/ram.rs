use log::debug;

pub struct RAM {
    pub mem: Vec<u8>,
}

impl RAM {
    pub fn new(size: usize) -> RAM {
        let mut storage = Vec::new();
        storage.resize(size, 0u8);
        RAM { mem: storage }
    }

    pub fn peek(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn read(&self, addr: u16) -> u8 {
        let data = self.mem[addr as usize];
        debug!("R @ {:04x} = {:02x}", addr, data);
        data
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        debug!("W @ {:04x} = {:02x}", addr, data);
        self.mem[addr as usize] = data;
    }
}
