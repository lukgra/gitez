pub struct Output {
    pub lines: Vec<String>,
}

impl Output {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn push(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }
}

