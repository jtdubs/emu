use crate::components::periph;

#[derive(Debug)]
pub struct SNESController {
}

impl SNESController {
    pub fn new() -> SNESController {
        SNESController { }
    }
}

impl periph::Attachment for SNESController {
    fn read(&mut self, _p: periph::Port) -> u8 {
        0xff
    }

    fn write(&mut self, _p: periph::Port, _val: u8) {
    }
}
