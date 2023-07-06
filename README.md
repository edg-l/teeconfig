# teeconfig

A ddnet / teeworlds configuration parser.

It recognizes the available options by parsing source files with `MACRO_CONFIG_` lines such as `src/engine/shared/config_variables.h`.


```rs
let header_source = include_str!("../config_variables.h");
let vars = parse_config_variables(header_source).unwrap();
assert!(!vars.is_empty())
```
