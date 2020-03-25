use crate::components::periph;

#[derive(Debug)]
pub struct SNESController {}

impl SNESController {
    pub fn new() -> SNESController {
        SNESController {}
    }
}

impl periph::Attachment for SNESController {
    fn peek(&self, _p: periph::Port) -> u8 {
        // 0xff
        0x00
    }

    fn read(&mut self, _p: periph::Port) -> u8 {
        // 0xff
        0x00
    }

    fn write(&mut self, _p: periph::Port, _val: u8) {}
}
