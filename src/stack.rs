use std::iter::Rev;
use std::slice::Iter;

pub struct Stack {
    stack: Vec<u16>
}

impl Stack {
    pub(crate) fn read_all(&self) -> Iter<u16> {
        self.stack.iter()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }
    pub fn push(&mut self, val: u16) {
        if self.stack.len() >= 16 {
            panic!("Stack overflow")
        }
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> u16 {
        self.stack.pop().unwrap()
    }
}