use std::collections::HashMap;

use alphanumeric_sort::sort_str_slice;
use colored::Colorize;

use crate::{
    argv::argv::{Argv, ArgvOption, ArgvType},
    executables::{
        internal_executable::InternalExecutable,
        internal_executable_definition::{
            InternalExecutableDefinition, InternalExecutableDefinitionInput,
        },
    },
    logger::logger::Logger,
    repokit::repokit_runtime::RepoKitRuntime,
};

pub struct ListThemes {
    pub definition: InternalExecutableDefinition,
}

impl ListThemes {
    pub fn new() -> ListThemes {
        ListThemes {
            definition: InternalExecutableDefinition::define(InternalExecutableDefinitionInput {
                name: "themes",
                description: "Lists your repositories available themes",
                args: [(
                    "(--set | -s)",
                    "An optional flag allowing you to set your theme",
                )],
            }),
        }
    }

    fn list_themes(&self) {
        Logger::info("Listing available themes");
        let themes = Logger::with_registry(|registry| registry.themes.clone());
        let current_theme = Logger::with_theme(|theme| theme.name.clone());
        let mut keys: Vec<&String> = themes.keys().collect();
        sort_str_slice(&mut keys);
        Logger::with_surrounding_space(|| {
            for (index, name) in keys.iter().enumerate() {
                let mut post_fix = "";
                let is_active_theme = name.as_str() == current_theme;
                if is_active_theme {
                    post_fix = " <--- selected";
                }
                println!(
                    "{}{}{}",
                    Logger::indent(None),
                    Logger::with_theme(|theme| {
                        let name_text = if is_active_theme {
                            theme.highlight(name).bold()
                        } else {
                            theme.highlight(name)
                        };
                        format!("{}. {}", (index + 1).to_string().as_str(), name_text)
                    }),
                    post_fix
                );
            }
        });
    }
}

impl InternalExecutable for ListThemes {
    fn run(&self, args: Vec<String>, _: &HashMap<String, Box<dyn InternalExecutable>>) {
        let argv = Argv::new(
            [ArgvOption {
                name: "set",
                value_type: ArgvType::String,
                short: None,
                multiple: None,
            }],
            Some(args),
        );
        if !argv.has("set") {
            return self.list_themes();
        }
        let desired_theme = argv.get_first("set");
        if Logger::with_registry(|registry| !registry.has(&desired_theme)) {
            Logger::error(
                format!(
                    "I'm not aware of a theme named {}",
                    Logger::with_theme(|theme| theme.highlight(&desired_theme))
                )
                .as_str(),
            );
            return self.list_themes();
        }
        if Logger::with_registry(|mut registry| registry.set_theme(&desired_theme)) {
            RepoKitRuntime::with_runtime(|runtime| {
                runtime
                    .caches
                    .settings_cache
                    .store_theme_preference(&desired_theme);
            });
        }
        Logger::info(
            format!(
                "Your theme has been set to {}",
                Logger::with_theme(|theme| theme.highlight(&desired_theme))
            )
            .as_str(),
        );
    }

    fn get_definition(&self) -> &InternalExecutableDefinition {
        &self.definition
    }
}
