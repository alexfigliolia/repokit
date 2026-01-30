use crate::{
    devkit::devkit::DevKit, executor::executor::Executor,
    internal_commands::typescript_command::TypescriptCommand,
};

mod concurrency;
mod configuration;
mod devkit;
mod executables;
mod executor;
mod external_commands;
mod internal_commands;
mod logger;
mod validations;

fn main() {
    let root = Executor::exec("git rev-parse --show-toplevel", |cmd| cmd);
    let config = TypescriptCommand::new(root.clone()).parse_configuration();
    let kit = DevKit::new(root, config);
    kit.invoke();
}
