use jsonschema::Validator;
use serde_json::Value;

pub trait RepoKitConstructValidator<T, V> {
    fn from_input(root: &str, input: T) -> V;
    fn on_parsing_error(root: &str, value: Value) -> Option<String>;

    fn is_valid(validator: &Validator, input: &Value) -> bool {
        if validator.validate(input).is_err() {
            return false;
        }
        true
    }
}
