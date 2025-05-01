//! Core framework types and functionality.
//!
//! This module contains the essential components of the framework:
//! - Component trait for defining UI components
//! - Event handling for keyboard and other events
//! - Application runtime for managing the event loop
//! - Utilities for testing and subscriptions

use std::{future::Future, pin::Pin};
use async_channel::Receiver;
use std::time::Duration;
use crossterm::event as c_event;

// --- Component Trait ---

/// A type alias for boxed async tasks that eventually yield a message.
pub type Command<M> = Pin<Box<dyn Future<Output = M> + Send>>;

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

    /// Unique identifier (used by widgets/layout); auto-generated if you `#[derive(Component)]`.
    fn id(&self) -> Id {
        Id::new("root")
    }
}

// --- Event Handling ---

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

impl From<c_event::KeyEvent> for KeyEvent {
    fn from(ev: c_event::KeyEvent) -> Self {
        use c_event::{KeyCode as C, KeyModifiers as M};
        let code = match ev.code {
            C::Char(c) => KeyCode::Char(c),
            C::Enter => KeyCode::Enter,
            C::Esc => KeyCode::Esc,
            C::Backspace => KeyCode::Backspace,
            C::Left => KeyCode::Left,
            C::Right => KeyCode::Right,
            C::Up => KeyCode::Up,
            C::Down => KeyCode::Down,
            C::Tab => KeyCode::Tab,
            C::BackTab => KeyCode::BackTab,
            C::Delete => KeyCode::Delete,
            _ => KeyCode::Esc, // fallback
        };
        let mut modifiers = KeyModifiers::NONE;
        if ev.modifiers.contains(M::CONTROL) {
            modifiers |= KeyModifiers::CTRL;
        }
        if ev.modifiers.contains(M::ALT) {
            modifiers |= KeyModifiers::ALT;
        }
        if ev.modifiers.contains(M::SHIFT) {
            modifiers |= KeyModifiers::SHIFT;
        }
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

// --- Subscriptions ---

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

    /// Convert the subscription into a stream
    pub fn into_stream(self) -> impl futures::Stream<Item = M> {
        self.rx
    }
}



// --- Utilities ---

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

/// Stable, unique identifier for nested components / widgets.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Id(String);

impl Id {
    /// Create a new identifier
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self(s.into())
    }
}


