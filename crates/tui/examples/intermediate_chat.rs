//! A chat application simulation using the simplified API.
//!
//! This example demonstrates:
//! - Using the simplified API with async support
//! - Message passing architecture
//! - Subscriptions for timed events
//! - More complex UI rendering
//!
//! Run with: cargo run --example intermediate_chat

use sage_tui::simple::async_api::*;
use std::time::Duration;

/// A simulated chat application
struct ChatApp {
    /// The list of messages
    messages: Vec<ChatMessage>,
    /// The current input text
    input: String,
    /// Whether we're "connected" to the chat server
    connected: bool,
    /// The simulated typing indicator
    typing_indicator: TypingIndicator,
}

/// A chat message
struct ChatMessage {
    /// The sender of the message
    sender: String,
    /// The content of the message
    content: String,
}

/// A typing indicator that shows when someone is "typing"
struct TypingIndicator {
    /// Whether someone is typing
    active: bool,
    /// The animation frame
    frame: usize,
}

impl TypingIndicator {
    /// Create a new typing indicator
    fn new() -> Self {
        Self {
            active: false,
            frame: 0,
        }
    }

    /// Get the current animation frame
    fn view(&self) -> &str {
        if !self.active {
            return "";
        }

        match self.frame % 4 {
            0 => "Someone is typing.",
            1 => "Someone is typing..",
            2 => "Someone is typing...",
            _ => "Someone is typing",
        }
    }
}

/// Messages that our chat app can handle
#[derive(Debug, Clone)]
enum Msg {
    /// Update the input field
    UpdateInput(String),
    /// Send the current message
    SendMessage,
    /// Receive a simulated message
    ReceiveMessage(String),
    /// Toggle the connection status
    ToggleConnection,
    /// Update the typing indicator
    UpdateTypingIndicator,
    /// Simulate receiving a message after a delay
    SimulateResponse,
}

impl ChatApp {
    /// Create a new chat app
    fn new() -> Self {
        Self {
            messages: vec![
                ChatMessage {
                    sender: "System".to_string(),
                    content: "Welcome to the chat! Press 'c' to connect.".to_string(),
                },
            ],
            input: String::new(),
            connected: false,
            typing_indicator: TypingIndicator::new(),
        }
    }


}

impl Component for ChatApp {
    type Msg = Msg;

    fn init(&mut self) -> Option<Command<Self::Msg>> {
        None
    }

    fn update(&mut self, msg: Self::Msg) -> Option<Command<Self::Msg>> {
        match msg {
            Msg::UpdateInput(input) => {
                self.input = input;
                None
            }
            Msg::SendMessage => {
                let content = self.input.clone();
                self.messages.push(ChatMessage {
                    sender: "You".to_string(),
                    content,
                });
                self.input.clear();

                // Simulate the other person typing
                self.typing_indicator.active = true;

                // Schedule a simulated response
                Some(Box::pin(async {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    Msg::SimulateResponse
                }))
            }
            Msg::ReceiveMessage(content) => {
                self.typing_indicator.active = false;
                self.messages.push(ChatMessage {
                    sender: "Friend".to_string(),
                    content,
                });
                None
            }
            Msg::ToggleConnection => {
                self.connected = !self.connected;
                if self.connected {
                    self.messages.push(ChatMessage {
                        sender: "System".to_string(),
                        content: "Connected to chat server.".to_string(),
                    });
                } else {
                    self.messages.push(ChatMessage {
                        sender: "System".to_string(),
                        content: "Disconnected from chat server.".to_string(),
                    });
                }
                None
            }
            Msg::UpdateTypingIndicator => {
                if self.typing_indicator.active {
                    self.typing_indicator.frame += 1;
                }
                None
            }
            Msg::SimulateResponse => {
                // Generate a simulated response
                let responses = [
                    "Hello there!",
                    "How are you doing?",
                    "That's interesting!",
                    "Tell me more about that.",
                    "I see what you mean.",
                ];
                let response = responses[self.messages.len() % responses.len()];

                Some(Box::pin(async {
                    Msg::ReceiveMessage(response.to_string())
                }))
            }
        }
    }

    fn view(&self, area: Size) -> String {
        let mut output = String::new();

        // Title
        output.push_str("🌿 Chat Application\n\n");

        // Connection status
        if self.connected {
            output.push_str("Status: Connected 🟢\n\n");
        } else {
            output.push_str("Status: Disconnected 🔴\n\n");
        }

        // Messages
        output.push_str("Messages:\n");
        output.push_str("----------\n");

        // Calculate how many messages we can show based on the area
        let max_messages = (area.rows as usize).saturating_sub(10);
        let start_idx = self.messages.len().saturating_sub(max_messages);

        for message in &self.messages[start_idx..] {
            output.push_str(&format!("{}: {}\n", message.sender, message.content));
        }

        output.push_str("----------\n");

        // Typing indicator
        if !self.typing_indicator.view().is_empty() {
            output.push_str(&format!("{}\n", self.typing_indicator.view()));
        }

        // Input field
        if self.connected {
            output.push_str(&format!("\nInput: {}_\n", self.input));
            output.push_str("\nPress Enter to send\n");
        } else {
            output.push_str("\nConnect to start chatting\n");
        }

        // Controls
        output.push_str("\nControls: c: Connect/Disconnect  q: Quit\n");

        output
    }

    fn handle_key_event(&self, key: KeyEvent) -> Option<Self::Msg> {
        match key.code {
            KeyCode::Char('c') => Some(Msg::ToggleConnection),
            _ => {
                if self.connected {
                    match key.code {
                        KeyCode::Char(c) => Some(Msg::UpdateInput(format!("{}{}", self.input, c))),
                        KeyCode::Backspace => {
                            if !self.input.is_empty() {
                                Some(Msg::UpdateInput(self.input[..self.input.len()-1].to_string()))
                            } else {
                                None
                            }
                        },
                        KeyCode::Enter => Some(Msg::SendMessage),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    fn subscriptions(&self) -> Subscriptions<Self::Msg> {
        // Update the typing indicator every 500ms
        Subscriptions::every(Duration::from_millis(500), || Msg::UpdateTypingIndicator)
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Create the chat app
    let app = ChatApp::new();

    // Run the application
    run(app).await
}
