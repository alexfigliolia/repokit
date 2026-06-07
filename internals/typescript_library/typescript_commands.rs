pub enum TypeScriptCommand {
    ParseCommands,
    ParseConfiguration,
}

impl TypeScriptCommand {
    pub fn resolve(&self) -> &str {
        match self {
            TypeScriptCommand::ParseCommands => "parse_commands",
            TypeScriptCommand::ParseConfiguration => "parse_configuration",
        }
    }
}
