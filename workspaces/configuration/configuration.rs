use std::{fs::File, io, path::Path, process::exit};

use crate::{internal_filesystem::internal_filesystem::InternalFileSystem, logger::logger::Logger};

pub struct Configuration;

impl Configuration {
    pub fn create(root: &str) {
        let file_path = format!("{root}/repokit.ts");
        let path_buf = Path::new(&file_path);
        if path_buf.exists() {
            return;
        }
        Configuration::welcome();
        let template_path = InternalFileSystem::resolve_template("configuration_template.ts");
        let mut source = File::open(template_path).expect("Template");
        let mut target = File::create(path_buf).expect("creating");
        io::copy(&mut source, &mut target).expect("writing");
        target.sync_all().expect("Flushing");
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
