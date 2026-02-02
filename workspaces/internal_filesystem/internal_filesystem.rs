use normalize_path::NormalizePath;
use std::path::{Path, PathBuf};

pub struct InternalFileSystem;

impl InternalFileSystem {
    pub fn absolute(segment: &str) -> PathBuf {
        let origin = env!("CARGO_MANIFEST_DIR");
        let path = Path::new(origin);
        path.join(segment).normalize()
    }

    pub fn resolve_command(file_name: &str) -> String {
        InternalFileSystem::path_buf_to_str(
            InternalFileSystem::commands_directory().join(file_name),
        )
    }

    pub fn resolve_template(file_name: &str) -> String {
        InternalFileSystem::path_buf_to_str(
            InternalFileSystem::templates_directory().join(file_name),
        )
    }

    fn commands_directory() -> PathBuf {
        InternalFileSystem::absolute("./src/commands")
    }

    fn templates_directory() -> PathBuf {
        InternalFileSystem::absolute("./src/templates")
    }

    fn path_buf_to_str(buffer: PathBuf) -> String {
        buffer
            .into_os_string()
            .into_string()
            .expect("Cannot construct path")
    }
}
