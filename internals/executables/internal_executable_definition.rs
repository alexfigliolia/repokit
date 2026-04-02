use std::collections::HashMap;

use crate::{
    git_scope::git_scope::GitScope,
    repokit::repokit_config::RepoKitConfig,
};

#[derive(Clone)]
pub struct RepoKitScope {
    pub root: String,
    pub commit_hash: String,
    pub configuration: RepoKitConfig,
}

impl RepoKitScope {
    pub fn new(git_scope: GitScope, configuration: RepoKitConfig) -> RepoKitScope {
        RepoKitScope {
            root: git_scope.root,
            commit_hash: git_scope.commit_hash,
            configuration,
        }
    }
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
