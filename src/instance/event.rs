use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum Event<T> {
    ConnectionAccepted(super::client::Client),
    MessageSent,
    MessageReceived(T),
}

#[derive(Clone)]
pub struct EventHandler {
    events: Arc<Mutex<Vec<String>>>,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn notify<T>(&self, event: Event<T>) {
        let event_str = match event {
            Event::ConnectionAccepted(_) => "ConnectionAccepted".to_string(),
            Event::MessageSent => "MessageSent".to_string(),
            Event::MessageReceived(_) => "MessageReceived".to_string(),
        };
        if let Ok(mut events) = self.events.lock() {
            events.push(event_str);
        }
    }

    pub fn get_events(&self) -> Vec<String> {
        self.events.lock().map_or(Vec::new(), |events| events.clone())
    }
}