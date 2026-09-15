use schemars::JsonSchema;
use serde::{Deserialize, de::DeserializeOwned};

#[derive(Debug, Deserialize, Clone, JsonSchema)]
pub struct FileParseError {
    pub path: String,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Clone, JsonSchema)]
#[serde(bound = "T: DeserializeOwned")]
pub struct FileParseResult<T> {
    pub result: Option<T>,
    pub error: Option<FileParseError>,
}
