use std::fmt::Display;

use gethostname::gethostname;
use serde_json::Value;

use crate::components::Component;

pub struct Hostname {
    hostname: String,
}

impl Hostname {
    pub fn new() -> Self {
        Hostname {
            hostname: String::from(gethostname().to_str().unwrap_or("")),
        }
    }
}

impl Display for Hostname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hostname)
    }
}

impl Component for Hostname {
    fn id(&self) -> String {
        "hostname".to_string()
    }

    fn include(&self) -> bool {
        true
    }

    fn load_config(&mut self, config: Value) {}
}
