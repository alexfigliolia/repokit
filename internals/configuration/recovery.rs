use crate::{
    executor::executor::Executor, internal_filesystem::internal_filesystem::InternalFileSystem,
    logger::logger::Logger,
};

pub struct Recovery {
    root: String,
}

impl Recovery {
    pub fn new(root: &str) -> Recovery {
        Recovery {
            root: root.to_owned(),
        }
    }

    pub fn run(&mut self, file_path: &str) {
        let command = self.get_typecheck_command(file_path);
        Executor::with_stdio(command, |cmd| cmd);
    }

    pub fn get_typecheck_command(&self, file_path: &str) -> String {
        let executor = InternalFileSystem::get_node_executor(&self.root);
        let typescript_version = InternalFileSystem::get_typescript_version(executor);
        let ignore_config = if typescript_version >= 6 {
            " --ignoreConfig".to_string()
        } else {
            "".to_string()
        };
        let tsc_command = format!("{} tsc {} --noEmit{}", executor, file_path, ignore_config);
        tsc_command
    }

    pub fn prompt_to_fix_errors(&self, config_path: &str) {
        Logger::info(
            "Please fix the above type-errors and rerun your command"
                .to_string()
                .as_str(),
        );
        Logger::log_file_path(config_path);
    }
}
