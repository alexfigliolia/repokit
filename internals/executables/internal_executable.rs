use std::collections::HashMap;

use crate::executables::internal_executable_definition::InternalExecutableDefinition;

pub trait InternalExecutable {
    fn run(&self, args: Vec<String>, internals: &HashMap<String, Box<dyn InternalExecutable>>);
    fn help(&self);
    fn get_definition(&self) -> &InternalExecutableDefinition;
}
