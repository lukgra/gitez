use super::cursor::Cursor;

pub struct Input {
    pub value: String,
}

impl Input {
    pub fn new() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub fn insert_char(&mut self, cursor: &mut Cursor, c: char) {
        self.value.insert(cursor.pos, c);
        cursor.pos += 1;
    }

    pub fn backspace(&mut self, cursor: &mut Cursor) {
        if cursor.pos > 0 {
            self.value.remove(cursor.pos - 1);
            cursor.move_left();
        }
    }

    pub fn clear(&mut self, cursor: &mut Cursor) {
        self.value.clear();
        cursor.reset();
    }

    pub fn drain(&mut self, cursor: &mut Cursor) -> String {
        cursor.reset();
        self.value.drain(..).collect()
    }
}
