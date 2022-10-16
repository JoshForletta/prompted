use std::fmt::Display;

use git_status_parser::{EntryStatus, GitStatus};
use serde_json::Value;
use subprocess::Exec;

use crate::components::Component;

#[derive(Default)]
pub struct Git {
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
        write!(f, "{} ", self.branch)?;

        if self.staged > 0 {
            write!(f, "{}[:] ", self.staged)?;
        }
        if self.unstaged > 0 {
            write!(f, "{}[!] ", self.unstaged)?;
        }
        if self.untracked > 0 {
            write!(f, "{}[?] ", self.untracked)?;
        }

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

    fn load_config(&mut self, config: Value) {}
}
