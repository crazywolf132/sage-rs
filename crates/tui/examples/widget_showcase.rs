//! A showcase of all available widgets using the full API.
//!
//! This example demonstrates:
//! - All built-in widgets (Text, Input, Select, Spinner, Layout)
//! - Complex layouts
//! - Advanced event handling
//! - Full component architecture
//!
//! Run with: cargo run --example widget_showcase

use sage_tui::simple::async_api::*;
use sage_tui::widgets::{Input, Select, Spinner, Layout, Row, Col};
use std::time::Duration;

/// A showcase of all available widgets
struct WidgetShowcase {
    /// The current tab
    tab: Tab,
    /// Text input widget
    input: Input,
    /// Select widget
    select: Select<String>,
    /// Spinner animation frame
    spinner_frame: usize,
    /// Spinner frames
    spinner_frames: Vec<&'static str>,
}

/// Available tabs
#[derive(Debug, Clone)]
enum Tab {
    /// Text widget showcase
    Text,
    /// Input widget showcase
    Input,
    /// Select widget showcase
    Select,
    /// Spinner widget showcase
    Spinner,
    /// Layout widget showcase
    Layout,
}

/// Messages that our showcase can handle
#[derive(Debug, Clone)]
enum Msg {
    /// Switch to a tab
    SwitchTab(Tab),
    /// Update the input widget
    UpdateInput(sage_tui::framework::KeyEvent),
    /// Update the select widget
    UpdateSelect(sage_tui::framework::KeyEvent),
    /// Update the spinner animation
    UpdateSpinner,
}

impl WidgetShowcase {
    /// Create a new widget showcase
    fn new() -> Self {
        Self {
            tab: Tab::Text,
            input: Input::default(),
            select: Select::new(vec![
                "Option 1".to_string(),
                "Option 2".to_string(),
                "Option 3".to_string(),
                "Option 4".to_string(),
                "Option 5".to_string(),
            ]),
            spinner_frame: 0,
            spinner_frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
        }
    }

    /// Render the text widget showcase
    fn render_text_tab(&self) -> String {
        let mut output = String::new();

        output.push_str("Text Widget Showcase\n");
        output.push_str("-------------------\n\n");

        output.push_str("The Text widget is the simplest widget. It displays text content.\n\n");

        // Different text styles
        output.push_str("Regular text: Hello, world!\n");
        output.push_str("Emphasized: *Hello, world!*\n");
        output.push_str("Bold: **Hello, world!**\n");
        output.push_str("Code: `Hello, world!`\n\n");

        // Using the Text widget
        let text = Text::new("This is rendered using the Text widget.");
        output.push_str(&format!("{}\n", text.view()));

        output
    }

    /// Render the input widget showcase
    fn render_input_tab(&self) -> String {
        let mut output = String::new();

        output.push_str("Input Widget Showcase\n");
        output.push_str("--------------------\n\n");

        output.push_str("The Input widget allows users to enter text.\n\n");

        // Show the input widget
        output.push_str("Try typing something:\n");
        output.push_str(&self.input.view());
        output.push_str("\n\nUse arrow keys to move the cursor, backspace to delete.\n");

        output
    }

    /// Render the select widget showcase
    fn render_select_tab(&self) -> String {
        let mut output = String::new();

        output.push_str("Select Widget Showcase\n");
        output.push_str("---------------------\n\n");

        output.push_str("The Select widget allows users to choose from a list of options.\n\n");

        // Show the select widget
        output.push_str("Use up/down arrows to navigate:\n\n");
        output.push_str(&self.select.view());
        output.push_str("\nSelected: Option ");
        output.push_str(&(self.select.selected + 1).to_string());

        output
    }

    /// Render the spinner widget showcase
    fn render_spinner_tab(&self) -> String {
        let mut output = String::new();

        output.push_str("Spinner Widget Showcase\n");
        output.push_str("----------------------\n\n");

        output.push_str("The Spinner widget shows an animation, useful for loading states.\n\n");

        // Show the spinner widget
        let _spinner = Spinner::new(&self.spinner_frames);
        output.push_str("Loading: ");
        output.push_str(self.spinner_frames[self.spinner_frame % self.spinner_frames.len()]);
        output.push_str("\n\nThis spinner automatically animates using a subscription.\n");

        output
    }

    /// Render the layout widget showcase
    fn render_layout_tab(&self) -> String {
        // Create a layout with rows and columns
        let layout = Layout::new()
            .row(
                Row::new()
                    .col(Col::new(|| "Top Left".to_string()))
                    .col(Col::new(|| "Top Right".to_string()))
            )
            .row(
                Row::new()
                    .col(Col::new(|| "Bottom Left".to_string()))
                    .col(Col::new(|| "Bottom Right".to_string()))
            );

        let mut output = String::new();

        output.push_str("Layout Widget Showcase\n");
        output.push_str("---------------------\n\n");

        output.push_str("The Layout widget organizes content in rows and columns.\n\n");

        // Show the layout
        output.push_str("Simple 2x2 grid layout:\n\n");
        output.push_str(&layout.view());
        output.push_str("\n\nLayouts can be nested to create complex UIs.\n");

        output
    }
}

impl Component for WidgetShowcase {
    type Msg = Msg;

    fn init(&mut self) -> Option<Command<Self::Msg>> {
        None
    }

    fn update(&mut self, msg: Self::Msg) -> Option<Command<Self::Msg>> {
        match msg {
            Msg::SwitchTab(tab) => {
                self.tab = tab;
                None
            }
            Msg::UpdateInput(key) => {
                self.input.update(key);
                None
            }
            Msg::UpdateSelect(key) => {
                self.select.update(key);
                None
            }
            Msg::UpdateSpinner => {
                self.spinner_frame = (self.spinner_frame + 1) % self.spinner_frames.len();
                None
            }
        }
    }

    fn view(&self, _area: Size) -> String {
        let mut output = String::new();

        // Title
        output.push_str("🌿 Widget Showcase\n\n");

        // Tabs
        output.push_str("Tabs: ");
        output.push_str("[1] Text | ");
        output.push_str("[2] Input | ");
        output.push_str("[3] Select | ");
        output.push_str("[4] Spinner | ");
        output.push_str("[5] Layout");
        output.push_str("\n\n");

        // Tab content
        match self.tab {
            Tab::Text => output.push_str(&self.render_text_tab()),
            Tab::Input => output.push_str(&self.render_input_tab()),
            Tab::Select => output.push_str(&self.render_select_tab()),
            Tab::Spinner => output.push_str(&self.render_spinner_tab()),
            Tab::Layout => output.push_str(&self.render_layout_tab()),
        }

        // Controls
        output.push_str("\n\nControls: 1-5: Switch tabs  q: Quit\n");

        output
    }

    fn handle_key_event(&self, key: KeyEvent) -> Option<Self::Msg> {
        match key.code {
            KeyCode::Char('1') => Some(Msg::SwitchTab(Tab::Text)),
            KeyCode::Char('2') => Some(Msg::SwitchTab(Tab::Input)),
            KeyCode::Char('3') => Some(Msg::SwitchTab(Tab::Select)),
            KeyCode::Char('4') => Some(Msg::SwitchTab(Tab::Spinner)),
            KeyCode::Char('5') => Some(Msg::SwitchTab(Tab::Layout)),
            _ => match self.tab {
                Tab::Input => {
                    let framework_key = sage_tui::framework::KeyEvent {
                        code: match key.code {
                            KeyCode::Char(c) => sage_tui::framework::KeyCode::Char(c),
                            KeyCode::Enter => sage_tui::framework::KeyCode::Enter,
                            KeyCode::Esc => sage_tui::framework::KeyCode::Esc,
                            KeyCode::Backspace => sage_tui::framework::KeyCode::Backspace,
                            KeyCode::Left => sage_tui::framework::KeyCode::Left,
                            KeyCode::Right => sage_tui::framework::KeyCode::Right,
                            KeyCode::Up => sage_tui::framework::KeyCode::Up,
                            KeyCode::Down => sage_tui::framework::KeyCode::Down,
                            KeyCode::Tab => sage_tui::framework::KeyCode::Tab,
                            KeyCode::BackTab => sage_tui::framework::KeyCode::BackTab,
                            KeyCode::Delete => sage_tui::framework::KeyCode::Delete,
                        },
                        modifiers: sage_tui::framework::KeyModifiers::NONE,
                    };
                    Some(Msg::UpdateInput(framework_key))
                },
                Tab::Select => {
                    let framework_key = sage_tui::framework::KeyEvent {
                        code: match key.code {
                            KeyCode::Char(c) => sage_tui::framework::KeyCode::Char(c),
                            KeyCode::Enter => sage_tui::framework::KeyCode::Enter,
                            KeyCode::Esc => sage_tui::framework::KeyCode::Esc,
                            KeyCode::Backspace => sage_tui::framework::KeyCode::Backspace,
                            KeyCode::Left => sage_tui::framework::KeyCode::Left,
                            KeyCode::Right => sage_tui::framework::KeyCode::Right,
                            KeyCode::Up => sage_tui::framework::KeyCode::Up,
                            KeyCode::Down => sage_tui::framework::KeyCode::Down,
                            KeyCode::Tab => sage_tui::framework::KeyCode::Tab,
                            KeyCode::BackTab => sage_tui::framework::KeyCode::BackTab,
                            KeyCode::Delete => sage_tui::framework::KeyCode::Delete,
                        },
                        modifiers: sage_tui::framework::KeyModifiers::NONE,
                    };
                    Some(Msg::UpdateSelect(framework_key))
                },
                _ => None,
            },
        }
    }

    fn subscriptions(&self) -> Subscriptions<Self::Msg> {
        // Update the spinner every 100ms
        Subscriptions::every(Duration::from_millis(100), || Msg::UpdateSpinner)
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Create the showcase
    let showcase = WidgetShowcase::new();

    // Run the application
    run(showcase).await
}
