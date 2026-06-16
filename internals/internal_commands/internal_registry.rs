use std::collections::HashMap;

use crate::{
    executables::internal_executable::InternalExecutable,
    internal_commands::{
        interactive::Interactive, list_commands::ListCommands, list_owners::ListOwners,
        list_themes::ListThemes, list_version::ListVersion, locate_command::LocateCommand,
        onboarder::Onboarder, register_command::RegisterCommand, search_commands::SearchCommands,
        upgrade_repokit::UpgradeRepoKit,
    },
};

pub struct InternalCommandRegistry;

impl InternalCommandRegistry {
    pub fn new() -> InternalCommandRegistry {
        InternalCommandRegistry {}
    }

    pub fn get_all(&self) -> HashMap<String, Box<dyn InternalExecutable>> {
        let internals: [Box<dyn InternalExecutable>; 10] = [
            Box::new(Onboarder::new()),
            Box::new(Interactive::new()),
            Box::new(ListCommands::new()),
            Box::new(SearchCommands::new()),
            Box::new(ListOwners::new()),
            Box::new(LocateCommand::new()),
            Box::new(RegisterCommand::new()),
            Box::new(UpgradeRepoKit::new()),
            Box::new(ListThemes::new()),
            Box::new(ListVersion::new()),
        ];
        HashMap::from(internals.map(|x| (x.get_definition().name.to_string(), x)))
    }
}
