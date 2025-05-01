//! Simple TUI framework for Rust developers.
//!
//! This module provides both a synchronous and asynchronous API for creating terminal UIs.
//! It's designed to be easy to understand and modify, even for Rust beginners.
//!
//! # Synchronous API Example
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
//!         format!("Counter: {}\n\nPress + to increment, - to decrement, q to quit", self.value)
//!     }
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     // Run the app
//!     run(Counter { value: 0 })
//! }
//! ```
//!
//! # Asynchronous API Example
//!
//! ```no_run
//! use sage_tui::simple::async::*;
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
//!     run(Counter { value: 0 }, CrosstermBackend::new()?).await
//! }
//! ```

use std::io::{self, Write, stdout};
use crossterm::{
    terminal::{self, Clear, ClearType},
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEvent},
    ExecutableCommand,
};

/// A key press event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    /// A character key (a, b, c, etc.)
    Char(char),
    /// The Enter key
    Enter,
    /// The Escape key
    Esc,
    /// The Backspace key
    Backspace,
    /// The left arrow key
    Left,
    /// The right arrow key
    Right,
    /// The up arrow key
    Up,
    /// The down arrow key
    Down,
    /// The Tab key
    Tab,
    /// The Delete key
    Delete,
}

impl From<KeyEvent> for Key {
    fn from(key_event: KeyEvent) -> Self {
        match key_event.code {
            KeyCode::Char(c) => Key::Char(c),
            KeyCode::Enter => Key::Enter,
            KeyCode::Esc => Key::Esc,
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Tab => Key::Tab,
            KeyCode::Delete => Key::Delete,
            _ => Key::Esc, // Default to Esc for unknown keys
        }
    }
}

/// The core trait for your application
pub trait App {
    /// Handle a key press
    fn update(&mut self, key: Key);

    /// Render your app to a string
    fn view(&self) -> String;
}

/// Run your application
///
/// This function:
/// 1. Sets up the terminal (raw mode, alternate screen)
/// 2. Runs your app's update/view loop
/// 3. Cleans up the terminal when done
///
/// # Example
///
/// ```no_run
/// use sage_tui::simple::*;
///
/// struct Counter { value: i32 }
///
/// impl App for Counter {
///     fn update(&mut self, key: Key) {
///         match key {
///             Key::Char('+') => self.value += 1,
///             Key::Char('-') => self.value -= 1,
///             _ => {}
///         }
///     }
///
///     fn view(&self) -> String {
///         format!("Counter: {}", self.value)
///     }
/// }
///
/// fn main() -> std::io::Result<()> {
///     run(Counter { value: 0 })
/// }
/// ```
pub fn run<A: App>(mut app: A) -> io::Result<()> {
    // Set up terminal
    terminal::enable_raw_mode()?;
    stdout().execute(terminal::EnterAlternateScreen)?;

    // Main loop
    let result = run_app(&mut app);

    // Clean up terminal
    stdout().execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    // Return any error that occurred
    result
}

// The actual app loop
fn run_app<A: App>(app: &mut A) -> io::Result<()> {
    loop {
        // Render the app
        render(app)?;

        // Wait for a key press
        if let Event::Key(key) = event::read()? {
            // Check for quit keys
            if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                return Ok(());
            }

            // Update the app with the key
            app.update(key.into());
        }
    }
}

// Render the app to the terminal
fn render<A: App>(app: &A) -> io::Result<()> {
    // Move cursor to top-left
    stdout().execute(MoveTo(0, 0))?;

    // Clear the screen
    stdout().execute(Clear(ClearType::All))?;

    // Print the app's view
    print!("{}", app.view());

    // Flush stdout
    stdout().flush()
}

/// Asynchronous API for more complex applications
pub mod async_api {
    use std::{future::Future, pin::Pin, time::Duration};
    use crossterm::{terminal, event as c_event, ExecutableCommand, cursor};
    use futures::stream::StreamExt;
    use async_channel::Receiver;
    use std::io::{stdout, Write};
    use eyre;

    /// Simple 16-bit terminal size.
    #[derive(Debug, Clone, Copy)]
    pub struct Size {
        /// Number of columns (width)
        pub cols: u16,
        /// Number of rows (height)
        pub rows: u16
    }

    impl Size {
        /// Create a new Size
        pub fn new(cols: u16, rows: u16) -> Self {
            Self { cols, rows }
        }
    }

    /// Minimal, backend-agnostic key representation.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct KeyEvent {
        /// The key code (character, enter, escape, etc.)
        pub code: KeyCode,
        /// Any modifier keys that were pressed (ctrl, alt, shift)
        pub modifiers: KeyModifiers,
    }

    /// Key codes for keyboard events
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum KeyCode {
        /// A character key
        Char(char),
        /// The Enter key
        Enter,
        /// The Escape key
        Esc,
        /// The Backspace key
        Backspace,
        /// The Left arrow key
        Left,
        /// The Right arrow key
        Right,
        /// The Up arrow key
        Up,
        /// The Down arrow key
        Down,
        /// The Tab key
        Tab,
        /// The Shift+Tab key combination
        BackTab,
        /// The Delete key
        Delete,
    }

    /// Keyboard modifier keys (Ctrl, Alt, Shift)
    bitflags::bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct KeyModifiers: u8 {
            /// No modifiers
            const NONE = 0;
            /// Control key
            const CTRL = 0b0001;
            /// Alt key
            const ALT = 0b0010;
            /// Shift key
            const SHIFT = 0b0100;
        }
    }

    impl From<crossterm::event::KeyEvent> for KeyEvent {
        fn from(k: crossterm::event::KeyEvent) -> Self {
            let code = match k.code {
                c_event::KeyCode::Char(c) => KeyCode::Char(c),
                c_event::KeyCode::Enter => KeyCode::Enter,
                c_event::KeyCode::Esc => KeyCode::Esc,
                c_event::KeyCode::Backspace => KeyCode::Backspace,
                c_event::KeyCode::Left => KeyCode::Left,
                c_event::KeyCode::Right => KeyCode::Right,
                c_event::KeyCode::Up => KeyCode::Up,
                c_event::KeyCode::Down => KeyCode::Down,
                c_event::KeyCode::Tab => KeyCode::Tab,
                c_event::KeyCode::BackTab => KeyCode::BackTab,
                c_event::KeyCode::Delete => KeyCode::Delete,
                _ => KeyCode::Esc,
            };

            let modifiers = KeyModifiers::from_bits_truncate(
                ((k.modifiers.contains(c_event::KeyModifiers::CONTROL) as u8) << 0) |
                ((k.modifiers.contains(c_event::KeyModifiers::ALT) as u8) << 1) |
                ((k.modifiers.contains(c_event::KeyModifiers::SHIFT) as u8) << 2)
            );

            KeyEvent { code, modifiers }
        }
    }

    /// Non-key events you might subscribe to.
    #[derive(Debug, Clone)]
    pub enum Event<M> {
        /// A keyboard event
        Key(KeyEvent),
        /// A timer tick
        Tick,
        /// A custom message
        Msg(M),
    }

    impl<M> From<KeyEvent> for Event<M> {
        fn from(k: KeyEvent) -> Self {
            Event::Key(k)
        }
    }

    /// A type alias for boxed async tasks that eventually yield a message.
    pub type Command<M> = Pin<Box<dyn Future<Output = M> + Send>>;

    /// A list of event streams merged into a single receiver.
    pub struct Subscriptions<M> {
        rx: Receiver<M>,
    }

    impl<M: Send + 'static> Subscriptions<M> {
        /// Create an empty subscription that never produces events
        pub fn none() -> Self {
            let (tx, rx) = async_channel::unbounded();
            drop(tx); // closed channel -> never produces
            Self { rx }
        }

        /// Emit `msg()` every `duration`.
        pub fn every<F>(duration: Duration, mut msg: F) -> Self
        where F: FnMut() -> M + Send + 'static {
            let (tx, rx) = async_channel::unbounded();
            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(duration).await;
                    if tx.send(msg()).await.is_err() { break; }
                }
            });
            Self { rx }
        }

        pub(crate) fn into_stream(self) -> impl futures::Stream<Item = M> {
            self.rx
        }
    }

    /// All user components implement this trait. It's deliberately *ergonomic* -
    /// most methods have a default so you can implement just [`update`] and [`view`].
    #[async_trait::async_trait]
    pub trait Component: Send + Sized + 'static {
        /// The message type produced by events, subscriptions, or commands.
        type Msg: Send + 'static;

        /// Called once before the first frame. Return a [`Command`] to run async work.
        fn init(&mut self) -> Option<Command<Self::Msg>> {
            None
        }

        /// Handle a message & mutate internal state.
        fn update(&mut self, msg: Self::Msg) -> Option<Command<Self::Msg>>;

        /// Render into a [`String`]. `area` gives the current terminal size.
        fn view(&self, area: Size) -> String;

        /// Convert a key event into a component message.
        /// This is where you map keyboard input to your component's messages.
        /// Return None if the key event doesn't map to any message.
        fn handle_key_event(&self, _key: KeyEvent) -> Option<Self::Msg> {
            None
        }

        /// Event subscriptions (keyboard, tick, etc.).
        fn subscriptions(&self) -> Subscriptions<Self::Msg> {
            Subscriptions::none()
        }
    }

    /// Initialize the terminal for rendering
    pub fn init_terminal() -> std::io::Result<()> {
        terminal::enable_raw_mode()?;
        stdout().execute(terminal::EnterAlternateScreen)?;
        Ok(())
    }

    /// Clean up the terminal
    pub fn cleanup_terminal() -> std::io::Result<()> {
        stdout().execute(terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Get the current terminal size
    pub fn terminal_size() -> std::io::Result<Size> {
        let (cols, rows) = terminal::size()?;
        Ok(Size { cols, rows })
    }

    /// Draw content to the terminal
    pub fn draw_to_terminal(content: String) -> std::io::Result<()> {
        // Move cursor to top-left corner
        stdout().execute(cursor::MoveTo(0, 0))?;

        // Clear the screen but keep the cursor position
        stdout().execute(terminal::Clear(terminal::ClearType::All))?;

        // Print the content
        print!("{}", content);

        // Make sure it's displayed immediately
        stdout().flush()?;

        Ok(())
    }

    /// Wait for the next event
    pub async fn next_event() -> Event<()> {
        loop {
            if c_event::poll(std::time::Duration::from_millis(16)).unwrap() {
                if let c_event::Event::Key(k) = c_event::read().unwrap() {
                    return Event::Key(KeyEvent::from(k));
                }
            }
        }
    }

    /// Main event-loop - enter raw mode, draw frames, route events.
    ///
    /// This is the heart of the application. It:
    /// 1. Draws the current view
    /// 2. Waits for events (keyboard, timer, etc.)
    /// 3. Updates the component state based on events
    /// 4. Repeats
    pub async fn run<C>(mut root: C) -> eyre::Result<()>
    where
        C: Component,
    {
        // Initialize the terminal
        init_terminal()?;

        // Set up cleanup on drop
        struct TerminalCleanup;
        impl Drop for TerminalCleanup {
            fn drop(&mut self) {
                let _ = cleanup_terminal();
            }
        }
        let _cleanup = TerminalCleanup;

        let mut cmd = root.init();
        let mut sub_stream = Box::pin(root.subscriptions().into_stream());

        loop {
            // Get the current size once
            let size = terminal_size()?;

            // Draw the current view
            let content = root.view(size);
            draw_to_terminal(content)?;

            // Process one event at a time
            let event = next_event().await;

            // Handle key events
            if let Event::Key(key) = event {
                // Check for quit keys first
                if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
                    return Ok(());
                }

                // Let the component handle the key event
                if let Some(msg) = root.handle_key_event(key) {
                    cmd = root.update(msg);
                }
            }

            // Process any subscription events
            if let Some(msg) = sub_stream.next().await {
                cmd = root.update(msg);
            }

            // Process any commands
            if let Some(c) = cmd {
                let msg = c.await;
                cmd = root.update(msg);
            }
        }
    }

    /// A simple text widget
    pub struct Text<'a> {
        /// The text content
        content: &'a str
    }

    impl<'a> Text<'a> {
        /// Create a new text widget
        pub fn new(content: &'a str) -> Self {
            Self { content }
        }

        /// Get the content as a string
        pub fn view(&self) -> String {
            self.content.to_string()
        }
    }
}
