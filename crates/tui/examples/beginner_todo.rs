//! A simple todo list application using the ultra-simple API.
//! 
//! This example demonstrates:
//! - Basic application structure
//! - Handling keyboard input
//! - Rendering a simple UI
//! 
//! Run with: cargo run --example beginner_todo

use sage_tui::simple::*;

/// A simple todo list application
struct TodoApp {
    /// The list of todo items
    todos: Vec<String>,
    /// The current input text
    input: String,
    /// The currently selected todo item
    selected: usize,
    /// The current mode (Input or Navigation)
    mode: Mode,
}

/// Application modes
enum Mode {
    /// Editing the input field
    Input,
    /// Navigating the todo list
    Navigation,
}

impl TodoApp {
    /// Create a new todo app
    fn new() -> Self {
        Self {
            todos: vec![
                "Learn Rust".to_string(),
                "Build a TUI application".to_string(),
                "Share with friends".to_string(),
            ],
            input: String::new(),
            selected: 0,
            mode: Mode::Navigation,
        }
    }
}

impl App for TodoApp {
    fn update(&mut self, key: Key) {
        match self.mode {
            Mode::Navigation => match key {
                // Navigation mode keys
                Key::Char('a') => {
                    // Switch to input mode
                    self.mode = Mode::Input;
                    self.input.clear();
                }
                Key::Char('d') => {
                    // Delete the selected todo
                    if !self.todos.is_empty() {
                        self.todos.remove(self.selected);
                        if self.selected >= self.todos.len() && !self.todos.is_empty() {
                            self.selected = self.todos.len() - 1;
                        }
                    }
                }
                Key::Up => {
                    // Move selection up
                    if !self.todos.is_empty() && self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                Key::Down => {
                    // Move selection down
                    if !self.todos.is_empty() && self.selected < self.todos.len() - 1 {
                        self.selected += 1;
                    }
                }
                _ => {}
            },
            Mode::Input => match key {
                // Input mode keys
                Key::Enter => {
                    // Add the new todo and switch back to navigation mode
                    if !self.input.is_empty() {
                        self.todos.push(self.input.clone());
                        self.input.clear();
                        self.selected = self.todos.len() - 1;
                        self.mode = Mode::Navigation;
                    }
                }
                Key::Esc => {
                    // Cancel input and switch back to navigation mode
                    self.input.clear();
                    self.mode = Mode::Navigation;
                }
                Key::Backspace => {
                    // Delete the last character
                    self.input.pop();
                }
                Key::Char(c) => {
                    // Add the character to the input
                    self.input.push(c);
                }
                _ => {}
            },
        }
    }

    fn view(&self) -> String {
        let mut output = String::new();

        // Title
        output.push_str("🌿 Simple Todo List\n\n");

        // Todo items
        if self.todos.is_empty() {
            output.push_str("No todos yet. Press 'a' to add one.\n\n");
        } else {
            for (i, todo) in self.todos.iter().enumerate() {
                if i == self.selected {
                    output.push_str(&format!("➡ {}\n", todo));
                } else {
                    output.push_str(&format!("  {}\n", todo));
                }
            }
            output.push('\n');
        }

        // Input field
        match self.mode {
            Mode::Input => {
                output.push_str("Add todo: ");
                output.push_str(&self.input);
                output.push('_'); // Cursor
                output.push_str("\n\nPress Enter to add, Esc to cancel\n");
            }
            Mode::Navigation => {
                output.push_str("a: Add new todo  d: Delete selected  ↑/↓: Navigate  q: Quit\n");
            }
        }

        output
    }
}

fn main() -> std::io::Result<()> {
    // Create and run the todo app
    run(TodoApp::new())
}
