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

/// FormattedOrganisationsnummer holds two formats of a normalized organization number, one long and
/// one short format.
pub struct FormattedOrganisationsnummer {
    long: String,
    short: String,
}

impl FormattedOrganisationsnummer {
    /// Returns the long format of a formatted organization number with separator as a String.
    pub fn long(&self) -> String {
        self.long.clone()
    }

    /// Returns the short format of a formatted organization number without separator as a String.
    pub fn short(&self) -> String {
        self.short.clone()
    }
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

        let mut number = org.to_string().replace("-", "");

        // May only be prefixed with 16.
        if prefix != 0 {
            if prefix != 16 {
                return Err(OrganisationsnummerError::InvalidInput);
            } else {
                number = number[2..].to_string();
            }
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
        if !luhn(number.clone()) {
            return Err(OrganisationsnummerError::InvalidInput);
        }

        return Ok(Organisationsnummer {
            personnummer: None,
            number: number.clone(),
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
    pub fn format(&self) -> FormattedOrganisationsnummer {
        let formatted = match &self.personnummer {
            Some(pnr) => {
                let f = pnr.format();
                let s = f.short();

                let mut l = f.long();
                if pnr.get_age() >= 100 {
                    l = l.replace("-", "+");
                }

                FormattedOrganisationsnummer {
                    long: l[2..].to_string(),
                    short: s[0..6].to_string() + &s[7..].to_string(),
                }
            },
            None => FormattedOrganisationsnummer {
                long: format!("{}-{}", &self.number[..6], &self.number[6..]),
                short: self.number.clone(),
            },
        };

        formatted
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
    use reqwest::blocking::get;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct TestItem {
        input: String,
        long_format: String,
        short_format: String,
        r#type: String,
        vat_number: String,
        valid: bool,
    }

    fn get_test_list() -> Vec<TestItem> {
        let res = get(
            "https://raw.githubusercontent.com/organisationsnummer/meta/main/testdata/list.json",
        )
        .unwrap();
        let list = res.json::<Vec<TestItem>>().unwrap();
        list
    }

    #[test]
    fn test_invalid_input() {
        let list = get_test_list();

        for item in list {
            if item.valid {
                continue;
            }

            assert!(Organisationsnummer::parse(item.input.as_str()).is_err());
        }
    }

    #[test]
    fn test_valid_organization_numbers() {
        let list = get_test_list();

        for item in list {
            if !item.valid {
                continue;
            }

            assert!(Organisationsnummer::parse(item.input.as_str())
                .unwrap()
                .valid());
        }
    }

    #[test]
    fn test_valid_organization_types() {
        let list = get_test_list();

        for item in list {
            if !item.valid {
                continue;
            }

            assert_eq!(
                Organisationsnummer::parse(item.input.as_str())
                    .unwrap()
                    .r#type(),
                item.r#type,
            );
        }
    }

    #[test]
    fn test_valid_format_with_separator() {
        let list = get_test_list();

        for item in list {
            if !item.valid {
                continue;
            }

            assert_eq!(
                Organisationsnummer::parse(item.input.as_str())
                    .unwrap()
                    .format()
                    .long(),
                item.long_format,
            );
        }
    }

    #[test]
    fn test_valid_format_without_separator() {
        let list = get_test_list();

        for item in list {
            if !item.valid {
                continue;
            }

            assert_eq!(
                Organisationsnummer::parse(item.input.as_str())
                    .unwrap()
                    .format()
                    .short(),
                item.short_format,
            );
        }
    }

    #[test]
    fn test_valid_vat_numbers() {
        let list = get_test_list();

        for item in list {
            if !item.valid {
                continue;
            }

            assert_eq!(
                Organisationsnummer::parse(item.input.as_str())
                    .unwrap()
                    .vat_number(),
                item.vat_number
            );
        }
    }

    #[test]
    fn test_valid_personal_identity_numbers() {
        let list = get_test_list();

        for item in list {
            if !item.valid {
                continue;
            }

            if item.r#type != "Enskild firma" {
                continue;
            }


            assert!(Organisationsnummer::parse(item.long_format.as_str())
                .unwrap()
                .valid());

                let org = Organisationsnummer::parse(item.input.as_str()).unwrap();
            assert!(org.valid());
            assert_eq!(org.format().short(), item.short_format);
            assert_eq!(org.format().long(), item.long_format);
            assert!(org.is_personnummer());
            assert!(org.valid());
            assert_eq!(org.vat_number(), item.vat_number);
        }
    }
}
