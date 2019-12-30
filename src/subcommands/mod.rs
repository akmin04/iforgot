pub mod delete;
pub mod howto;
pub mod list;
pub mod new;

use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter, Result};

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub id: usize,
    pub command: String,
    pub keywords: Vec<String>,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}: {}\n{}: {}\n{}: {}",
            "ID".bold().blue(),
            self.id,
            "Command".bold().blue(),
            self.command,
            "Keywords".bold().blue(),
            self.keywords.join(" "),
        )
    }
}
