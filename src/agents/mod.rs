pub mod agent;

use crate::agents::agent::BaseAgent;
use std::sync::OnceLock;

pub static AGENT: OnceLock<BaseAgent> = OnceLock::new();
