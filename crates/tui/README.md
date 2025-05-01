# 🌿 Sage TUI

A simple, ergonomic TUI framework for Rust with both synchronous and asynchronous APIs.

## Features

- **Two API levels** for developers of all skill levels
- **No feature flags** - everything is included by default
- **Clean, readable code** that's easy to understand and modify
- **Well-documented** with examples for each API level
- **Async support** for complex applications
- **Crossterm** for cross-platform terminal support

## Quick Start

Add sage-tui to your Cargo.toml:

```toml
[dependencies]
sage-tui = "0.1.0"
```

### Synchronous API (for beginners)

```rust
use sage_tui::simple::*;

// Define your app's state
struct Counter {
    value: i32,
}

// Implement the App trait
impl App for Counter {
    // Handle key presses
    fn update(&mut self, key: Key) {
        match key {
            Key::Char('+') => self.value += 1,
            Key::Char('-') => self.value -= 1,
            _ => {}
        }
    }

    // Render your app
    fn view(&self) -> String {
        format!("Counter: {}\n\nPress + to increment, - to decrement, q to quit", self.value)
    }
}

fn main() -> std::io::Result<()> {
    // Run the app
    run(Counter { value: 0 })
}
```

### Asynchronous API (for advanced developers)

```rust
use sage_tui::simple::async_api::*;

// Define a component
struct Counter { value: i32 }

// Define messages
enum Msg { Inc, Dec }

// Implement the Component trait
impl Component for Counter {
    type Msg = Msg;

    fn update(&mut self, msg: Self::Msg) -> Option<Command<Self::Msg>> {
        match msg {
            Msg::Inc => self.value += 1,
            Msg::Dec => self.value -= 1
        }
        None
    }

    fn view(&self, _area: Size) -> String {
        format!("Counter: {}", self.value)
    }

    fn handle_key_event(&self, key: KeyEvent) -> Option<Self::Msg> {
        match key.code {
            KeyCode::Char('+') => Some(Msg::Inc),
            KeyCode::Char('-') => Some(Msg::Dec),
            _ => None,
        }
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    run(Counter { value: 0 }).await
}
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
