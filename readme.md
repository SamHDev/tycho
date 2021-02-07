# Tycho Binary Format
A self describing binary format designed around the serde data model.

Used within the [Luna Server](https://github.com/samhdev/luna) project as the main format during transmission.

> #### Links
> [Specification](specification.md)
>
> [Rust Documentation](https://docs.rs/tycho) 
> : :
> [Rust Crate](https://crates.io/crates/tycho)

### Design Targets
- Self describing
- Efficient and Small
- Map to Rust/Serde data model
- Support for 128-bit numbers
- Easy/Fast to encode/decode
- Ability to contain a single non-terminating value.

### Example
```
------ JSON ------
{ "foo": "Hello World", "bar": 10, "baz": true }
------ JSON BYTES ------
7b 22 66 6f 6f 22 3a 22 48 65 6c 6c 6f 20 57 6f 72 6c 64 22 2c 22 62 61 72 22 3a 31 30 2c 22 62 61 7a 22 3a 74 72 75 65 7d
------ BSON ------
29 00 00 00 02 66 6f 6f 00 0c 00 00 00 48 65 6c 6c 6f 20 57 6f 72 6c 64 00 10 62 61 72 00 0a 00 00 00 08 62 61 7a 00 01 00
------ TYCHO ------
40 03 03 66 6f 6f 1d 11 48 65 6c 6c 6f 20 57 6f 72 6c 64 93 62 61 72 12 10 03 62 61 7a 10 01
```

#### Explanation
```
40 - Structure
    03 - Structure of length 3
    (Element 0)
        03 - Key has length 3
            66 - "f"
            6f - "o"
            6f - "o"
        1D - Value is a String
            11 - String has length 11
            48 - "H"
            65 - "e" 
            6c - "l"
            6c - "l"
            6f - "o"
            20 - " "
            57 - "W"
            6f - "o"
            72 - "r"
            6c - "l"
            64 - "d"
    (Element 1)   
        03 - Key has length 3
            62 - "b"
            61 - "a"
            72 - "r"
        12 - Value is an unsigned 8-bit int
            10 - u8(10)
    (Element 2)   
        03 - Key has length 3
            62 - "b"
            61 - "a"
            7a - "z"
        10 - Value is a boolean
            01 - true

40 03 03 66 6f 6f 1d 11 48 65 6c 6c 6f 20 57 6f 72 6c 64 93 62 61 72 12 10 03 62 61 7a 10 01
```