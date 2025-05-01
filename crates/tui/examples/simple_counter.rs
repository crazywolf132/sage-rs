use sage_tui::simple::async_api::*;

/// A simple counter component that demonstrates the basics of the framework
#[derive(Default)]
struct Counter {
    value: i32
}

/// Messages that our counter component can handle
#[derive(Debug, Clone)]
enum Msg {
    Inc,  // Increment the counter
    Dec   // Decrement the counter
}

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
        format!("🌿 Simple Counter Example\n\nValue: {}\n\nPress + to increment, - to decrement, q to quit", self.value)
    }

    // Handle keyboard input
    fn handle_key_event(&self, key: KeyEvent) -> Option<Self::Msg> {
        match key.code {
            KeyCode::Char('+') | KeyCode::Char('=') => Some(Msg::Inc),
            KeyCode::Char('-') | KeyCode::Char('_') => Some(Msg::Dec),
            _ => None,
        }
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Create a new counter component
    let counter = Counter::default();

    // Run the application
    run(counter).await
}
