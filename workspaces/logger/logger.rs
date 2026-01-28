use std::process;

use colored::{ColoredString, Colorize};

pub struct Logger;

impl Logger {
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
        return " ".repeat(indentation.try_into().unwrap());
    }

    pub fn blue(message: &str) -> ColoredString {
        return message.blue();
    }

    pub fn blue_bright(message: &str) -> ColoredString {
        return message.bright_blue().bold();
    }

    pub fn magenta_bright(message: &str) -> ColoredString {
        return message.bright_magenta().bold();
    }

    pub fn magenta(message: &str) -> ColoredString {
        return message.bright_magenta();
    }

    pub fn green(message: &str) -> ColoredString {
        return message.green();
    }

    fn info_prefix() -> ColoredString {
        return "Devkit: ".bright_magenta().bold();
    }

    fn error_prefix() -> ColoredString {
        return "Devkit: ".red().bold();
    }
}
