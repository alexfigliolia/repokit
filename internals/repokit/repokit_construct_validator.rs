use jsonschema::Validator;
use serde_json::Value;

pub trait RepoKitConstructValidator<T, V> {
    fn from_input(root: &str, input: T) -> V;
    fn on_parsing_error(root: &str, value: Value) -> Option<String>;

    fn is_valid<'a>(validator: &Validator, input: &Value) -> bool {
        if let Err(_) = validator.validate(input) {
            return false;
        }
        true
    }
}
