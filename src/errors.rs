#![warn(missing_docs)]
use std::{error::Error, fmt};

/// Enum representing reasons why a SEDOL string might be invalid
#[derive(Debug, PartialEq)]
pub enum SedolError {
    /// Invalid character present, only digits 0-9 and letters B-Z (excluding vowels) are allowed
    InvalidCharacter {
        /// The invalid char
        character: char,
    },
    /// Length must be 7
    InvalidLength,
    /// First char is a digit but rest of string is not ASCII digits. Old format SEDOLs contain only digits.
    InvalidOldFormat,
    /// Check digit is invalid
    InvalidCheckDigit {
        /// The check digit provided in the input
        got_check_digit: char,
        /// The calculated check digit
        calc_check_digit: char,
    },
}

impl fmt::Display for SedolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SedolError::InvalidCharacter { character } => {
                write!(f, "invalid character {}", character)
            }
            SedolError::InvalidLength => {
                write!(f, "invalid length, expected 7")
            }
            SedolError::InvalidOldFormat => {
                write!(
                    f,
                    "invalid format, expected all digits when first char is digit"
                )
            }
            SedolError::InvalidCheckDigit {
                got_check_digit,
                calc_check_digit,
            } => {
                write!(
                    f,
                    "invalid check digit {}, expected {}",
                    got_check_digit, calc_check_digit
                )
            }
        }
    }
}

impl Error for SedolError {}
