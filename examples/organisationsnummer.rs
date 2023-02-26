use organisationsnummer::{Organisationsnummer, OrganisationsnummerError};
use std::env;

fn main() -> Result<(), OrganisationsnummerError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run --example organisationsnummer <organisationsnummer>");
        return Err(OrganisationsnummerError::InvalidInput);
    }

    let org = Organisationsnummer::new(&args[1])?;

    if org.valid() {
        println!(
            "The company with organization number {} is a {} and the vat number is {}",
            org.format().long(),
            org.r#type(),
            org.vat_number()
        );
    } else {
        println!("invalid organization number provided");
    }

    Ok(())
}