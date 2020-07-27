use regex::Regex;

pub struct RegexBuildError {
    value: String,
}

// Implement std::fmt::Display for TimeUnitParserError
impl std::fmt::Display for RegexBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Couldn't validate regex: {}", self.value) // user-facing output
    }
}

// Implement std::fmt::Debug for TimeUnitParserError
impl std::fmt::Debug for RegexBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ value: {} }}", self.value) // programmer-facing output
    }
}

pub fn build_regex(i: &str) -> Result<regex::Regex, RegexBuildError> {
    match Regex::new(i) {
        Ok(reg) => {
            return Result::Ok(reg);
        }
        Err(_) => {
            return Result::Err(RegexBuildError {
                value: i.to_owned(),
            });
        }
    }
}
