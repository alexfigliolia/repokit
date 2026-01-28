use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
    process::exit,
};

use crate::logger::logger::Logger;

pub struct Configuration;

impl Configuration {
    pub fn create(root: &str) {
        Configuration::welcome();
        let file_path = format!("{root}/devkit.ts");
        let mut source = File::open(Configuration::template_path()).expect("Template");
        let mut target = File::create(Path::new(&file_path)).expect("creating");
        io::copy(&mut source, &mut target).expect("writing");
        target.sync_all().expect("Flushing");
        println!(
            "\n{}{}\n",
            Logger::indent(None),
            Logger::blue_bright(file_path.as_str()),
        );
        Logger::info("Please fill out this file with your desired settings");
        exit(0);
    }

    fn welcome() {
        Logger::info("Welcome to Devkit! Let's get you setup");
        Logger::info("Creating your configuration file:");
    }

    fn template_path() -> PathBuf {
        let file_path = file!();
        let dir = Path::new(file_path)
            .parent()
            .expect("Failed to get parent directory");
        dir.join("configuration_template.ts")
    }
}
