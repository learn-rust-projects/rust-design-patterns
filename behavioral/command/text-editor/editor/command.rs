pub trait Command {
    fn execute(&mut self);
    fn undo(&mut self);
    fn description(&self) -> String;
}

use crate::Editor;
use std::cell::RefCell;
use std::rc::Rc;

pub struct InsertTextCommand {
    editor: Rc<RefCell<Editor>>,
    text: String,
}

impl InsertTextCommand {
    pub fn new(editor: Rc<RefCell<Editor>>, text: impl Into<String>) -> Self {
        Self {
            editor,
            text: text.into(),
        }
    }
}

impl Command for InsertTextCommand {
    fn execute(&mut self) {
        self.editor.borrow_mut().insert(&self.text);
    }

    fn undo(&mut self) {
        self.editor.borrow_mut().backspace(self.text.len());
    }

    fn description(&self) -> String {
        format!("Insert '{}'", self.text)
    }
}
