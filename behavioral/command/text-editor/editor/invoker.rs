use super::command::Command;
use std::collections::VecDeque;
pub struct CommandQueue {
    queue: VecDeque<Box<dyn Command>>,
    history: Vec<Box<dyn Command>>,
    logs: Vec<String>,
}

impl CommandQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            history: vec![],
            logs: vec![],
        }
    }

    pub fn add_command(&mut self, cmd: Box<dyn Command>) {
        self.queue.push_back(cmd);
    }

    pub fn execute_all(&mut self) {
        while let Some(mut cmd) = self.queue.pop_front() {
            cmd.execute();
            self.logs.push(cmd.description());
            self.history.push(cmd);
        }
    }

    pub fn undo_last(&mut self) {
        if let Some(mut last_cmd) = self.history.pop() {
            last_cmd.undo();
            self.logs.push(format!("Undo: {}", last_cmd.description()));
        }
    }

    pub fn show_log(&self) {
        for entry in &self.logs {
            println!("[LOG] {entry}");
        }
    }
}
