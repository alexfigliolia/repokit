use std::{collections::HashMap, env};

use crate::logger::logger::Logger;

#[derive(Clone)]
pub enum ArgvType {
    String,
    Boolean,
}

#[derive(Clone)]
pub struct ArgvOption {
    pub value_type: ArgvType,
    pub short: Option<&'static str>,
    pub name: &'static str,
    pub multiple: Option<bool>,
}

pub struct Argv {
    pub values: HashMap<String, Vec<String>>,
    pub positionals: Vec<String>,
    lookup_table: HashMap<String, ArgvOption>,
}

impl Argv {
    pub fn new<const N: usize>(schema: [ArgvOption; N], args: Option<Vec<String>>) -> Argv {
        let mut argv = Argv {
            values: HashMap::new(),
            positionals: Vec::new(),
            lookup_table: Argv::build_option_table(&schema),
        };
        argv.parse(args);
        argv
    }

    pub fn has(&self, key: &str) -> bool {
        let fallback_bucket = Vec::new();
        let values = self.values.get(key).unwrap_or(&fallback_bucket);
        !values.is_empty()
    }

    pub fn get_first(&self, key: &str) -> String {
        let fallback_bucket = Vec::new();
        let values = self.values.get(key).unwrap_or(&fallback_bucket);
        if values.is_empty() {
            return "".to_string();
        }
        values[0].clone()
    }

    pub fn get_all(&mut self, key: &str) -> &mut Vec<String> {
        self.values.entry(key.to_string()).or_default()
    }

    fn parse(&mut self, argv: Option<Vec<String>>) {
        let args = argv.unwrap_or(env::args().collect());
        let mut pointer = 0;
        let length = args.len();
        while pointer < length {
            let arg = &args[pointer];
            if self.lookup_table.contains_key(arg) {
                let schema = self.lookup_table.get(arg).expect("arg exists");
                let values = self.values.entry(schema.name.to_string()).or_default();
                match schema.value_type {
                    ArgvType::Boolean => {
                        values.push("1".to_string());
                        pointer += 1;
                        continue;
                    }
                    ArgvType::String => {
                        let mut current = pointer + 1;
                        while current < length {
                            let value = &args[current];
                            if self.lookup_table.contains_key(value) {
                                break;
                            }
                            let values_for_argument = values.len();
                            if values_for_argument == 0 || schema.multiple.is_some_and(|v| v) {
                                values.push(value.to_string());
                                current += 1;
                            } else {
                                break;
                            }
                        }
                    }
                }
            } else {
                self.positionals.push(arg.to_owned());
            }
            pointer += 1;
        }
    }

    fn build_option_table<const N: usize>(
        options: &[ArgvOption; N],
    ) -> HashMap<String, ArgvOption> {
        let mut table: HashMap<String, ArgvOption> = HashMap::new();
        for option in options {
            if option.name.len() < 2 {
                Logger::exit_with_error("Option names must be at least 2 characters");
            }
            let first_char = option
                .name
                .chars()
                .next()
                .expect("already checked for emptiness")
                .to_string();
            let short_flag = option.short.unwrap_or(&first_char);
            let flags = [format!("--{}", option.name), format!("-{}", short_flag)];
            for flag in flags {
                if table.contains_key(&flag) {
                    Logger::error(
                        format!(
                            "I encountered the flag {} more than once.",
                            Logger::with_theme(|theme| theme.highlight(&flag)),
                        )
                        .as_str(),
                    );
                    Logger::error(
                        format!(
                            "Short flags will default to the first letter of the option's name. If two options begin with the same letter, please specify the {} option for one of them.",
                            Logger::with_theme(|theme| theme.highlight("short")),
                        )
                        .as_str(),
                    );
                }
                table.insert(flag, option.clone());
            }
        }
        table
    }
}
