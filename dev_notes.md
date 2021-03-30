
### Syntax
```
"\x00" 					A literial byte (with value 0)
FOO						A type reference
FOO ::=					A type definition
BAR(X)					A type referance with a type paramater.
BAR(X) ::=				A type definition with a paramater.
byte					A single data byte
4bytes					4 data bytes.
SIZE { FOO }            Variable length number representing the size/length of the contained bytes.
*FOO					Multiple of one item.
```

### Specification
```
IDENT	  	::= "\x00" 						null
			  | "\x01"						boolean
			  | "\x02"						string
			  | "\x03"						char
			  | "\x04" NUMIDENT				number
			  | "\x05"						bytes
			  | "\x06"						uuid

NUMIDENT    ::= "\x00" 						bit
			  | "\x01"						u8
			  | "\x02"						u16
			  | "\x03" 						u32
			  | "\x04"						u64
			  | "\x05"						u128
			  | "\x11"						i8
			  | "\x12"						i16
			  | "\x13"						i32
			  | "\x14"						i64
			  | "\x15"						i128
			  | "\x23"						f32
			  | "\x24"						f64
			  | "\x25"						decimal128

DATA(IDENT) ::= X="\x00"					null
			  | X="\x01" "\x00"         	boolean false
			  | X="\x01" "\x01"         	boolean true
			  | X="\x02" STRING				utf-8 string
			  | X="\x03" *bytes       		utf-8 char
			  | X=["\x04" NUMIDENT] NUMBER	number
			  | X="\x05" SIZE {*byte }    	bytes
			  | X="\x06" 16bytes			uuid

NUMBER(X)   ::= X="\x00" byte 				bit
			  | X="\x01" byte				u8
			  | X="\x02" 2bytes				u16
			  | X="\x03" 4bytes				u32
			  | X="\x04" 8bytes 			u64
			  | X="\x05" 16bytes			u128
			  | X="\x11" byte    			i8
			  | X="\x12" 2bytes				i16
			  | X="\x13" 4bytes 			i32
			  | X="\x14" 8bytes				i64
			  | X="\x15" 16bytes			i128
			  | X="\x23" 4bytes 			f32
			  | X="\x24" 8bytes 			f64
			  | X="\x25" 16bytes			decimal128

VALUE       ::= IDENT DATA(IDENT)

ELEMENT     ::= "\x00" 					    unit
			  | "\x01" VALUE	    		value
			  | "\x02" 						none
			  | "\x03" ELEMENT 				some
			  | "\x04" TSTRING ELEMENT    	variant
			  | "\x05" SIZE{ *FIELD }     	struct
			  | "\x06" SIZE{ *ELEMENT }   	list
			  | "\x07" TYPE SIZE{ DATA }  	array
			  | "\x08" TYPE SIZE{ *PAIR } 	map
			  | "\xF0" SIZE *bytes			compression container

FIELD 	  	::= TSTRING ELEMENT				struct field
PAIR(TYPE)	::= DATA(TYPE) ELEMENT			map pair
			
TSTRING		::= *byte "\x00"				a cstring
STRING		::= SIZE { *byte }				
```


### Numbers
```
---- constants ----
NUM_FLOAT = 0x20
NUM_SIGNED = 0x10
NUM_LEN_1 = 0x00
NUM_LEN_8 = 0x01
NUM_LEN_16 = 0x02
NUM_LEN_32 = 0x03
NUM_LEN_64 = 0x04
NUM_LEN_128 = 0x05
NUM_LEN_256 = 0x06

---- defs ----
u8 = NUM_LEN_8
u16 = NUM_LEN_16
u32 = NUM_LEN_32
u64 = NUM_LEN_64
u128 = NUM_LEN_128
i8 = NUM_LEN_8 | NUM_SIGNED
i16 = NUM_LEN_16 | NUM_SIGNED
i32 = NUM_LEN_32 | NUM_SIGNED
i64 = NUM_LEN_64 | NUM_SIGNED 
i128 = NUM_LEN_128 | NUM_SIGNED
f32 = NUM_LEN_32 | NUM_FLOAT
f64 = NUM_LEN_64 | NUM_FLOAT
decimal128 = NUM_LEN_128 | NUM_FLOAT

---- math ----
function encode_number(number)
	var bytes = number.into_bytes()
	var prefix = log2(bytes.length)
	
	if number is signed then
		prefix =| 0x10
	else if number is float then
		prefix =! 0x20
	end if
	
	return prefix .. bytes
end function
```
