# `toml2json` and `json2toml`

This repository houses the command line tools `toml2json` and `json2toml` which can be used to convert a (configuration) file from TOML to JSON or vice-versa.

A popular use is to maintain a configuration file for an app that consumes (for example) JSON configuration files in a configuration language that you prefer or are more comfortable with (say TOML).

The tool tries to round-trip between the formats losslessly where possible.

## Installation

The tool can currently be installed via the `cargo` package manager:

```
cargo install toml2json
```

## License

This utility is developed by Mahmoud Al-Qudsi of NeoSmart Technologies and is provided to the public free of charge with the hope that it might be beneficial, but without warranty. The source is released under the MIT public license.
