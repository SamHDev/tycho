# Tycho Specification
This document describes the syntax/grammar for the Tycho binary format version 4.0.

### Bytes
The binary format uses octets (8-bits) and quartets (4-bits)
```
0xA0          An octet of value 10
0xA           A quartet of value 10
0xA 0xB       Two quartet's as a single byte (0xAB)
              (consecutive quartets form a byte)
```
As stated above, two consecutive quartets will become one octet.

### Defined Types
The following types are basic/primitive types used within the specification.
```
byte            1 byte (8-bits)
unsigned8       1 byte (8-bit unsigned integer)
unsigned16      2 bytes (16-bit unsigned integer, big-endian)
unsigned32      4 bytes (32-bit unsigned integer, big-endian)
unsigned64      8 bytes (64-bit unsigned integer, big-endian)
unsigned128     16 bytes (128-bit unsigned integer, big-endian)
signed8         1 byte (8-bit signed integer, 2's compelement)
signed16        2 bytes (16-bit signed integer, big-endian, 2's compelement)
signed32        4 bytes (32-bit signed integer, big-endian, 2's compelement)
signed64        8 bytes (64-bit signed integer, big-endian, 2's compelement)
signed128       16 bytes (128-bit signed integer, big-endian, 2's compelement)
float32         4 bytes  ("binary32" IEEE 754-2008, big-endian)
float64         8 bytes  ("binary64" IEEE 754-2008, big-endian)
char            1-6 bytes (UTF-8 character, big-endian)
varlength       1-4 bytes (A variable length unsigned number, representing a length. [See Below])
```

#### Variable Lengths
Variable lengths are used to keep document sizes down when specifying strings, lists, structures, maps and arrays.
Rather than using a fix-length number, a variable length can be between 1-4 bytes, only using a byte when needed.

This implementation of variable length numbers prefixes the length of the number (in bytes) within the 2 bits.
This allows for 6-30 bits to be used for data with a maximum value of 1073741823.

##### Diagram
```
OCTETS       0                       1                    2                       3  
BITS         0  1  2  3  4  5  6  7  8  9 10 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
LENGTH  ::=  LEN   DATA              DATA (LEN > 0)       DATA (LEN > 1)          DATA (LEN > 2)
          |  
          | LEN       A 2 bit unsigned number representing the number of following bytes
          | DATA      A 6-30 but unsigned number representing the length of the following string/array/map
```

##### Example Code
```python
# Todo
```

##### Examples
| Numerical | Length | Bytes                        | Hex                          |
| --------- | -------| ---------------------------- | ---------------------------- |
| 1         | 1      | `1`                          | `0x01`                       |
| 64        | 2      | `64`, `64`                   | `0x4040`                     |
| 255       | 2      | `64`, `255`                  | `0x40FF`                     |
| 1024      | 2      | `68`, `0`                    | `0x40FF`                     |
| 42069     | 3      | `128`, `164`, `85`           | `0x80A455`                   |
| 42069420  | 4      | `194`, `129`, `237`, `172`   | `0xC281EDAC`                 |


### Specification
A Specification of the format, using the [defined types](#Defined-Types) and [byte notation](#Bytes) as seen above.
#### Grammar
```
0x01                            Literial Byte
01                              Literial Nibble
unsigned8                       A type (defined in 'defined types' or within the spec)
*byte                           An array of a type
item(ident)                     A type with a paramater
value ::= 0x01 value            Type definition
value(x:ident) ::= 0x01 value   Type definition with a paramter (where x is the name, and ident is the type)
```

#### Specification
```
value          ::= ident data(ident)

data(i:ident)  ::= i=0x0 0x00                        Boolean False
                 | i=0x0 0x01                        Boolean True
                 | i=0x1 unsigned8                   8-bit unsigned integer
                 | i=0x2 unsigned16                  16-bit unsigned integer
                 | i=0x3 unsigned32                  32-bit unsigned integer
                 | i=0x4 unsigned64                  64-bit unsigned integer
                 | i=0x5 unsigned128                 128-bit unsigned integer
                 | i=0x6 signed8                     8-bit signed integer
                 | i=0x7 signed16                    16-bit signed integer
                 | i=0x8 signed32                    32-bit signed integer
                 | i=0x9 signed64                    64-bit signed integer
                 | i=0xA signed128                   128-bit signed integer
                 | i=0xB float32                     32-bit floating point number
                 | i=0xC float64                     64-bit floating point number
                 | i=0xD string                      An UTF-8 string value
                 | i=0xE char                        A single UTF-8 character
                 | i=0xF bytes                       A array of octets

ident          ::= nibble                            A 

string         ::= varlength *byte                   An UTF-8 string, where (*byte) is an array of octets with the
                 |                                   length specified with the varlength

bytes          ::= varlength *byte                   An array of bytes, where (*byte) is an array of octets with the
                 |                                   length specified with the varlength

tstring        ::= *byte 0x00                        A terminating UTF-8 string, where (*byte) is an array of octets.
                                                     (Deprecated)

element        ::= 0x0 0x0                           Unit (Null/Nil) - No data
                 | 0x1 value                         A primitive, terminating value
                 | 0x2 0x0                           Optional - None
                 | 0x2 0x1 element                   Optional - Some
                 | 0x3 0x0 varlength *element        An array of elements with length given by the preceding varlength
                 | 0x4 0x0 varlength *field          A structure, a variable type key-element map, defined by an array
                 |                                   of field with length given by the preceding varlength
                 | 0x5 0x0 string element            A variable element type with a given name.
                 | 0x6 ident varlength *item(ident)
                 | 0x7 ident varlength *data(ident)

field          ::= string element                    A structure field, containg a key (string) and value (element)

item(x:ident)  ::= data(x) element                   A map item, containg a key (data/value) and value (element) 

```
