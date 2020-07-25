use regex::Regex;

pub fn is_url(input: &str) -> bool {
  match url::Url::parse(input) {
    Ok(_output) => return input.starts_with("https://") || input.starts_with("http://"),
    _ => false,
  }
}

pub fn get_params(input: &str) -> std::collections::HashMap<&str, &str> {
  let query: &str = if Regex::new(r"^[?#]").unwrap().captures_iter(input).count() > 0 {
    &input[1..]
  } else {
    &input
  };

  let mut param_map = std::collections::HashMap::<&str, &str>::new();
  for part in query.split("&") {
    let key_value: Vec<&str> = part.split("=").collect();
    param_map.insert(&key_value[0], &key_value[1]);
  }
  return param_map;
}
