use std::str::FromStr;
pub struct TimeUnitParserError {
  value: String,
}

// Implement std::fmt::Display for TimeUnitParserError
impl std::fmt::Display for TimeUnitParserError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Couldn't validate json: {}", self.value) // user-facing output
  }
}

// Implement std::fmt::Debug for TimeUnitParserError
impl std::fmt::Debug for TimeUnitParserError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
  }
}

pub enum TimeUnit {
  Days,
  Hours,
  Minutes,
  Seconds,
  Milliseconds,
}

impl std::str::FromStr for TimeUnit {
  type Err = TimeUnitParserError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "ms" => Ok(TimeUnit::Milliseconds),
      "milliseconds" => Ok(TimeUnit::Milliseconds),
      "s" => Ok(TimeUnit::Seconds),
      "seconds" => Ok(TimeUnit::Seconds),
      "m" => Ok(TimeUnit::Minutes),
      "minutes" => Ok(TimeUnit::Minutes),
      "h" => Ok(TimeUnit::Hours),
      "hours" => Ok(TimeUnit::Hours),
      "d" => Ok(TimeUnit::Days),
      "days" => Ok(TimeUnit::Days),
      _ => Err(TimeUnitParserError {
        value: s.to_owned(),
      }),
    }
  }
}

impl TimeUnit {
  fn to_str(&self) -> &str {
    match self {
      TimeUnit::Days => "days",
      TimeUnit::Hours => "hours",
      TimeUnit::Minutes => "minute",
      TimeUnit::Seconds => "seconds",
      TimeUnit::Milliseconds => "milliseconds",
    }
  }
}

pub struct Duration {
  unit: TimeUnit,
  amount: u128,
}

// Implement std::fmt::Debug for AppError
impl std::fmt::Debug for Duration {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{{ unit: {}, amount: {} }}",
      self.unit.to_str(),
      self.amount
    ) // programmer-facing output
  }
}

impl Duration {
  pub fn new(input: &str) -> Result<Duration, TimeUnitParserError> {
    let substring_exclude_1_char = &input[..input.len() - 1];

    match substring_exclude_1_char.parse::<u128>() {
      Ok(value) => {
        let last_char = &input[input.len() - 1..];

        return Ok(Duration {
          unit: TimeUnit::from_str(&last_char)?,
          amount: value,
        });
      }
      Err(_) => {
        let substring_exclude_2_char = &input[..input.len() - 2];
        match substring_exclude_2_char.parse::<u128>() {
          Ok(value) => {
            let last_char = &input[input.len() - 2..];

            return Ok(Duration {
              unit: TimeUnit::from_str(&last_char)?,
              amount: value,
            });
          }
          Err(_) => {
            return Err(TimeUnitParserError {
              value: input.to_owned(),
            })
          }
        }
      }
    }
  }
}
