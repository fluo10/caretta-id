# caretta-id Specification

## Overview

caretta-id is a human-friendly unique identifier format designed for readability, memorability, and compactness. It encodes integer values into a 7-character string using a custom BASE32 encoding. caretta-id is suitable for use in distributed systems, logging, URLs, and other contexts where short, unique identifiers are beneficial.

### Motivation

When considering implementing IDs for users (not for internal systems) to specify items, such as GitHub commit hashes or issue numbers in a distributed system using P2P, the following issues arose.

- Sequential numbers like Git issues are difficult to implement in distributed systems because collisions are unavoidable.
- Existing random identifiers like UUID are too long for users.
- Short random numbers like 7-digit commit hashes seem good but they are not standardized specifications.

So I decided to make my own ID specification.

## Structure

A caretta-id consists of exactly 7 characters encoded with a custom BASE32 alphabet.

### Format

- Exactly 7 characters, no separators.
- Characters are selected from a custom BASE32 alphabet (see Encoding section).
- Characters are ordered from most significant to least significant.

### Why 7 Characters?

7 characters was chosen for the following reasons:

- Git's short commit IDs conventionally use 7 characters, making the length familiar to developers.
- 7 BASE32 characters encode 35 bits, which provides approximately 1090 years of range at Unix second precision, or approximately 109 years at decisecond (0.1s) precision. Decisecond precision is sufficient for uniqueness in personal-use applications, and a range exceeding 100 years before wrap-around is adequate for the vast majority of users. (And if it does wrap around, that would be quite the occasion.)

### Examples

- `0000000`
- `123abcd`
- `zzzzzzz`

## Encoding/Decoding

caretta-id uses a custom BASE32 encoding/decoding to convert integer values into/from character strings.

### Alphabet

- Based on [Crockford's Base32](https://www.crockford.com/base32.html).
- In encoding, visually ambiguous characters `i`, `l`, `o`, and `u` are excluded.
- In decoding, visually ambiguous characters work as aliases of their canonical character.

| value | Encode | Decode aliases     |
|------:|:------:|:------------------:|
|     0 |      0 | 0, o, O            |
|     1 |      1 | 1, i, I, l, L      |
|     2 |      2 | 2                  |
|     3 |      3 | 3                  |
|     4 |      4 | 4                  |
|     5 |      5 | 5                  |
|     6 |      6 | 6                  |
|     7 |      7 | 7                  |
|     8 |      8 | 8                  |
|     9 |      9 | 9                  |
|    10 |      a | a, A               |
|    11 |      b | b, B               |
|    12 |      c | c, C               |
|    13 |      d | d, D               |
|    14 |      e | e, E               |
|    15 |      f | f, F               |
|    16 |      g | g, G               |
|    17 |      h | h, H               |
|    18 |      j | j, J               |
|    19 |      k | k, K               |
|    20 |      m | m, M               |
|    21 |      n | n, N               |
|    22 |      p | p, P               |
|    23 |      q | q, Q               |
|    24 |      r | r, R               |
|    25 |      s | s, S               |
|    26 |      t | t, T               |
|    27 |      v | v, V, u, U         |
|    28 |      w | w, W               |
|    29 |      x | x, X               |
|    30 |      y | y, Y               |
|    31 |      z | z, Z               |

### Bit Width

- Each character encodes exactly 5 bits (BASE32).
- 7 characters encode 35 bits.
- Maximum representable value: 2^35 - 1 = 34,359,738,367

### Encoding Process

1. Mask the integer value to 35 bits.
2. Extract each of the 7 five-bit groups from most significant to least significant.
3. Map each 5-bit value to the corresponding character in the encode alphabet.

### Decoding Process

1. Verify the string is exactly 7 characters.
2. Map each character to its 5-bit value using the decode alias table.
3. Combine the 7 five-bit values into a 35-bit integer.

## Examples

| Integer         | caretta-id  |
|----------------:|:-----------:|
|             `0` | `0000000`   |
|           `255` | `000007z`   |
|   `34359738367` | `zzzzzzz`   |

## Implementation Notes

- caretta-id is language-agnostic and can be implemented in any language with integer and string manipulation capabilities.
- The Rust implementation provides a `CarettaId` struct backed by a `u64` with common conversion traits.
- Parsing and formatting functions should validate character set and string length.
- Lossy conversion from oversized integers is allowed. In this case, higher bits are discarded (modulo 2^35).

## License

This specification is open and free to use under the same license as the caretta-id reference implementation.
