# msrv-bump

Finds MSRV bumps in packages according to the `rust-version` field. Uses the [crates.io](https://crates.io/) API.

Usage:
```
$ cargo run toml
0.8.8 -> 0.8.9: Some("1.67") -> Some("1.69")
0.8.2 -> 0.8.3: Some("1.66.0") -> Some("1.67")
0.7.6 -> 0.7.7: Some("1.64.0") -> Some("1.66.0")
0.7.3 -> 0.7.4: Some("1.60.0") -> Some("1.64.0")
0.5.11 -> 0.6.0: Some("1.48.0") -> Some("1.60.0")
0.5.9 -> 0.5.10: None -> Some("1.48.0")
```
