#![warn(missing_docs)]
#![allow(unused_doc_comments)]
// Ignore warnings from the bitflags macro
#![allow(clippy::missing_docs_in_private_items)]
//! 🌿 **sage-tui** — a tiny, ergonomic TUI framework for Rust.
//!
//! This crate provides two different APIs for building terminal UIs:
//!
//! ## 1. Synchronous API (Beginners)
//!
//! Perfect for beginners and simple applications. No async, no complex types, just update and view.
//!
//! ```no_run
//! use sage_tui::simple::*;
//!
//! // Define your app's state
//! struct Counter {
//!     value: i32,
//! }
//!
//! // Implement the App trait
//! impl App for Counter {
//!     // Handle key presses
//!     fn update(&mut self, key: Key) {
//!         match key {
//!             Key::Char('+') => self.value += 1,
//!             Key::Char('-') => self.value -= 1,
//!             _ => {}
//!         }
//!     }
//!
//!     // Render your app
//!     fn view(&self) -> String {
//!         format!("Counter: {}", self.value)
//!     }
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     // Run the app
//!     run(Counter { value: 0 })
//! }
//! ```
//!
//! ## 2. Asynchronous API (Advanced)
//!
//! For more complex applications with async support and message passing.
//!
//! ```no_run
//! use sage_tui::simple::async_api::*;
//!
//! // Define a component
//! struct Counter { value: i32 }
//!
//! // Define messages
//! enum Msg { Inc, Dec }
//!
//! // Implement the Component trait
//! impl Component for Counter {
//!     type Msg = Msg;
//!
//!     fn update(&mut self, msg: Self::Msg) -> Option<Command<Self::Msg>> {
//!         match msg {
//!             Msg::Inc => self.value += 1,
//!             Msg::Dec => self.value -= 1
//!         }
//!         None
//!     }
//!
//!     fn view(&self, _area: Size) -> String {
//!         format!("Counter: {}", self.value)
//!     }
//!
//!     fn handle_key_event(&self, key: KeyEvent) -> Option<Self::Msg> {
//!         match key.code {
//!             KeyCode::Char('+') => Some(Msg::Inc),
//!             KeyCode::Char('-') => Some(Msg::Dec),
//!             _ => None,
//!         }
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> eyre::Result<()> {
//!     run(Counter { value: 0 }).await
//! }
//! ```

// --- core modules ---
pub mod framework;
pub mod widgets;

/// Simple TUI framework with both synchronous and asynchronous APIs
pub mod simple;

/// Common-used items - `use sage_tui::prelude::*` and start coding.
pub mod prelude {
    pub use crate::simple::async_api::{run, Command, Component, Size, KeyEvent, KeyCode, KeyModifiers};
    pub use crate::simple::async_api::{Subscriptions, Event, Text};
    pub use crate::simple::async_api::{init_terminal, cleanup_terminal, terminal_size, draw_to_terminal, next_event};
    pub use crate::widgets::{Input, Select, Spinner, Layout, Row, Col};
}

pub use sage_tui_derive::Component;
