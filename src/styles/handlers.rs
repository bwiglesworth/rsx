use tokio::sync::mpsc;

pub struct StyleEventHandler {
    sender: mpsc::Sender<StyleEvent>,
    registry: Arc<StyleRegistry>,
    compiler: Arc<StyleCompiler>,
}

impl StyleEventHandler {
    pub fn new(registry: Arc<StyleRegistry>, compiler: Arc<StyleCompiler>) -> Self {
        let (sender, mut receiver) = mpsc::channel(100);
        let registry_clone = registry.clone();
        
        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                let update = Self::process_event(event, ®istry_clone);
                StyleBroadcaster::global().broadcast(update);
            }
        });

        Self {
            sender,
            registry,
            compiler,
        }
    }

    fn process_event(event: StyleEvent, registry: &StyleRegistry) -> StyleUpdate {
        match event {
            StyleEvent::Added(e) => {
                let compiled = registry.compile_style(&e.content);
                StyleUpdate::Added { path: e.path, style: compiled }
            },
            StyleEvent::Modified(e) => {
                let compiled = registry.compile_style(&e.content);
                StyleUpdate::Modified { path: e.path, style: compiled }
            },
            StyleEvent::Removed(e) => {
                StyleUpdate::Removed { path: e.path }
            }
        }
    }
}
