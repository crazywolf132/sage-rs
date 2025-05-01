use sage_tui::simple::*;

/// A simple counter app
struct Counter {
    value: i32,
}

impl App for Counter {
    fn update(&mut self, key: Key) {
        match key {
            Key::Char('+') | Key::Char('=') => self.value += 1,
            Key::Char('-') | Key::Char('_') => self.value -= 1,
            _ => {}
        }
    }

    fn view(&self) -> String {
        format!(
            "🌿 Ultra Simple Counter\n\nValue: {}\n\nPress + to increment, - to decrement, q to quit",
            self.value
        )
    }
}

fn main() -> std::io::Result<()> {
    // Run the app with an initial counter value of 0
    run(Counter { value: 0 })
}
