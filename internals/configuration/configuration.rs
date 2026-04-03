use std::path::Path;

use crate::{
    context::file_system::FileSystem, internal_filesystem::file_builder::FileBuilder,
    logger::logger::Logger, post_processing::post_processor::PostProcessor,
};

pub struct Configuration;

impl Configuration {
    pub fn create(files: &FileSystem) {
        let file_path = format!("{}/repokit.ts", &files.root);
        let path = Path::new(&file_path);
        if path.exists() {
            Logger::info(
                format!(
                    "I found a Repokit configuration without an exported {} instance",
                    Logger::with_theme(|theme| theme.highlight("RepokitConfig"))
                )
                .as_str(),
            );
            return Logger::exit_with_info("Please create an instance and export it");
        }
        Configuration::welcome();
        let mut source = files.resolve_template("configuration_template.txt");
        let mut target = FileBuilder::create(path, |_| Logger::file_create_error());
        FileBuilder::copy_to(&mut source, &mut target, |_| Logger::file_write_error());
        Logger::info(
            format!(
                "Please fill out this file with your desired settings. Then run {}",
                Logger::with_theme(|theme| theme.highlight("repokit onboard"))
            )
            .as_str(),
        );
        Logger::log_file_path(file_path.as_str());
        PostProcessor::get().flush();
    }

    fn welcome() {
        Logger::info("Welcome to Repokit! Let's get you setup");
        Logger::info("Creating your configuration file:");
    }
}
