pub struct JsonValidationError {
  json: String,
}

// Implement std::fmt::Display for JsonValidationError
impl std::fmt::Display for JsonValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Couldn't validate json: {}", self.json) // user-facing output
  }
}

// Implement std::fmt::Debug for AppError
impl std::fmt::Debug for JsonValidationError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
  }
}

pub fn validate_json(i: &str) -> Result<String, JsonValidationError> {
  let json: std::result::Result<serde_json::Value, serde_json::error::Error> =
    serde_json::from_str(i);
  match json {
    Ok(_) => {
      return Result::Ok(i.to_owned());
    }
    Err(_) => {
      return Result::Err(JsonValidationError { json: i.to_owned() });
    }
  }
}
