pub struct Stack {
    stack: [u16; 16],
    index: usize
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: [0; 16],
            index: 0
        }
    }
    pub fn push(&mut self, val: u16) {
        self.stack[self.index] = val;
        self.index = match self.index {
            16 => panic!("Stack overflow"),
            index => index + 1
        }
    }

    pub fn pop(&mut self) -> u16 {
        self.index = match self.index {
            0 => panic!("Stack empty while poping"),
            index => index - 1
        };
        let value: u16 = self.stack[self.index];
        self.stack[self.index] = 0;
        value
    }
}