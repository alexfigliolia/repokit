use std::collections::HashMap;

use crate::{
    devkit::interfaces::DevKitConfig,
    executables::intenal_executable::InternalExecutable,
    internal_commands::{
        list_commands::ListCommands, locate_command::LocateCommand, onboarder::Onboarder,
        register_command::RegisterCommand, upgrade_devkit::UpgradeDevKit,
    },
};

pub struct InternalRegistry {
    root: String,
    configuration: DevKitConfig,
}

impl InternalRegistry {
    pub fn new(root: String, configuration: DevKitConfig) -> InternalRegistry {
        InternalRegistry {
            root,
            configuration,
        }
    }

    pub fn get_all(&self) -> HashMap<String, Box<dyn InternalExecutable>> {
        let internals: [Box<dyn InternalExecutable>; 5] = [
            Box::new(Onboarder::new(
                self.root.clone(),
                self.configuration.clone(),
            )),
            Box::new(ListCommands::new(
                self.root.clone(),
                self.configuration.clone(),
            )),
            Box::new(LocateCommand::new(
                self.root.clone(),
                self.configuration.clone(),
            )),
            Box::new(RegisterCommand::new(
                self.root.clone(),
                self.configuration.clone(),
            )),
            Box::new(UpgradeDevKit::new(
                self.root.clone(),
                self.configuration.clone(),
            )),
        ];
        HashMap::from(internals.map(|x| (x.get_definition().name.to_string(), x)))
    }
}
