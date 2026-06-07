pub enum TypeScriptTemplate {
    CommandTemplate,
    ConfigurationTemplate,
}

impl TypeScriptTemplate {
    pub fn resolve(&self) -> &str {
        match self {
            TypeScriptTemplate::CommandTemplate => "command_template.txt",
            TypeScriptTemplate::ConfigurationTemplate => "configuration_template.txt",
        }
    }
}
