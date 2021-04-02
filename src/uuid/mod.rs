use std::fmt;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
/// A simple UUID (v4) implementation.
pub struct Uuid(u128);

impl Uuid {
    /// Create a new uuid (any version) using a random number
    pub fn new() -> Self {
        Self(rand::random())
    }

    /// Create a new v4 (typed) uuid with a random number.
    pub fn v4() -> Self {
        Self((
            rand::random::<u128>()
                & 0xffffffff_ffff_0fff_ffff_ffffffffffffu128)
            | 0x00000000_0000_4000_0000_000000000000u128)
    }

    /// Get the hex representation of the uuid (no hyphens)
    ///
    /// ```
    /// use tycho::Uuid;
    /// let uuid = Uuid::from_numeric(62911151285467627956781226334379231326);
    ///
    /// assert_eq!(uuid.hex(), "2f543f3c06594e9233b0c8a85c2ac85e");
    /// ```
    pub fn hex(&self) -> String {
        format!("{:032x?}", self.0)
    }

    /// Get the formated hex representation of the uuid (with hyphens)
    ///
    /// ```
    /// use tycho::Uuid;
    /// let uuid = Uuid::from_numeric(62911151285467627956781226334379231326);
    ///
    /// assert_eq!(uuid.string(), "2f543f3c-0659-4e92-33b0-c8a85c2ac85e");
    /// ```
    pub fn string(&self) -> String {
        let mut hex = self.hex();
        hex.insert(8, '-');
        hex.insert(13, '-');
        hex.insert(18, '-');
        hex.insert(23, '-');
        hex
    }

    /// Get the numerical representation of the uuid as a unsigned 128-bit number.
    ///
    /// ```
    /// use tycho::Uuid;
    /// let uuid = Uuid::from_string("2f543f3c-0659-4e92-33b0-c8a85c2ac85e").unwrap();
    ///
    /// assert_eq!(uuid.numeric(), 62911151285467627956781226334379231326);
    /// ```
    pub fn numeric(&self) -> u128 {
        self.0
    }

    /// Get the bytes representation of a uuid.
    /// ```
    /// use tycho::Uuid;
    /// let uuid = Uuid::from_string("2f543f3c-0659-4e92-33b0-c8a85c2ac85e").unwrap();
    /// assert_eq!(uuid.bytes(), vec![47, 84, 63, 60, 6, 89, 78, 146, 51, 176, 200, 168, 92, 42, 200, 94]);
    /// ```
    pub fn bytes(&self) -> Vec<u8> { self.0.to_be_bytes().to_vec() }

    /// Get the slice of the bytes representation of a uuid.
    /// ```
    /// use tycho::Uuid;
    /// let uuid = Uuid::from_string("2f543f3c-0659-4e92-33b0-c8a85c2ac85e").unwrap();
    /// assert_eq!(uuid.slice(), [47, 84, 63, 60, 6, 89, 78, 146, 51, 176, 200, 168, 92, 42, 200, 94]);
    /// ```
    pub fn slice(&self) -> [u8; 16] { self.0.to_be_bytes() }


    /// Get a null (0 value) uuid.
    ///
    /// `00000000-0000-0000-0000-000000000000`
    pub fn nil() -> Self {
        Self(0)
    }

    /// Check if the uuid is nil, a value of 0.
    /// ```
    /// use tycho::Uuid;
    ///
    /// let uuid = Uuid::nil();
    /// assert_eq!(uuid.is_nil(), true);
    /// ```
    pub fn is_nil(&self) -> bool {
        self.0 == 0
    }

    /// Get the version number of a uuid as an u8.
    ///
    /// ```
    /// use tycho::Uuid;
    /// let uuid = Uuid::from_numeric(62911151285467627956781226334379231326);
    ///
    /// assert_eq!(uuid.version(), 4);
    /// ```
    pub fn version(&self) -> u8 {
        ((self.0 & 0x00000000_0000_f000_0000_000000000000u128) >> 76) as u8
    }

    /// Create an uuid from a numerical value.
    /// ```
    /// use tycho::Uuid;
    /// let uuid = Uuid::from_numeric(62911151285467627956781226334379231326);
    ///
    /// assert_eq!(uuid.hex(), "2f543f3c06594e9233b0c8a85c2ac85e");
    /// ```
    pub fn from_numeric(x: u128) -> Self {
        Self(x)
    }

    /// Create a uuid from an unformatted 32 length hex string.
    ///
    /// Returns none on failure.
    /// ```
    /// use tycho::Uuid;
    /// let uuid = Uuid::from_hex("2f543f3c06594e9233b0c8a85c2ac85e").unwrap();
    ///
    /// assert_eq!(uuid.hex(), "2f543f3c06594e9233b0c8a85c2ac85e");
    /// ```
    pub fn from_hex(x: &str) -> Option<Self> {
        Some(Self(u128::from_str_radix(x, 16).ok()?))
    }

    /// Create a uuid from an formatted 32 length hex string.
    ///
    /// Returns none on failure.
    /// ```
    /// use tycho::Uuid;
    /// let uuid = Uuid::from_string("2f543f3c-0659-4e92-33b0-c8a85c2ac85e").unwrap();
    /// assert_eq!(uuid.hex(), "2f543f3c06594e9233b0c8a85c2ac85e");
    /// ```
    pub fn from_string(x: &str) -> Option<Self> {
        Self::from_hex(&x.replace("-", ""))
    }

    /// Try to create a uuid from any string.
    ///
    /// This function strips all non-hex characters and trims length of string.
    ///
    /// If it fails, it returns a nil uuid.
    /// ```
    /// use tycho::Uuid;
    /// let uuid = Uuid::from_string_lossy("2<f54>??z3f3c-0659-[4]e92_33b0-c8a85opolc2ac85e@bbc.co.uk");
    ///
    /// assert_eq!(uuid.hex(), "2f543f3c06594e9233b0c8a85c2ac85e");
    /// ```
    pub fn from_string_lossy(x: &str) -> Self {
        let mut y = x.chars().filter(|x|
                   x == &'0' || x == &'1' || x == &'2' || x == &'3'
                || x == &'4' || x == &'5' || x == &'6' || x == &'7'
                || x == &'8' || x == &'9' || x == &'a' || x == &'b'
                || x == &'c' || x == &'d' || x == &'e' || x == &'f' )
            .collect::<String>();
        y.truncate(32);
        Self::from_hex(&y).unwrap_or(Uuid::nil())
    }

    /// Create a uuid from a slice of length 16.
    ///
    /// ```
    /// use tycho::Uuid;
    /// let uuid = Uuid::from_slice([47, 84, 63, 60, 6, 89, 78, 146, 51, 176, 200, 168, 92, 42, 200, 94]);
    ///
    /// assert_eq!(uuid.string(), "2f543f3c-0659-4e92-33b0-c8a85c2ac85e");
    /// ```
    pub fn from_slice(x: [u8; 16]) -> Self {
        Self(u128::from_be_bytes(x))
    }

    /// Create a uuid from any size vec of u8.
    ///
    /// ```
    /// use tycho::Uuid;
    /// let uuid = Uuid::from_bytes(&vec![47, 84, 63, 60, 6, 89, 78, 146, 51, 176, 200, 168, 92, 42, 200, 94]);
    ///
    /// assert_eq!(uuid.string(), "2f543f3c-0659-4e92-33b0-c8a85c2ac85e");
    /// ```
    pub fn from_bytes(x: &[u8]) -> Self {
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&x);
        Self(u128::from_be_bytes(bytes))
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self::nil()
    }
}

impl fmt::Debug for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("UUID(")?;
        f.write_str(&self.string())?;
        f.write_str(")")
    }
}
impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.string())
    }
}

#[cfg(feature="serde_support")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Visitor, de::Error as DeError};

#[cfg(feature="serde_support")]
impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.string())
        } else {
            let mut stu = serializer.serialize_struct("___tycho___/uuid", 1)?;
            stu.serialize_field("inner", &UuidBytes(self.slice()))?;
            stu.end()
        }
    }
}

#[cfg(feature="serde_support")]
pub struct UuidBytes([u8; 16]);

#[cfg(feature="serde_support")]
impl Serialize for UuidBytes {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        serializer.serialize_bytes(&self.0)
    }
}

#[cfg(feature="serde_support")]
impl<'de> Deserialize<'de> for Uuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_any(UuidVisitor)
    }
}

#[cfg(feature="serde_support")]
pub struct UuidVisitor;

#[cfg(feature="serde_support")]
impl<'de> Visitor<'de> for UuidVisitor {
    type Value = Uuid;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("valid UUID.")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
        E: DeError, {
        if let Some(value) = Uuid::from_string(&v) {
            Ok(value)
        } else {
            Err(E::custom("Invalid UUID"))
        }
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where
        E: DeError, {
        Ok(Uuid::from_bytes(&v))
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(x: uuid::Uuid) -> Self {
        Self(x.as_u128())
    }
}

impl Into<uuid::Uuid> for Uuid {
    fn into(self) -> uuid::Uuid {
        uuid::Uuid::from_u128(self.0)
    }
}

use std::hash::{Hash, Hasher};

impl Hash for Uuid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}