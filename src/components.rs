use std::fmt::Display;

use serde_json::Value;

pub mod logname;

#[derive(Default)]
pub struct NoComponent {}

impl Display for NoComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Component for NoComponent {
    fn id(&self) -> String {
        "NO_COMPONENT".to_string()
    }
}

pub trait Component
where
    Self: Display,
{
    fn id(&self) -> String;

    fn include(&self) -> bool {
        false
    }

    fn load_config(&mut self, config: Value) {}
}
