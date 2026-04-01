use std::collections::HashMap;

use crate::{
    executables::{
        internal_executable::InternalExecutable, internal_executable_definition::RepoKitScope,
    },
    internal_commands::{
        list_commands::ListCommands, list_owners::ListOwners, list_themes::ListThemes,
        locate_command::LocateCommand, onboarder::Onboarder, register_command::RegisterCommand,
        search_commands::SearchCommands, upgrade_repokit::UpgradeRepoKit,
    },
};

pub struct InternalRegistry {
    scope: RepoKitScope,
}

impl InternalRegistry {
    pub fn new(scope: &RepoKitScope) -> InternalRegistry {
        InternalRegistry {
            scope: scope.clone(),
        }
    }

    pub fn get_all(&self) -> HashMap<String, Box<dyn InternalExecutable>> {
        let internals: [Box<dyn InternalExecutable>; 8] = [
            Box::new(Onboarder::new(&self.scope)),
            Box::new(ListCommands::new(&self.scope)),
            Box::new(SearchCommands::new(&self.scope)),
            Box::new(ListOwners::new(&self.scope)),
            Box::new(LocateCommand::new(&self.scope)),
            Box::new(RegisterCommand::new(&self.scope)),
            Box::new(UpgradeRepoKit::new(&self.scope)),
            Box::new(ListThemes::new(&self.scope)),
        ];
        HashMap::from(internals.map(|x| (x.get_definition().name.to_string(), x)))
    }
}
