use colored::Colorize;
use std::fmt::{Display, Formatter};

use inquire_derive::Selectable;

#[derive(Debug, Copy, Clone, Selectable)]
pub enum YesNo {
    No,
    Yes,
}

impl Display for YesNo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            YesNo::No => write!(f, "{}", "No".bold()),
            YesNo::Yes => write!(f, "{}", "Yes".bold()),
        }
    }
}
