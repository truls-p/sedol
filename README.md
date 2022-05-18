[![Crates.io](https://img.shields.io/crates/v/sedol.svg)](https://crates.io/crates/sedol)
[![Workflow Status](https://github.com/truls-p/sedol/workflows/main/badge.svg)](https://github.com/truls-p/sedol/actions?query=workflow%3A%22main%22)

# sedol

SEDOL

Crate to validate SEDOLs.

<https://en.wikipedia.org/wiki/SEDOL>

<https://www.lseg.com/markets-products-and-services/data-analytics/data-solutions/sedol/documentation>

## Examples
```rust
let sedol_string = "BD9MZZ7";
match sedol::validate(sedol_string) {
   Ok(s) => println!("SEDOL validated: {}", s),
   Err(e) => eprint!("{}", e),
}

let invalid_sedol_string = "BD9MZZ6";
match sedol::validate(invalid_sedol_string) {
   Ok(s) => println!("SEDOL validated: {}", s),
   Err(e) => eprintln!("{}", e),
}

let unclean_sedol_string = " BD9-MZ-Z7?";
match sedol::validate(&sedol::clean(unclean_sedol_string)) {
   Ok(s) => println!("SEDOL validated: {}", s),
   Err(e) => eprintln!("{}", e),
}

let sedol_6_string = "BD9MZZ";
println!("SEDOL with calculated check digit: {}{}", sedol_6_string, sedol::calc_check_digit(sedol_6_string));
```

Current version: 0.1.0

License: MIT OR Apache-2.0
