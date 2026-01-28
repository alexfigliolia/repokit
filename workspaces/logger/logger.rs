use std::sync::Mutex;
use std::{process, sync::LazyLock};

use colored::{ColoredString, Colorize, CustomColor};

static REGISTERED_NAME: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new("Devkit".to_string()));

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

    pub fn exitWithInfo(message: &str) {
        Logger::info(message);
        process::exit(0);
    }

    pub fn exitWithError(message: &str) {
        Logger::error(message);
        process::exit(0);
    }

    pub fn space_around(message: &str) {
        println!("\n{}{}\n", Logger::info_prefix(), message);
    }

    pub fn indent(times: Option<i32>) -> String {
        let indentation: i32 = times.unwrap_or(5);
        " ".repeat(indentation.try_into().unwrap())
    }

    pub fn blue(message: &str) -> ColoredString {
        message.blue()
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
