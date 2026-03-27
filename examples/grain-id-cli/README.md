# grain-id-cli
Reference tool to generate/encode/decode [grain-id](https://github.com/fluo10/grain-id).

## Installation

```
cargo install grain-id-cli
```

## Usage

```
Reference tool to generate/encode/decode grain-id

Usage: grain-id-cli <COMMAND>

Commands:
  decode     Decode grain-id string to integer
  encode     Encode integer to grain-id string
  generate   (deprecated) Generate random grain-id
  timestamp  Generate time-based grain-id
  random     Generate random grain-id
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Generate new GrainIdS

```
$ grain-id-cli random 
123abcd
```

### Generate time-based GrainId

```
$ grain-id-cli timestamp --unix
gdw0982
```

### Encode GrainIdD

```
$ grain-id-cli encode 0
0000000
```

### Decode GrainIdQ

```
$ grain-id-cli decode 0000000
0
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
