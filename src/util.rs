use serde_json::Value;

/// Utility function to convert an array of strings to a JSON array.
pub fn strings_to_json_array(strings: &[String]) -> Value {
    Value::Array(strings.iter().map(|s| Value::String(s.clone())).collect())
}

/// Utility function to handle optional parameters for API requests.
pub fn insert_optional_param(params: &mut serde_json::Map<String, Value>, key: &str, value: Option<impl Into<Value>>) {
    if let Some(v) = value {
        params.insert(key.to_string(), v.into());
    }
}