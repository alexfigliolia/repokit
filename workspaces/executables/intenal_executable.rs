use crate::executables::internal_executable_definition::InternalExecutableDefinition;

pub trait InternalExecutable {
    fn run(&self, args: Vec<String>);
    fn help(&self);
    fn get_definition(&self) -> &InternalExecutableDefinition;
}
