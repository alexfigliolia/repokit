use std::collections::HashMap;

use crate::{
    context::{
        cache_scope::CacheScope, file_system::FileSystem, git_scope::GitScope,
        node_scope::NodeScope, repokit_version_scope::RepoKitVersionScope,
        typescript_bridge::TypeScriptBridge,
    },
    repokit::repokit_config::RepoKitConfig,
};

#[derive(Clone)]
pub struct RepoKitScope {
    pub git: GitScope,
    pub node: NodeScope,
    pub files: FileSystem,
    pub cache: CacheScope,
    pub bridge: TypeScriptBridge,
    pub versions: RepoKitVersionScope,
    pub configuration: RepoKitConfig,
}

#[derive(Clone)]
pub struct InternalExecutableDefinition {
    pub name: String,
    pub description: String,
    pub args: Option<HashMap<String, String>>,
}

pub struct InternalExecutableDefinitionInput<'a, const N: usize> {
    pub name: &'a str,
    pub description: &'a str,
    pub args: [(&'a str, &'a str); N],
}

impl InternalExecutableDefinition {
    pub fn define<const N: usize>(
        definition: InternalExecutableDefinitionInput<N>,
    ) -> InternalExecutableDefinition {
        let InternalExecutableDefinitionInput {
            name,
            description,
            args,
        } = definition;
        InternalExecutableDefinition {
            name: String::from(name),
            description: String::from(description),
            args: InternalExecutableDefinition::args(args),
        }
    }

    pub fn args<const N: usize>(tuples: [(&str, &str); N]) -> Option<HashMap<String, String>> {
        if tuples.is_empty() {
            return None;
        }
        Some(HashMap::from(tuples.map(|(key, value)| {
            (String::from(key), String::from(value))
        })))
    }
}
