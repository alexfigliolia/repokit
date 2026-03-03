use std::{path::Path, process::exit};

use crate::{
    internal_filesystem::{file_builder::FileBuilder, internal_filesystem::InternalFileSystem},
    logger::logger::Logger,
};

pub struct Configuration;

impl Configuration {
    pub fn create(root: &str) {
        let file_path = format!("{root}/repokit.ts");
        let path = Path::new(&file_path);
        if path.exists() {
            Logger::info(
                format!(
                    "I found a Repokit configuration without an exported {} instance",
                    Logger::blue("RepokitConfig")
                )
                .as_str(),
            );
            return Logger::exit_with_info("Please create an instance and export it");
        }
        Configuration::welcome();
        let mut source =
            InternalFileSystem::new(root).resolve_template("configuration_template.txt");
        let mut target = FileBuilder::create(path, |_| Logger::file_create_error());
        FileBuilder::copy_to(&mut source, &mut target, |_| Logger::file_write_error());
        Logger::info(
            format!(
                "Please fill out this file with your desired settings. Then run {}",
                Logger::blue_bright("repokit onboard")
            )
            .as_str(),
        );
        Logger::log_file_path(file_path.as_str());
        exit(0);
    }

    fn welcome() {
        Logger::info("Welcome to Repokit! Let's get you setup");
        Logger::info("Creating your configuration file:");
    }
}
