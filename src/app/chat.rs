pub enum ChatMessage {
    Commands(Vec<String>),
    Error(String),
}

pub struct Chat {
    pub messages: Vec<String>,
    pub error: Option<String>,
    pub pending_commands: Vec<String>,
    pub executed_commands: Vec<String>,
    pub current_command_value: String,
    pub current_command_index: usize,
}

impl Chat {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            error: None,
            pending_commands: Vec::new(),
            executed_commands: Vec::new(),
            current_command_value: String::new(),
            current_command_index: 0,
        }
    }

    pub fn push(&mut self, msg: String) {
        self.messages.push(msg);
    }

    pub fn set_error(&mut self, e: String) {
        self.error = Some(e);
    }

    pub fn clear_error(&mut self) {
        self.error = None;
    }

    pub fn load_commands(&mut self, cmds: Vec<String>) {
        self.pending_commands = cmds.clone();
        self.current_command_index = 0;
        self.current_command_value = cmds[self.current_command_index].clone();
        self.executed_commands.clear();
    }

    pub fn next_command(&mut self) -> bool {
        if self.current_command_index + 1 < self.pending_commands.len() {
            self.current_command_index += 1;
            self.current_command_value = self.pending_commands[self.current_command_index].clone();
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.error = None;
        self.pending_commands.clear();
        self.executed_commands.clear();
        self.current_command_value.clear();
        self.current_command_index = 0;
    }
}

