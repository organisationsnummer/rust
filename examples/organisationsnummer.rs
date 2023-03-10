use personnummer::{Personnummer, PersonnummerError};
use std::env;

fn main() -> Result<(), PersonnummerError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run --example organisationsnummer <organisationsnummer>");
        return Err(PersonnummerError::InvalidInput);
    }

    let org = Personnummer::new(&args[1])?;

    if org.valid() {
        println!(
            "The company with organization number {}",
            org.format().long(),
        );
    } else {
        println!("invalid organization number provided");
    }

    Ok(())
}
