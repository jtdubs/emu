use log::debug;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Button {
    A = 0,
    B = 1,
    Select = 2,
    Start = 3,
    Up = 4,
    Down = 5,
    Left = 6,
    Right = 7,
}

#[derive(Debug)]
pub struct SNESController {
    state: u8, // current, ephemeral button states
    latch: u8, // latched button states.  button presses immediately update the latch.
    shift: u8, // shift register value.  transfers from latch on latch signal.
}

impl SNESController {
    pub fn new() -> SNESController {
        SNESController {
            state: 255u8,
            latch: 255u8,
            shift: 255u8,
        }
    }

    pub fn on_press(&mut self, btn: Button) {
        let bit = btn as u8;
        self.state &= !(1u8 << bit);
        self.latch &= !(1u8 << bit);
    }

    pub fn on_release(&mut self, btn: Button) {
        self.state |= 1u8 << (btn as u8);
    }

    pub fn peek(&self) -> u8 {
        self.shift & 1u8
    }

    pub fn read(&self) -> u8 {
        debug!("R");
        self.shift & 1u8
    }

    pub fn write(&mut self, latch: bool, clk: bool) {
        debug!("W L={:?} C={:?}", latch, clk);

        if latch {
            self.shift = self.latch;
            self.latch = self.state;
        }

        if clk {
            self.shift >>= 1;
        }
    }
}
