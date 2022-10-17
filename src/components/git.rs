use std::fmt::Display;

use git_status_parser::{EntryStatus, GitStatus};
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use subprocess::Exec;

use crate::components::{Color, Component, Sgr, SGR_RESET};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CountConfig {
    symbol: Option<String>,
    foreground: Option<Color>,
    bold: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BranchConfig {
    dirty: Option<Color>,
    clean: Option<Color>,
    bold: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GitConfig {
    branch: Option<BranchConfig>,
    staged: Option<CountConfig>,
    unstaged: Option<CountConfig>,
    untracked: Option<CountConfig>,
}

#[derive(Default)]
pub struct Git {
    config: GitConfig,
    branch: String,
    staged: i32,
    unstaged: i32,
    untracked: i32,
}

impl Git {
    pub fn new() -> Self {
        let gs = match Exec::shell("git status --branch --porcelain=2")
            .stdout(subprocess::Redirection::Pipe)
            .capture()
        {
            Ok(d) => match GitStatus::parse(d.stdout_str()) {
                Ok(gs) => gs,
                Err(_) => return Default::default(),
            },
            Err(_) => return Default::default(),
        };

        let mut g: Git = Default::default();

        g.branch = gs.branch_head.unwrap_or(String::new());

        for e in &gs.entries {
            match (&e.index, &e.working_tree) {
                (EntryStatus::Unmodified, EntryStatus::Unmodified) => (),
                (i, EntryStatus::Unmodified) if i != &EntryStatus::Unmodified => g.staged += 1,
                _ => g.unstaged += 1,
            };
        }

        for _ in &gs.untracked {
            g.untracked += 1;
        }

        g
    }
}

impl Display for Git {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(bc) = &self.config.branch {
            let sgr = Sgr {
                foreground: if (self.unstaged + self.untracked + self.staged) > 0 {
                    bc.dirty
                } else {
                    bc.clean
                },
                bold: bc.bold,
            };
            write!(f, "{sgr}{}{SGR_RESET} ", self.branch)?;
        } else {
            write!(f, "{} ", self.branch)?;
        }

        let mut fmt_count = |c: i32, cc: &Option<CountConfig>| {
            if c > 0 {
                match cc {
                    Some(cc) => {
                        let s = String::new();
                        let symbol = match &cc.symbol {
                            Some(s) => s,
                            None => &s,
                        };
                        let sgr = Sgr {
                            foreground: cc.foreground,
                            bold: cc.bold,
                        };
                        write!(f, "{sgr}{c}{}{SGR_RESET}", symbol)
                    }
                    None => write!(f, "{c}"),
                }?
            }
            Ok(())
        };

        fmt_count(self.staged, &self.config.staged)?;
        fmt_count(self.unstaged, &self.config.unstaged)?;
        fmt_count(self.untracked, &self.config.untracked)?;

        Ok(())
    }
}

impl Component for Git {
    fn id(&self) -> String {
        "git".to_string()
    }

    fn include(&self) -> bool {
        true
    }

    fn load_config(&mut self, config: Value) {
        if config == Value::Null {
            return;
        }
        self.config = serde_json::from_value(config).unwrap_or_else(|e| {
            error!("Failed parsing git config: {}", e);
            Default::default()
        });
    }
}
