# teeconfig

[![Version](https://img.shields.io/crates/v/teeconfig)](https://crates.io/crates/teeconfig)
[![Downloads](https://img.shields.io/crates/d/teeconfig)](https://crates.io/crates/teeconfig)
[![License](https://img.shields.io/crates/l/teeconfig)](https://crates.io/crates/teeconfig)
![Rust](https://github.com/edg-l/teeconfig/workflows/Rust/badge.svg)
[![Docs](https://docs.rs/teeconfig/badge.svg)](https://docs.rs/teeconfig)

A ddnet / teeworlds configuration parser.

It recognizes the available options by parsing source files with `MACRO_CONFIG_` lines such as `src/engine/shared/config_variables.h`.


```rust
use teeconfig::parse_config_variables;

let header_source = include_str!("../config_variables.h");
let vars = parse_config_variables(header_source).unwrap();
assert!(!vars.is_empty())
```
