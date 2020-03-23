use log::debug;
use std::fmt;
use std::rc::Rc;
use std::sync::Mutex;

pub trait Attachment {
    fn step(&mut self);
}

pub struct Clock {
    attachments: Vec<Rc<Mutex<dyn Attachment>>>,
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Clock").finish()
    }
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            attachments: Vec::new(),
        }
    }

    pub fn attach(&mut self, attachment: Rc<Mutex<dyn Attachment>>) {
        self.attachments.push(attachment);
    }

    pub fn step(&mut self) {
        debug!("CLK STEP");
        self.attachments
            .iter_mut()
            .for_each(|a| a.lock().unwrap().step());
    }
}
