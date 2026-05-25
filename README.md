# MathRC
[![CI](https://github.com/as7ar/mathrc/actions/workflows/rust.yml/badge.svg)](https://github.com/as7ar/mathrc/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/mathrc.svg)](https://crates.io/crates/mathrc)
[![Docs.rs](https://docs.rs/mathrc/badge.svg)](https://docs.rs/mathrc)
[![Downloads](https://img.shields.io/crates/d/mathrc.svg)](https://crates.io/crates/mathrc)
[![Rust](https://img.shields.io/badge/rust-1.95.0%2B-orange.svg)](https://www.rust-lang.org)

MathRC is a lightweight Rust mathematics library that provides basic mathematical utilities, fraction handling, functional analysis, and a simple arithmetic expression calculator.

## Installation

Add MathRC to your `Cargo.toml`:
```toml
[dependencies]
mathrc = "(latest version)"
```
Or, if you want to use the Git repository directly:
```toml
[dependencies]
mathrc = { git = "https://github.com/as7ar/mathrc" }
```

## Status

MathRC is currently in early development. APIs may change as the library grows.

## License

This project is licensed under the GPL-3.0 license.

## Change Log
### Fixed
1. Fixed the issue where variables moved after Frac normalization
2. Fixed the issue where variables moved after Func derivative/integral