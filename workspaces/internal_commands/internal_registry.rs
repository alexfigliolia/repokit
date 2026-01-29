use std::collections::HashMap;

use crate::{
    executables::intenal_executable::InternalExecutable,
    internal_commands::{locate_command::LocateCommand, register_command::RegisterCommand},
};

pub struct InternalRegistry {
    root: String,
}

impl InternalRegistry {
    pub fn new(root: String) -> InternalRegistry {
        InternalRegistry { root }
    }

    pub fn get_all(&self) -> HashMap<String, Box<dyn InternalExecutable>> {
        let internals: [Box<dyn InternalExecutable>; 2] = [
            Box::new(LocateCommand::new(self.root.clone())),
            Box::new(RegisterCommand::new(self.root.clone())),
        ];
        HashMap::from(internals.map(|x| (x.get_definition().name.to_string(), x)))
    }
}
