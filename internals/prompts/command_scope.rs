use colored::Colorize;
use std::fmt::{Display, Formatter};

use inquire_derive::Selectable;

#[derive(Debug, Copy, Clone, Selectable)]
pub enum CommandScope {
    Internal,
    Root,
    Registered,
}

impl Display for CommandScope {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandScope::Internal => write!(f, "{}", "Internal".bold()),
            CommandScope::Root => write!(f, "{}", "Root".bold()),
            CommandScope::Registered => write!(f, "{}", "Registered".bold()),
        }
    }
}
