pub mod agent;
pub mod sentience_wrapper;

pub use agent::BaseAgent;
use once_cell::sync::OnceCell;

pub static AGENT: OnceCell<BaseAgent> = OnceCell::new();
