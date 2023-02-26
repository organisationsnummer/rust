# Organisationsnummer

[![Crates.io](https://img.shields.io/crates/v/organisationsnummer.svg)](https://crates.io/crates/organisationsnummer)
[![Rust](https://github.com/organisationsnummer/rust/actions/workflows/rust.yml/badge.svg)](https://github.com/organisationsnummer/rust/actions/workflows/rust.yml)

Validate Swedish organization numbers with [Rust](https://www.rust-lang.org/). Follows version 1.1 of the [specification](https://github.com/organisationsnummer/meta#package-specification-v11).

## Usage

```rust
use organisationsnummer::Organisationsnummer;

fn main() {
    match Organisationsnummer::new("202100-5489") {
        Ok(pnr) => println!("{}: {}", org.format().long(), org.valid()),
        Err(e) => panic!("Error: {}", e),
    }
}
```

Fore more details, see [examples](examples) and/or run

```sh
$ cargo run --example organisationsnummer <organisationsnummer>
```

## License

MIT
