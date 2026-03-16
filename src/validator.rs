use std::collections::HashMap;

pub struct ValidationError {
    pub key: String,
    pub _rule: String,
    pub message: String,
}

pub fn validate_env(map: HashMap<String, Option<String>>) -> Vec<ValidationError> {
    // let jwt_val = map.get("JWT_SECRET");
    let mut vec_errors: Vec<ValidationError> = Vec::new();

    for (key, value) in map {
        if key.contains("SECRET") {
            match value {
                None => {
                    vec_errors.push(error_gen(
                        key,
                        "empty".to_string(),
                        "must not be empty".to_string(),
                    ));
                }
                Some(val) => {
                    let is_valid = val.chars().count() >= 32;
                    if !is_valid {
                        vec_errors.push(error_gen(
                            key,
                            "min_length".to_string(),
                            "must be greater than or equal to 32".to_string(),
                        ));
                    }
                }
            }
        } else if key.contains("PORT") {
            match value {
                None => {
                    vec_errors.push(error_gen(
                        key,
                        "empty".to_string(),
                        "must not be empty".to_string(),
                    ));
                }
                Some(val) => {
                    let is_num = val.parse::<u16>().is_ok();
                    if !is_num {
                        vec_errors.push(error_gen(
                            key,
                            "format".to_string(),
                            "must be a number".to_string(),
                        ));
                    }
                }
            }
        } else if key.contains("URL") {
            match value {
                None => {
                    vec_errors.push(error_gen(
                        key,
                        "empty".to_string(),
                        "must not be empty".to_string(),
                    ));
                }
                Some(val) => {
                    let is_valid = if val.starts_with("http://")
                        || val.starts_with("https://")
                        || val.starts_with("postgres://")
                        || val.starts_with("mysql://")
                    {
                        true
                    } else {
                        false
                    };
                    if !is_valid {
                        vec_errors.push(error_gen(
                            key,
                            "format".to_string(),
                            "must start with http://, https://, postgres://, mysql://".to_string(),
                        ));
                    }
                }
            }
        }
    }

    vec_errors
}

fn error_gen(key: String, _rule: String, message: String) -> ValidationError {
    ValidationError {
        key,
        _rule,
        message,
    }
}
