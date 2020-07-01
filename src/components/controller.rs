use log::debug;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Button {
    A = 1,
    B = 2,
    Select = 3,
    Start = 4,
    Up = 5,
    Down = 6,
    Left = 7,
    Right = 8,
}

#[derive(Debug)]
pub struct SNESController {
    // count of unreported button presses
    pub events: HashMap<Button, i8>,

    // current button to be shifted out
    pub cur_button: Button,
}

impl SNESController {
    pub fn new() -> SNESController {
        let mut events = HashMap::new();
        events.insert(Button::A, 0);
        events.insert(Button::B, 0);
        events.insert(Button::Select, 0);
        events.insert(Button::Start, 0);
        events.insert(Button::Up, 0);
        events.insert(Button::Down, 0);
        events.insert(Button::Left, 0);
        events.insert(Button::Right, 0);

        SNESController {
            events: events,
            cur_button: Button::A,
        }
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'w' => {
                *self.events.get_mut(&Button::Up).unwrap() += 1;
            }
            's' => {
                *self.events.get_mut(&Button::Down).unwrap() += 1;
            }
            'a' => {
                *self.events.get_mut(&Button::Left).unwrap() += 1;
            }
            'd' => {
                *self.events.get_mut(&Button::Right).unwrap() += 1;
            }
            'j' => {
                *self.events.get_mut(&Button::A).unwrap() += 1;
            }
            'k' => {
                *self.events.get_mut(&Button::B).unwrap() += 1;
            }
            'l' => {
                *self.events.get_mut(&Button::Select).unwrap() += 1;
            }
            ';' => {
                *self.events.get_mut(&Button::Start).unwrap() += 1;
            }
            _ => {}
        }
    }

    pub fn peek(&self) -> u8 {
        let entry = self.events.get(&self.cur_button).unwrap();
        if *entry == 0 {
            0x01
        } else {
            0x00
        }
    }

    pub fn read(&self) -> u8 {
        debug!("R");
        self.peek()
    }

    pub fn write(&mut self, latch: u8, clk: u8) {
        debug!("W L={:?} C={:?}", latch, clk);

        if latch != 0 {
            self.cur_button = Button::A;
        }

        if clk != 0 {
            let event = self.events.get_mut(&self.cur_button).unwrap();
            if *event > 0 {
                *event -= 1;
            }
            self.cur_button = match self.cur_button {
                Button::A => Button::B,
                Button::B => Button::Select,
                Button::Select => Button::Start,
                Button::Start => Button::Up,
                Button::Up => Button::Down,
                Button::Down => Button::Left,
                Button::Left => Button::Right,
                Button::Right => Button::A,
            };
        }
    }
}
