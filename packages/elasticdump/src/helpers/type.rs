#[derive(Debug)]
pub enum Type {
  Data,
  Settings,
  Analyzer,
  Mapping,
  Alias,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTypeError {
  name: String,
}

impl std::string::ToString for ParseTypeError {
  fn to_string(&self) -> String {
    format!("Couldn't parse data transfer type: {}", self.name).to_owned()
  }
}

impl std::str::FromStr for Type {
  type Err = ParseTypeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "data" => Ok(Type::Data),
      "settings" => Ok(Type::Settings),
      "analyzer" => Ok(Type::Analyzer),
      "mapping" => Ok(Type::Mapping),
      "alias" => Ok(Type::Alias),
      _ => Err(ParseTypeError { name: s.to_owned() }),
    }
  }
}
