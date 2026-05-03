use core::panic;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::sync::MutexGuard;

use colored::{ColoredString, Colorize};

use crate::repokit::repokit_runtime::RepoKitRuntime;
use crate::themes::theme::Theme;
use crate::themes::theme_registry::ThemeRegistry;

static REGISTERED_NAME: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new("Repokit".to_string()));

static THEMES: LazyLock<Mutex<ThemeRegistry>> = LazyLock::new(|| Mutex::new(ThemeRegistry::new()));

pub struct Logger;

impl Logger {
    pub fn initialize() {
        RepoKitRuntime::with_runtime(|runtime| {
            Logger::set_name(&runtime.configuration.project);
            for theme in &runtime.configuration.themes {
                Logger::with_registry(|mut registry| registry.register_user_theme(theme))
            }
            Logger::with_registry(|mut registry| {
                registry.set_theme(&runtime.caches.settings_cache.theme_preference)
            });
        });
    }

    pub fn info(message: &str) {
        println!("{}{}", Logger::info_prefix(), message);
    }

    pub fn error(message: &str) {
        eprintln!("{}{}", Logger::error_prefix(), message);
    }

    pub fn exit_with_info(message: &str) {
        Logger::info(message);
        panic!();
    }

    pub fn exit_with_error(message: &str) {
        Logger::error(message);
        panic!();
    }

    pub fn list(items: &[&str], indentation: Option<i32>) {
        Logger::with_surrounding_space(|| {
            for (index, item) in items.iter().enumerate() {
                println!("{}{}. {}", Logger::indent(indentation), index + 1, item);
            }
        })
    }

    pub fn parse_error(file_type: &str, stdout: &str) {
        Logger::info(format!("There was an error parsing your {}", file_type).as_str());
        Logger::info(
            "This can occur when RepoKit attempts to parse a TypeScript file that can terminate the process upon evaluation",
        );
        Logger::info("Please file a bug to with any related error or stack trace found below");
        Logger::log_issue_link();
        println!();
        if !stdout.is_empty() {
            println!("{stdout}");
        }
        panic!();
    }

    pub fn space_around(message: &str) {
        println!("\n{}{}\n", Logger::info_prefix(), message);
    }

    pub fn with_surrounding_space<F>(mut func: impl FnMut() -> F) -> F {
        println!();
        let result = func();
        println!();
        result
    }

    pub fn log_file_path(path: &str) {
        Logger::with_surrounding_space(|| {
            println!(
                "{}{}",
                Logger::indent(None),
                Logger::with_theme(|theme| theme.highlight(path))
            );
        })
    }

    pub fn list_file_paths(paths: &Vec<String>) {
        Logger::with_surrounding_space(|| {
            for path in paths {
                println!(
                    "{}{}",
                    Logger::indent(None),
                    Logger::with_theme(|theme| theme.highlight(path))
                );
            }
        })
    }

    pub fn indent(times: Option<i32>) -> String {
        let indentation: i32 = times.unwrap_or(5);
        " ".repeat(indentation.try_into().unwrap())
    }

    pub fn cyan(message: &str) -> ColoredString {
        message.cyan()
    }

    pub fn file_create_error() {
        Logger::file_error("create a file");
    }

    pub fn file_directory_error() {
        Logger::file_error("create a directory");
    }

    pub fn open_file_error() {
        Logger::file_error("read a file");
    }

    pub fn file_write_error() {
        Logger::file_error("write to a file");
    }

    pub fn log_issue_link() {
        Logger::log_file_path("https://github.com/alexfigliolia/repokit/issues");
    }

    pub fn with_theme<R>(func: impl Fn(&Theme) -> R) -> R {
        Logger::with_registry(|registry| func(registry.current_theme()))
    }

    pub fn with_registry<R>(func: impl Fn(MutexGuard<'_, ThemeRegistry>) -> R) -> R {
        let registry = THEMES.lock().unwrap();
        func(registry)
    }

    fn file_error(operation: &str) {
        Logger::info(format!("I was unable to {operation} in your repository").as_str());
        Logger::error("Please verify the permissions on your working directory or file a bug here");
        Logger::log_issue_link();
        panic!();
    }

    fn info_prefix() -> String {
        Logger::with_theme(|theme| format!("{}: ", theme.prefix(&REGISTERED_NAME.lock().unwrap())))
    }

    fn error_prefix() -> String {
        Logger::with_theme(|theme| {
            format!("{}: ", theme.error_prefix(&REGISTERED_NAME.lock().unwrap()))
        })
    }

    fn set_name(value: &str) {
        *REGISTERED_NAME.lock().unwrap() = value.to_string();
    }
}
