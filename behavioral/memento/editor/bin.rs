use std::fmt;

// ========== Memento ==========
#[derive(Clone, Default)]
struct Memento {
    state: String,
}

impl Memento {
    fn new(state: String) -> Self {
        Memento { state }
    }

    fn get_state(&self) -> &str {
        &self.state
    }
}

// ========== Originator ==========
struct Editor {
    state: String,
}

impl Editor {
    fn new() -> Self {
        Editor {
            state: String::new(),
        }
    }

    fn type_text(&mut self, text: &str) {
        self.state.push_str(text);
    }

    fn create_memento(&self) -> Memento {
        Memento::new(self.state.clone())
    }

    fn restore(&mut self, memento: &Memento) {
        self.state = memento.get_state().to_string();
    }
}

impl fmt::Display for Editor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Editor State: {}", self.state)
    }
}

// ========== Caretaker ==========
struct History {
    mementos: Vec<Memento>,
}

impl History {
    fn new() -> Self {
        History {
            mementos: Vec::new(),
        }
    }

    fn save(&mut self, memento: Memento) {
        self.mementos.push(memento);
    }

    fn undo(&mut self) -> Option<Memento> {
        self.mementos.pop()
    }
}

// ========== usage ==========

fn main() {
    let mut editor = Editor::new();
    let mut history = History::new();

    editor.type_text("Hello");
    history.save(editor.create_memento());

    editor.type_text(", Rustaceans!");
    history.save(editor.create_memento());

    // Editor State: Hello, Rustaceans!
    println!("{editor}");

    if let Some(prev) = history.undo() {
        editor.restore(&prev);
        // Editor State: Hello
        println!("Undo: {editor}");
    }

    if let Some(prev) = history.undo() {
        editor.restore(&prev);
        // Editor State:
        println!("Undo Again: {editor}");
    }
}
