pub use agent::BaseAgent;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod agent;

pub static AGENT: OnceCell<Arc<Mutex<BaseAgent>>> = OnceCell::new();
