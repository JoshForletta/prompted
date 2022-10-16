use std::fmt::Display;

use gethostname::gethostname;
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::components::{Color, Component, Sgr, SGR_RESET};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HostnameConfig {
    foreground: Option<Color>,
    bold: Option<bool>,
}

pub struct Hostname {
    config: HostnameConfig,
    hostname: String,
}

impl Hostname {
    pub fn new() -> Self {
        Hostname {
            config: Default::default(),
            hostname: String::from(gethostname().to_str().unwrap_or("")),
        }
    }
}

impl Display for Hostname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sgr = Sgr {
            foreground: self.config.foreground,
            bold: self.config.bold,
        };

        write!(f, "{sgr}{}{SGR_RESET}", self.hostname)
    }
}

impl Component for Hostname {
    fn id(&self) -> String {
        "hostname".to_string()
    }

    fn include(&self) -> bool {
        true
    }

    fn load_config(&mut self, config: Value) {
        if config == Value::Null {
            return;
        }
        self.config = serde_json::from_value(config).unwrap_or_else(|e| {
            error!("Failed parsing hostname config: {}", e);
            Default::default()
        });
    }
}
