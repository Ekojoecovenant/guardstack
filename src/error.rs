pub struct GuardStackError {
    pub key: String,
    #[allow(dead_code)]
    pub rule: String,
    pub message: String,
}

impl GuardStackError {
    pub fn new(key: String, rule: String, message: String) -> Self {
        GuardStackError { key, rule, message }
    }
}
