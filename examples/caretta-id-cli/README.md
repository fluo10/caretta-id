# caretta-id-cli
Reference tool to generate/encode/decode [caretta-id](https://github.com/fluo10/caretta-id).

## Installation

```
cargo install caretta-id-cli
```

## Usage

```
Reference tool to generate/encode/decode caretta-id

Usage: caretta-id-cli <COMMAND>

Commands:
  decode     Decode caretta-id string to integer
  encode     Encode integer to caretta-id string
  generate   (deprecated) Generate random caretta-id
  timestamp  Generate time-based caretta-id
  random     Generate random caretta-id
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Generate new CarettaIdS

```
$ caretta-id-cli random 
123abcd
```

### Generate time-based CarettaId

```
$ caretta=id-cli timestamp --unix
gdw0982
```

### Encode CarettaIdD

```
$ caretta-id-cli encode 0
0000000
```

### Decode CarettaIdQ

```
$ caretta-id-cli decode 0000000
0
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
