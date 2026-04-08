use std::collections::HashMap;

use crate::{
    executables::internal_executable_definition::InternalExecutableDefinition,
    internal_commands::help::Help,
};

pub trait InternalExecutable {
    fn run(&self, args: Vec<String>, internals: &HashMap<String, Box<dyn InternalExecutable>>);
    fn get_definition(&self) -> &InternalExecutableDefinition;

    fn help(&self) {
        Help::log_internal_command(self.get_definition());
    }
}
