use std::collections::HashMap;

pub struct ValidationError {
    pub key: String,
    pub rule: String,
    pub message: String,
}

impl ValidationError {
    fn new(key: String, rule: String, message: String) -> Self {
        ValidationError { key, rule, message }
    }
}

// RULE TRAIT
pub trait Rule {
    fn check(&self, key: &str, value: &str) -> Option<ValidationError>;
}

pub struct SecretRule;
pub struct PortRule;
pub struct UrlRule;

// Impl
impl Rule for SecretRule {
    fn check(&self, key: &str, value: &str) -> Option<ValidationError> {
        if !key.contains("SECRET") {
            return None;
        }

        if value.is_empty() {
            return Some(ValidationError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }

        if value.chars().count() < 32 {
            return Some(ValidationError::new(
                key.to_string(),
                "min_length".to_string(),
                "must be greater than or equal to 32".to_string(),
            ));
        }

        None
    }
}

impl Rule for PortRule {
    fn check(&self, key: &str, value: &str) -> Option<ValidationError> {
        if !key.contains("PORT") {
            return None;
        }
        if value.is_empty() {
            return Some(ValidationError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }
        if !value.parse::<u16>().is_ok() {
            return Some(ValidationError::new(
                key.to_string(),
                "format".to_string(),
                "must be a number".to_string(),
            ));
        }

        None
    }
}

impl Rule for UrlRule {
    fn check(&self, key: &str, value: &str) -> Option<ValidationError> {
        if !key.contains("URL") {
            return None;
        }
        if value.is_empty() {
            return Some(ValidationError::new(
                key.to_string(),
                "empty".to_string(),
                "must not be empty".to_string(),
            ));
        }
        if !VALID_URL_PREFIXES
            .iter()
            .any(|prefix| value.starts_with(prefix))
        {
            return Some(ValidationError::new(
                key.to_string(),
                "format".to_string(),
                String::from(
                    "must start with http://, https://, postgres://, postgresql://, mysql://, redis://, rediss://, mongodb://, mongodb+srv://, amqp://, amqps://, sqlite://",
                ),
            ));
        }

        None
    }
}

// outside the loop - created once!!
const VALID_URL_PREFIXES: &[&str] = &[
    "http://",
    "https://",
    "postgres://",
    "postgresql://",
    "mysql://",
    "redis://",
    "rediss://",
    "mongodb://",
    "mongodb+srv://",
    "amqp://",
    "amqps://",
    "sqlite://",
];

pub fn validate_env(map: HashMap<String, Option<String>>) -> Vec<ValidationError> {
    let rules: Vec<Box<dyn Rule>> =
        vec![Box::new(SecretRule), Box::new(PortRule), Box::new(UrlRule)];

    let mut vec_errors: Vec<ValidationError> = Vec::new();

    for (key, value) in &map {
        let val_str = match value {
            None => "",
            Some(v) => v.as_str(),
        };

        for rule in &rules {
            if let Some(error) = rule.check(key, val_str) {
                vec_errors.push(error);
            }
        }
    }

    vec_errors
}
