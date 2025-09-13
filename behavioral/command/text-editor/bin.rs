mod editor;
use std::{cell::RefCell, rc::Rc};

use editor::*;

fn main() {
    let editor = Rc::new(RefCell::new(Editor::new()));

    let cmd1 = Box::new(InsertTextCommand::new(editor.clone(), "Hello"));
    let cmd2 = Box::new(InsertTextCommand::new(editor.clone(), ", world!"));

    let mut queue = CommandQueue::new();
    queue.add_command(cmd1);
    queue.add_command(cmd2);

    queue.execute_all();

    println!("Editor content: {}", editor.borrow().content());

    queue.undo_last();
    println!("After undo: {}", editor.borrow().content());

    queue.show_log();
}
