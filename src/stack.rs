/// Limit the stack to 16 two-byte entries.
pub const STACK_DEPTH: usize = 16;

pub struct Stack {
    inner: [u16; 16],
    pointer: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            inner: [0u16; 16],
            pointer: 0,
        }
    }
    pub fn push(&mut self, val: u16) {
        if self.pointer >= STACK_DEPTH {
            eprintln!("Stack overflow");
            std::process::exit(1);
        }
        self.inner[self.pointer] = val;
        self.pointer += 1;
    }

    pub fn pop(&mut self) -> u16 {
        let val = self.inner[self.pointer];
        self.pointer -= 1;
        val
    }

    pub fn read_all(&self) -> impl Iterator<Item = u16> {
        let mut s = self.inner;
        s.reverse();
        s.into_iter()
    }
}
