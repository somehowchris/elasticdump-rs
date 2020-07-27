use crate::helpers::url;

pub struct SourceValidationError {
    value: String,
}

// Implement std::fmt::Display for TimeUnitParserError
impl std::fmt::Display for SourceValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Couldn't validate source path: {}", self.value) // user-facing output
    }
}

// Implement std::fmt::Debug for TimeUnitParserError
impl std::fmt::Debug for SourceValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ value: {} }}", self.value) // programmer-facing output
    }
}

pub enum SourceType {
    S3,
    URL,
    FILE,
}

pub struct Source {
    r#type: SourceType,
    path: String,
}

// Implement std::fmt::Debug for Source
impl std::fmt::Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ type: {} , path: {} }}",
            self.r#type.to_str(),
            self.path
        ) // programmer-facing output
    }
}

impl SourceType {
    fn to_str(&self) -> &str {
        match self {
            SourceType::S3 => "s3",
            SourceType::FILE => "File",
            SourceType::URL => "Url",
        }
    }
}

pub fn validate_source_path(input: &str) -> Result<Source, SourceValidationError> {
    if input.starts_with("s3://") {
        return Result::Ok(Source {
            r#type: SourceType::S3,
            path: input.to_owned(),
        });
    } else if url::is_url(input) {
        return Result::Ok(Source {
            r#type: SourceType::URL,
            path: input.to_owned(),
        });
    } else if std::path::PathBuf::from(input).is_file() {
        return Result::Ok(Source {
            r#type: SourceType::FILE,
            path: input.to_owned(),
        });
    }

    Result::Err(SourceValidationError {
        value: input.to_owned(),
    })
}
