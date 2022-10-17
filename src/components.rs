use std::fmt::Display;

use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod git;
pub mod hostname;
pub mod logname;
pub mod path;

pub static SGR_RESET: &str = "\x1b[m";

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Color {
    C8(u8),
    C24(u8, u8, u8),
}

#[derive(Default, Debug)]
pub struct Sgr {
    pub foreground: Option<Color>,
    pub bold: Option<bool>,
}

impl Display for Sgr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params: Vec<u8> = Vec::new();

        if let Some(color) = &self.foreground {
            match color {
                Color::C8(c) => params.extend_from_slice(&[38, 5, *c]),
                Color::C24(r, g, b) => params.extend_from_slice(&[38, 2, *r, *g, *b]),
            };
        }

        if let Some(bold) = self.bold {
            match bold {
                true => params.push(5),
                false => params.push(21),
            }
        }

        let params: Vec<String> = params.iter().map(|p| p.to_string()).collect();
        let params = params.join(";");

        write!(f, "\x1b[{params}m")
    }
}

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

    fn load_config(&mut self, _config: Value) {}
}
