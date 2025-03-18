use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StyleEvent {
    Added(StyleAddedEvent),
    Modified(StyleModifiedEvent),
    Removed(StyleRemovedEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleAddedEvent {
    pub path: PathBuf,
    pub content: String,
    pub timestamp: i64,
}

impl StyleEvent {
    pub fn handle(&self, registry: &StyleRegistry) {
        match self {
            StyleEvent::Added(event) => {
                registry.add_style(&event.path, &event.content);
                StyleBroadcaster::global().broadcast(StyleUpdate::Added(event.clone()));
            },
            StyleEvent::Modified(event) => {
                registry.update_style(&event.path, &event.content);
                StyleBroadcaster::global().broadcast(StyleUpdate::Modified(event.clone()));
            },
            StyleEvent::Removed(event) => {
                registry.remove_style(&event.path);
                StyleBroadcaster::global().broadcast(StyleUpdate::Removed(event.clone()));
            },
        }
    }
}
