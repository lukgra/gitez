pub struct Cursor {
    pub pos: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self { pos: 0 }
    }

    pub fn move_left(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    pub fn move_right(&mut self, max: usize) {
        if self.pos < max {
            self.pos += 1;
        }
    }

    pub fn move_home(&mut self) {
        self.pos = 0;
    }

    pub fn move_end(&mut self, max: usize) {
        self.pos = max;
    }

    pub fn reset(&mut self) {
        self.pos = 0;
    }
}
