use std::fmt;

#[derive(Debug)]
pub enum SuDoKuError {
    InvalidValue,
    OutOfBound,
    NotSolveable,
    DuplicateValue,
}

impl fmt::Display for SuDoKuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for SuDoKuError {}
