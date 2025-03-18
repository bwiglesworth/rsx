use std::future::Future;
use std::pin::Pin;

pub trait OnMount {
    fn on_mount(&self) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>;
}

#[derive(Debug)]
pub enum LifecycleState {
    BeforeMount,
    Mounted,
    Unmounted,
}