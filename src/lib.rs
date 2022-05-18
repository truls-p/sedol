//! SEDOL
//!
//! Crate to validate SEDOLs.
//!
//! <https://en.wikipedia.org/wiki/SEDOL>
//!
//! <https://www.lseg.com/markets-products-and-services/data-analytics/data-solutions/sedol/documentation>
//!
//! # Examples
//! ```
//!let sedol_string = "BD9MZZ7";
//!match sedol::validate(sedol_string) {
//!    Ok(s) => println!("SEDOL validated: {}", s),
//!    Err(e) => eprint!("{}", e),
//!}
//!
//!let invalid_sedol_string = "BD9MZZ6";
//!match sedol::validate(invalid_sedol_string) {
//!    Ok(s) => println!("SEDOL validated: {}", s),
//!    Err(e) => eprintln!("{}", e),
//!}
//!
//!let unclean_sedol_string = " BD9-MZ-Z7?";
//!match sedol::validate(&sedol::clean(unclean_sedol_string)) {
//!    Ok(s) => println!("SEDOL validated: {}", s),
//!    Err(e) => eprintln!("{}", e),
//!}
//!
//!let sedol_6_string = "BD9MZZ";
//!println!("SEDOL with calculated check digit: {}{}", sedol_6_string, sedol::calc_check_digit(sedol_6_string));
//! ```

#![warn(missing_docs)]

mod errors;
pub use errors::SedolError;

/// Remove all characters except is_ascii_alphabetic and is_ascii_digit
pub fn clean(sedol: &str) -> String {
    sedol.replace(
        |x: char| !x.is_ascii_alphabetic() && !x.is_ascii_digit(),
        "",
    )
}

/// Check if the SEDOL is valid.
///
/// We do the checks in the following order:
/// 1. only digits 0-9 and letters B-Z (excluding vowels) are present
/// 2. the length of the string is 7
/// 3. all characters are digits if the first char is a digit
/// 4. compute and compare the check digit
pub fn validate(sedol: &str) -> Result<&str, SedolError> {
    let allowed_characters = "0123456789BCDFGHJKLMNPQRSTVWXYZ";
    for character in sedol.chars() {
        if !allowed_characters.contains(character) {
            return Err(SedolError::InvalidCharacter { character });
        }
    }
    if sedol.len() != 7 {
        return Err(SedolError::InvalidLength);
    }
    if sedol.chars().next().unwrap().is_numeric() && !sedol.bytes().all(|c| c.is_ascii_digit()) {
        return Err(SedolError::InvalidOldFormat);
    }
    let got_check_digit = sedol.chars().rev().next().unwrap();
    let calc_check_digit = calc_check_digit(sedol);

    if got_check_digit != calc_check_digit {
        return Err(SedolError::InvalidCheckDigit {
            got_check_digit,
            calc_check_digit,
        });
    }
    return Ok(sedol);
}

/// Calculate the check digits for the sedol
pub fn calc_check_digit(sedol: &str) -> char {
    let weights = [1, 3, 1, 7, 3, 9];
    let allowed_characters = "0123456789 BCD FGH JKLMN PQRST VWXYZ"; // spaces are important for indexing
    let s: usize = weights
        .iter()
        .zip(sedol.chars())
        .map(|(x, y)| x * allowed_characters.chars().position(|c| c == y).unwrap())
        .sum();
    let check_digit = char::from_digit((10 - s as u32 % 10) % 10, 10).unwrap();
    check_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_character() {
        assert_eq!(
            Err(SedolError::InvalidCharacter { character: 'A' }),
            validate("A15KXQ8")
        );
    }

    #[test]
    fn valid() {
        assert_eq!("B15KXQ8", validate("B15KXQ8").unwrap());
    }

    #[test]
    fn valid_two() {
        assert_eq!("5954135", validate("5954135").unwrap());
    }

    #[test]
    fn invalid_new_format() {
        assert_eq!(Err(SedolError::InvalidOldFormat), validate("015KXQ8"));
    }
    #[test]
    fn invalid_checkdigit() {
        assert_eq!(
            Err(SedolError::InvalidCheckDigit {
                got_check_digit: '7',
                calc_check_digit: '8'
            }),
            validate("B15KXQ7")
        );
    }

    #[test]
    fn clean_and_validate() {
        // Common UK DMO format: https://www.dmo.gov.uk/media/12976/pr160216.pdf
        assert_eq!("BD9MZZ7", validate(&clean("BD-9MZ-Z7")).unwrap());
    }

    #[test]
    fn clean_and_validate_two() {
        assert_eq!("BD9MZZ7", validate(&clean("BD-9MZ-Z7??!!  ")).unwrap());
    }

    #[test]
    fn clean_and_validate_mismatch() {
        assert_eq!(
            Err(SedolError::InvalidCheckDigit {
                got_check_digit: '6',
                calc_check_digit: '7'
            }),
            validate(&clean("BD-9MZ-Z6??!!  "))
        );
    }

    #[test]
    fn test_error_format() {
        let invalid_sedol_string = "BD9MZZ6";
        match validate(invalid_sedol_string) {
            Err(e) => assert_eq!("invalid check digit 6, expected 7", format!("{}",e)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_error_format_invalid_old() {
        let invalid_sedol_string = "0D9MZZ6";
        match validate(invalid_sedol_string) {
            Err(e) => assert_eq!("invalid format, expected all digits when first char is digit", format!("{}",e)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_error_format_invalid_length() {
        let invalid_sedol_string = "0D9MZZ";
        match validate(invalid_sedol_string) {
            Err(e) => assert_eq!("invalid length, expected 7", format!("{}",e)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_error_format_invalid_char() {
        let invalid_sedol_string = "!D9MZZ";
        match validate(invalid_sedol_string) {
            Err(e) => assert_eq!("invalid character !", format!("{}",e)),
            _ => panic!(),
        }
    }
}
