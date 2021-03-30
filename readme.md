# Tycho Binary Format
A minimal, self-describing and traversable data format designed around rust and the serde data model.

### Design Features
- Small storage sizes
- Traversable
- Self describing
- Map to Rust/Serde data model
- Support for 128-bit numbers
- Easy/Fast to encode/decode
- Ability to contain a single non-terminating value.


[Specification]()

### Examples
```
--- JSON ---
{ "foo": "Hello World", "bar": 10, "baz": true }
------ JSON BYTES ------
7b 22 66 6f 6f 22 3a 22 48 65 6c 6c 6f 20 57 6f 72 6c 64 22 2c 22 62 61 72 22 3a 31 30 2c 22 62 61 7a 22 3a 74 72 75 65 7d
------ BSON ------
29 00 00 00 02 66 6f 6f 00 0c 00 00 00 48 65 6c 6c 6f 20 57 6f 72 6c 64 00 10 62 61 72 00 0a 00 00 00 08 62 61 7a 00 01 00
------ TYCHO ------
05 21 62 61 7a 00 01 01 01 62 61 72 00 01 04 01 0a 66 6f 6f 00 01 02 0b 48 65 6c 6c 6f 20 57 6f 72 6c 64
------ TYCHO VERBOSE -----
Struct({"baz": Value(Boolean(true)), "bar": Value(Number(Unsigned8(10))), "foo": Value(String("Hello World"))})
```


### Deconstruction
```
05 - Structure
    21 - (33 bytes length)
    [Field 0]
        66 - "f"
        6f - "o"
        6f - "o"
        00 - NULL/EOS
        01 - Value
            02 - String
                0b - (11 bytes length)
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
    [Field 1]
        62 - "b"
        61 - "a"
        72 - "r"
        00 - NULL/EOS
        01 - Value
            04 - Number
                01 - Unsigned 8
                    10 - #10
    [Field 2]
        62 - "b"
        61 - "a"
        7a - "z"
        00 - NULL/EOS
        01 - Value
            01 - Boolean
                01 - true
```