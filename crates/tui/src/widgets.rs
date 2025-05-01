//! Built-in widgets for creating user interfaces.
//! 
//! This module provides a collection of widgets that can be used to build UIs:
//! - Text: Simple text display
//! - Input: Text input field
//! - Select: Selection list
//! - Spinner: Loading indicator
//! - Layout: Flexible layout system

use crate::framework::{KeyCode, KeyEvent};

// --- Text Widget ---

/// A simple text widget for displaying content
pub struct Text<'a> { 
    /// The text content to display
    content: &'a str 
}

impl<'a> Text<'a> { 
    /// Create a new text widget
    pub fn new(content: &'a str) -> Self { 
        Self { content } 
    } 
}

impl std::fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        write!(f, "{}", self.content) 
    }
}

// --- Input Widget ---

/// A single-line text input widget
#[derive(Default)]
pub struct Input {
    /// The current value of the input
    pub value: String,
    /// The cursor position
    cursor: usize,
}

impl Input {
    /// Render the input
    pub fn view(&self) -> String { 
        format!("> {}_", self.value) 
    }

    /// Update the input based on a key event
    pub fn update(&mut self, ev: KeyEvent) {
        match ev.code {
            KeyCode::Char(c) => { 
                self.value.insert(self.cursor, c); 
                self.cursor += 1; 
            }
            KeyCode::Backspace if self.cursor > 0 => { 
                self.cursor -= 1; 
                self.value.remove(self.cursor); 
            }
            KeyCode::Left if self.cursor > 0 => self.cursor -= 1,
            KeyCode::Right if self.cursor < self.value.len() => self.cursor += 1,
            _ => {}
        }
    }
}

// --- Select Widget ---

/// A select list widget
pub struct Select<T> { 
    /// The items in the list
    items: Vec<T>, 
    /// The currently selected index
    pub selected: usize 
}

impl<T: ToString> Select<T> {
    /// Create a new select list
    pub fn new(items: Vec<T>) -> Self { 
        Self { items, selected: 0 } 
    }
    
    /// Update the selection based on a key event
    pub fn update(&mut self, ev: KeyEvent) {
        match ev.code {
            KeyCode::Up => { 
                if self.selected > 0 { 
                    self.selected -= 1; 
                } 
            }
            KeyCode::Down => { 
                if self.selected + 1 < self.items.len() { 
                    self.selected += 1; 
                } 
            }
            _ => {}
        }
    }
    
    /// Render the select list
    pub fn view(&self) -> String {
        self.items.iter().enumerate().map(|(i, it)| {
            if i == self.selected { 
                format!("➡ {}\n", it.to_string()) 
            } else { 
                format!("  {}\n", it.to_string()) 
            }
        }).collect()
    }
}

// --- Spinner Widget ---

/// A simple spinner widget
pub struct Spinner<'a> { 
    /// The frames to cycle through
    frames: &'a [&'a str], 
    /// The current frame index
    idx: usize 
}

impl<'a> Spinner<'a> { 
    /// Create a new spinner
    pub fn new(frames: &'a [&'a str]) -> Self { 
        Self { frames, idx: 0 } 
    } 
    
    /// Advance to the next frame
    pub fn tick(&mut self) { 
        self.idx = (self.idx + 1) % self.frames.len(); 
    } 
    
    /// Get the current frame
    pub fn view(&self) -> &str { 
        self.frames[self.idx] 
    } 
}

// --- Layout System ---

/// A simple layout system for organizing widgets
pub struct Layout { 
    /// The rows in this layout
    rows: Vec<Row> 
}

impl Layout { 
    /// Add a row to this layout
    pub fn row(mut self, row: Row) -> Self { 
        self.rows.push(row); 
        self 
    } 
    
    /// Create a new empty layout
    pub fn new() -> Self { 
        Self { rows: vec![] } 
    } 
    
    /// Render the layout
    pub fn view(&self) -> String { 
        self.rows.iter().map(|r| r.view()).collect::<Vec<_>>().join("\n") 
    } 
}

/// A row in a layout
pub struct Row { 
    /// The columns in this row
    cols: Vec<Col> 
}

impl Row { 
    /// Add a column to this row
    pub fn col(mut self, col: Col) -> Self { 
        self.cols.push(col); 
        self 
    } 
    
    /// Create a new empty row
    pub fn new() -> Self { 
        Self { cols: vec![] } 
    } 
    
    /// Render the row
    pub fn view(&self) -> String { 
        self.cols.iter().map(|c| c.view()).collect::<Vec<_>>().join(" ") 
    } 
}

/// A column in a layout
pub struct Col { 
    /// The content rendering function
    content: Box<dyn Fn() -> String + Send + Sync> 
}

impl Col { 
    /// Create a new column with the given content function
    pub fn new<F: Fn() -> String + 'static + Send + Sync>(f: F) -> Self { 
        Self { content: Box::new(f) } 
    } 
    
    /// Render the column
    pub fn view(&self) -> String { 
        (self.content)() 
    } 
}
