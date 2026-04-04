use jsonschema::Validator;
use serde_json::Value;

pub trait RepoKitConstructValidator {
    fn is_valid(validator: &Validator, input: &Value) -> bool {
        if validator.validate(input).is_err() {
            return false;
        }
        true
    }
}
