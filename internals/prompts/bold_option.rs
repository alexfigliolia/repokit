use colored::Colorize;
use std::fmt::{Display, Formatter};

pub struct BoldOption(pub String);

impl Display for BoldOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.bold())
    }
}

impl BoldOption {
    pub fn get(&self) -> &str {
        (&self.0) as _
    }
}
