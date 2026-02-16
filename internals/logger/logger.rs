use std::process::exit;
use std::sync::LazyLock;
use std::sync::Mutex;

use colored::{ColoredString, Colorize, CustomColor};

static REGISTERED_NAME: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new("Repokit".to_string()));

pub struct Logger {}

impl Logger {
    pub fn set_name(value: &str) {
        *REGISTERED_NAME.lock().unwrap() = value.to_string();
    }

    pub fn info(message: &str) {
        println!("{}{}", Logger::info_prefix(), message);
    }

    pub fn error(message: &str) {
        eprintln!("{}{}", Logger::error_prefix(), message);
    }

    pub fn exit_with_info(message: &str) {
        Logger::info(message);
        exit(0);
    }

    pub fn exit_with_error(message: &str) {
        Logger::error(message);
        exit(0);
    }

    pub fn space_around(message: &str) {
        println!("\n{}{}\n", Logger::info_prefix(), message);
    }

    pub fn log_file_path(path: &str) {
        println!("\n{}{}\n", Logger::indent(None), Logger::blue_bright(path));
    }

    pub fn indent(times: Option<i32>) -> String {
        let indentation: i32 = times.unwrap_or(5);
        " ".repeat(indentation.try_into().unwrap())
    }

    pub fn blue(message: &str) -> ColoredString {
        message.bright_blue()
    }

    pub fn blue_bright(message: &str) -> ColoredString {
        message.bright_blue().bold()
    }

    pub fn magenta_bright(message: &str) -> ColoredString {
        message.bright_magenta().bold()
    }

    pub fn magenta(message: &str) -> ColoredString {
        message.magenta()
    }

    pub fn green(message: &str) -> ColoredString {
        message.green()
    }

    pub fn green_bright(message: &str) -> ColoredString {
        message.bright_green()
    }

    pub fn cyan(message: &str) -> ColoredString {
        message.cyan()
    }

    pub fn cyan_bright(message: &str) -> ColoredString {
        message.bright_cyan().bold()
    }

    pub fn gray(message: &str) -> ColoredString {
        message.custom_color(CustomColor {
            r: 128,
            g: 128,
            b: 128,
        })
    }

    pub fn lime(message: &str) -> ColoredString {
        message.custom_color(CustomColor {
            r: 175,
            g: 247,
            b: 7,
        })
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

    fn file_error(operation: &str) {
        Logger::info(format!("I was unable to {operation} in your repository").as_str());
        Logger::error("Please verify the permissions on your working directory or file a bug here");
        Logger::log_issue_link();
        exit(0);
    }

    fn info_prefix() -> ColoredString {
        format!("{}: ", *REGISTERED_NAME.lock().unwrap())
            .bright_magenta()
            .bold()
    }

    fn error_prefix() -> ColoredString {
        format!("{}: ", *REGISTERED_NAME.lock().unwrap())
            .red()
            .bold()
    }
}
