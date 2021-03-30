# Tycho 7 Specification

## Design
The original purpose of the Tycho binary format was a transmission format, aiming to be small while still being
self describing and offing types that respected the language that I worked in. Other binary formats such as BSON and
MessagePack were unable to fulfill this.

However, a new binary format is required for my astra database project, and small transmission sizes, while enticing are not
the main requirement anymore. It makes sense for me to redesign the tycho binary format, to achieve these requirements:

- Simple/Low processing to read.
- Contain all types of Rust/Serde data model
- Support UUIDs
- Partial Traversal for large amounts of data.
- Small enough for viable transmission.
- Compression support

## Specification
### Lengths
In-order to keep structure sizes small, lengths or arrays or prefixed byte sizes use a variable length number to
represent its respective size.

Tycho's Variable Length Numbers are defined as such:
- A number is formed of 8 bit bytes
- The 7 least significant bits of an octet represent the number data.
- The most significant bit, is a logical bit representing if there is another byte.
- The number is represented in little endian format.
- The number is a 32-bit unsigned number (5 encoded bytes max)


Here is an example of the encoding:
```
CONTINUE(Y)         CONTINUE(N)
|  DATA             |  DATA
1  1 0 1 0 1 0 1    0  1 0 0 0 1 1 0
```

Tycho's Variable length encoding is very similar to ProtoBuf's uint32.

### Terminating Strings
Terminating strings (`tstring`) are used in key/name fields. 

Like a CString, it's a UTF-8 encoded string followed by a "terminating" NULL (`0x00`) byte.

> A `tstring` is to not be confused with `string` which is length prefixed.

### Types
Tycho comes with two types of data, **Values** and **Elements**. Values are primitive, terminating data such as
strings, booleans, numbers and byte arrays. Elements are non-terminating data and have the ability to contain other
values and elements.

### Values 
Values contain two components; An **Ident** "prefix" and their respective payload. A value's prefix does not need
to be adjacent to value, in a few cases a parent element can define the Value Ident, and the inner values should
be read accordingly. Value's Idents can be one or two bytes long.

> Please allow for any length of ident within your implementation by using a byte array rather than reading an u16.
 
The payload of a value varies in size should be read in full even when parsing partially.

#### Base Values
| Name | Rust | Ident | Payload | Description |
| ---- | ---- | ----- | ------- | ----------- |
| Null | N/A  | `0x00`  | No data | A null type, used to signify no type or no length. |
| Boolean | bool | `0x01` | 1 byte| A boolean, where the single data byte is 0x00 (false) or 0x01 (true) |
| String | String | `0x02` | `length` `[...bytes]` | A byte length prefixed UTF-8 string.
| Char | car | `0x03` | `[1-6 bytes]` | A UTF-8 encoded char with no terminator or length. |
| Number | ... | `0x04` ... | ... | A number with a given prefix, defined below. |
| Bytes | \[u8\] | `length` `[...bytes]` | An array of bytes with a given length. |
| UUID | uuid::Uuid | `0x06` | `[12 bytes]` | A 128-bit Uuid in big-endian. |

#### Numerical Values
| Name | Rust | Ident | Payload | Description |
| ---- | ---- | ----- | ------- | ----------- |
| Bit | N/A | `0x04` `0x00` | 1 byte | A single unsigned bit. 0x00 or 0x01 matching 0b0 0b1 respectively. In most situations Boolean should be used rather than Bit. |
| Unsigned8 | u8 | `0x04` `0x01` | 1 byte | A unsigned 8 bit number or single octet byte.  |
| Unsigned16 | u16 | `0x04` `0x02` | 2 bytes | A big-endian encoded unsigned 16 bit number.  |
| Unsigned32 | u32 | `0x04` `0x03` | 4 bytes | A big-endian encoded unsigned 32 bit number.  |
| Unsigned64 | u64 | `0x04` `0x04` | 8 bytes | A big-endian encoded unsigned 64 bit number.  |
| Unsigned128 | u128 | `0x04` `0x05` | 16 bytes | A big-endian encoded unsigned 128 bit number.  |
| Signed8 | i8 | `0x04` `0x11` | 1 byte | A two's complement signed 8bit number.  |
| Signed16 | i16 | `0x04` `0x12` | 2 bytes | A big-endian encoded two's complement signed 16 bit number.  |
| Signed32 | i32 | `0x04` `0x13` | 4 bytes | A big-endian encoded two's complement signed 32 bit number.  |
| Signed64 | i64 | `0x04` `0x14` | 8 bytes | A big-endian encoded two's complement signed 64 bit number.  |
| Signed128 | i128 | `0x04` `0x15` | 16 bytes | A big-endian encoded two's complement signed 128 bit number.  |
| Float32 | f32 | `0x04` `0x23` | 4 bytes | A IEEE 754 32 bit floating point number.  |
| Float64 | f64 | `0x04` `0x24` | 8 bytes | A IEEE 754 64 bit floating point number.  |


### Elements
An Element is a non-terminating potential container of elements or values. While elements do have Idents like values,
they must be adjacent to the payload. Hence, they are considered **Prefixes** rather than Idents.


| Name | Rust | Prefix | Data | Description |
| ---- | ---- | ----- | ------- | ----------- |
| Unit | () | `0x00` | No Data | A Unit/Null type containing no data. |
| Value | N/A | `0x01` | `ident`  `payload` | A primitive value containing values defined above with their respective ident. |
| None | Option::None | `0x02` | No Data | An optional element, where some is false, with no inner element. |
| Some | Option::Some(T) | `0x03` | `element` | An optional element, where some is true, with a inner element. |
| Variant | enum | `0x04` | `tstring` `element` | A named variable type. |
| Struct | struct | `0x05` | `size` *{ `tstring` `element` } | A string-value map of elements.
| List | vec | `0x06` | `size`  *{ `element` } | An ordered list of elements.
| Map | HashMap | `0x07` | `ident` `size` *{ `payload` `element` } | A map of values and elements where the value key is type restricted. |
| Array | vec | `0x08` | `ident` `size` *{ `payload` } | A type restricted array of values |
| Compression | N/A | `0xF0` | `size` `[...bytes]` | Gz compressed element. 

> \*1 Size is variable length number representing the size of the payload in bytes, not including itself

> \*2 The `ident` type is a value ident, representing the type of `payload`, which is a value payload.
> If `ident` is of type `Null`, then the element does not contain any data, and hence size or any other data is not present.



### Implementation Tips
- Tycho was designed to read/written recursively.
- Separate parsing of prefixes/idents and payloads for both elements, values and numbers.
- [My development notes](dev_notes.md)
