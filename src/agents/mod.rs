pub mod agent;

use std::sync::Arc;

pub use agent::BaseAgent;
use once_cell::sync::OnceCell;

pub static AGENT: OnceCell<Arc<BaseAgent>> = OnceCell::new();
