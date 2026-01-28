use std::collections::HashMap;

#[derive(Clone)]
pub struct InternalExecutableDefinition {
    pub name: &'static str,
    pub description: &'static str,
    pub args: HashMap<&'static str, &'static str>,
}
