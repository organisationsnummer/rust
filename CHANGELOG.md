# Changelog

## 1.1.0

- Published as `organisationsnummer-rs`. This crate replaces `organisationsnummer`, which could not be updated after the original owner passed away. To migrate, change the dependency in `Cargo.toml`. The library is still imported as `organisationsnummer` and the API is unchanged.
- Accept `+` as separator when parsing, as for personnummer of people aged 100 or more.
- Fix the type name for organisation group 8 ("Ideella förening och stiftelse").
- Use `std::sync::LazyLock` instead of `lazy_static`. The minimum supported Rust version is now 1.80.
- Publish to crates.io from tags through GitHub Actions with trusted publishing.
