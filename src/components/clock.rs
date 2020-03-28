use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Attachment {
    fn cycle(&mut self);
}

pub struct Clock {
    attachments: Vec<Rc<RefCell<dyn Attachment>>>,
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

    pub fn attach(&mut self, attachment: Rc<RefCell<dyn Attachment>>) {
        self.attachments.push(attachment);
    }

    pub fn cycle(&mut self) {
        for a in &self.attachments {
            a.borrow_mut().cycle();
        }
    }
}
