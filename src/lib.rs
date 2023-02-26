#[macro_use]
extern crate lazy_static;

use personnummer::{Personnummer, PersonnummerError};
use regex::{Match, Regex};
use std::{convert::TryFrom, error::Error, fmt};

lazy_static! {
    static ref ORG_REGEX: Regex =
        Regex::new(r"(?x)^(\d{2}){0,1}(\d{2})(\d{2})(\d{2})([-+]?)?(\d{3})(\d)$").unwrap();
}

#[derive(Debug)]
pub enum OrganisationsnummerError {
    InvalidInput,
}

impl fmt::Display for OrganisationsnummerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OrganisationsnummerError::InvalidInput => write!(f, "Invalid format"),
        }
    }
}

impl Error for OrganisationsnummerError {}

#[allow(dead_code)]
/// Organisationsnummer holds relevant data to check for valid organization numbers.
pub struct Organisationsnummer {
    personnummer: Option<Personnummer>,
    number: String,
}

impl TryFrom<&str> for Organisationsnummer {
    type Error = OrganisationsnummerError;

    fn try_from(org: &str) -> Result<Self, OrganisationsnummerError> {
        let caps = ORG_REGEX
            .captures(org)
            .ok_or(OrganisationsnummerError::InvalidInput)?;

        let match_to_u32 =
            |m: Option<Match<'_>>| -> u32 { m.unwrap().as_str().parse::<u32>().unwrap_or(0) };

        let prefix = match caps.get(1) {
            Some(m) => match_to_u32(Some(m)),
            None => 0,
        };

        // May only be prefixed with 16.
        if prefix != 0 && prefix != 16 {
            return Err(OrganisationsnummerError::InvalidInput);
        }

        let third = match_to_u32(caps.get(3));

        // Third digit bust be more than 20.
        if third < 20 {
            return Err(OrganisationsnummerError::InvalidInput);
        }

        let second = match_to_u32(caps.get(2));

        // Second digit may not start with leading 0.
        if second < 10 {
            return Err(OrganisationsnummerError::InvalidInput);
        }

        // Luhn checksum must be valid.
        if !luhn(org.to_string().replace("-", "")) {
            return Err(OrganisationsnummerError::InvalidInput);
        }

        return Ok(Organisationsnummer {
            personnummer: None,
            number: org.to_string().replace("-", ""),
        });
    }
}

impl Organisationsnummer {
    /// Returns a new instance of a Organisationsnummer.
    pub fn new(org: &str) -> Result<Organisationsnummer, OrganisationsnummerError> {
        match Personnummer::new(org) {
            Ok(pnr) => Ok(Organisationsnummer {
                personnummer: Some(pnr),
                number: org.to_string().replace("-", ""),
            }),
            Err(_) => Organisationsnummer::try_from(org),
        }
    }

    /// Same as new().
    pub fn parse(org: &str) -> Result<Organisationsnummer, OrganisationsnummerError> {
        Organisationsnummer::new(org)
    }

    /// Validate a Organisationsnummer. The validation requires a valid Luhn checksum.
    pub fn valid(&self) -> bool {
        true
    }

    /// Format organization number with or without separator.
    pub fn format(&self, separator: Option<bool>) -> String {
        let number = match &self.personnummer {
            Some(pnr) => pnr.format().long()[2..13].to_string().replace("-", ""),
            None => self.number.clone(),
        };

        if separator.unwrap_or(true) {
            return format!("{}-{}", &number[..6], &number[6..]);
        }

        number.clone()
    }

    /// Get the organization type.
    pub fn r#type(&self) -> String {
        let first = match &self.personnummer {
            Some(_) => 0,
            None => self
                .number
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap_or(0),
        };

        let r#type = match first {
            0 => "Enskild firma",
            1 => "Dödsbon",
            2 => "Stat, landsting, kommun eller församling",
            3 => {
                "Utländska företag som bedriver näringsverksamhet eller äger fastigheter i Sverige"
            }
            5 => "Aktiebolag",
            6 => "Enkelt bolag",
            7 => "Ekonomisk förening eller bostadsrättsförening",
            8 => "'Ideella förening och stiftelse",
            9 => "Handelsbolag, kommanditbolag och enkelt bolag",
            _ => "Okänt",
        };

        r#type.to_string()
    }

    /// Get organization vat number.
    pub fn vat_number(&self) -> String {
        let number = match &self.personnummer {
            Some(pnr) => pnr.format().long()[2..13].to_string().replace("-", ""),
            None => self.number.clone(),
        };

        format!("SE{}01", number)
    }

    /// Get Personnummer instance.
    pub fn personnummer(&self) -> Result<Personnummer, PersonnummerError> {
        Personnummer::new(&self.number)
    }

    /// Determine if personnummer or not.
    pub fn is_personnummer(&self) -> bool {
        match &self.personnummer {
            Some(_) => true,
            None => false,
        }
    }
}

/// Determine if the checksum based on luhn algorithm is valid. See more information here:
/// https://en.wikipedia.org/wiki/Luhn_algorithm.
fn luhn(value: String) -> bool {
    let checksum = value
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0))
        .enumerate()
        .fold(0, |acc, (idx, v)| {
            let value = if idx % 2 == 0 { v * 2 } else { v };
            acc + if value > 9 { value - 9 } else { value }
        });

    (10 - (checksum as u8 % 10)) % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_invalid_input() {
        let cases = vec!["556016-0681", "556103-4250", "5561034250"];

        for tc in cases {
            assert!(Organisationsnummer::parse(tc).is_err());
        }
    }

    #[test]
    fn test_valid_organization_numbers() {
        let cases = vec!["556016-0680", "556103-4249", "5561034249", "559244-0001"];

        for tc in cases {
            assert!(Organisationsnummer::parse(tc).unwrap().valid());
        }
    }

    #[test]
    fn test_valid_limited_companies() {
        let cases = vec!["556016-0680", "556103-4249", "5561034249", "559244-0001"];

        for tc in cases {
            assert_eq!(
                Organisationsnummer::parse(tc).unwrap().r#type(),
                "Aktiebolag"
            );
        }
    }

    #[test]
    fn test_valid_format_with_separator() {
        let cases = HashMap::from([
            ("559244-0001", "559244-0001"),
            ("556016-0680", "556016-0680"),
            ("556103-4249", "556103-4249"),
            ("5561034249", "556103-4249"),
        ]);

        for (key, value) in cases {
            assert_eq!(Organisationsnummer::parse(key).unwrap().format(None), value);
        }
    }

    #[test]
    fn test_valid_format_without_separator() {
        let cases = HashMap::from([
            ("559244-0001", "5592440001"),
            ("556016-0680", "5560160680"),
            ("556103-4249", "5561034249"),
            ("5561034249", "5561034249"),
        ]);

        for (key, value) in cases {
            assert_eq!(
                Organisationsnummer::parse(key).unwrap().format(Some(false)),
                value
            );
        }
    }

    #[test]
    fn test_valid_vat_numbers() {
        let cases = HashMap::from([
            ("559244-0001", "SE559244000101"),
            ("556016-0680", "SE556016068001"),
            ("556103-4249", "SE556103424901"),
            ("5561034249", "SE556103424901"),
        ]);

        for (key, value) in cases {
            assert_eq!(Organisationsnummer::parse(key).unwrap().vat_number(), value);
        }
    }

    #[test]
    fn test_valid_personal_identity_numbers() {
        let cases = HashMap::from([("121212121212", "121212-1212")]);

        for (key, value) in cases {
            let key_org = Organisationsnummer::parse(key).unwrap();
            assert!(key_org.valid());
            assert_eq!(key_org.format(Some(false)), value.replace("-", ""));
            assert_eq!(key_org.format(None), value);
            assert!(key_org.is_personnummer());
            assert!(Organisationsnummer::parse(value).unwrap().valid());
            assert!(key_org.personnummer().unwrap().valid());
        }
    }

    #[test]
    fn test_valid_personal_identity_numbers_vat_number() {
        let cases = HashMap::from([
            ("121212121212", "SE121212121201"),
            ("12121212-1212", "SE121212121201"),
        ]);

        for (key, value) in cases {
            assert_eq!(Organisationsnummer::parse(key).unwrap().vat_number(), value);
        }
    }
}
