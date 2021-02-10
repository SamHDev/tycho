use byteorder::{BigEndian, ByteOrder};
use crate::decode::error::DecodeError;

pub(crate) struct Reader(pub(crate) Vec<u8>, pub(crate) usize);

impl Reader {
    pub(crate) fn create(data: Vec<u8>) -> Self {
        Self(data, 0)
    }

    #[allow(dead_code)]
    pub(crate) fn destroy(self) -> Vec<u8> {
        self.0
    }

    pub(crate) fn remaining(&self) -> usize {
        self.0.len() - self.1
    }

    pub(crate) fn pos(&self) -> usize {
        self.1
    }

    pub(crate) fn read_one(&mut self) -> Result<u8, DecodeError> {
        if self.remaining() == 0 {
            return Err(DecodeError::UnexpectedEnd { expected: 1, found: 0 });
        }
        self.1 += 1;
        Ok((self.0[self.1 - 1]).clone())
    }

    pub(crate) fn read_many(&mut self, size: usize) -> Result<Vec<u8>, DecodeError> {
        if self.remaining() < size {
            return Err(DecodeError::UnexpectedEnd { expected: size as usize, found: self.0.len() });
        }
        self.1 += size;
        let c = self.0[self.1-size..self.1].to_vec();
        Ok(c)
    }

    #[allow(dead_code)]
    pub(crate) fn peek_one(&mut self) -> Result<u8, DecodeError> {
        if self.remaining() == 0 {
            return Err(DecodeError::UnexpectedEnd { expected: 1, found: 0 });
        }
        Ok((self.0[self.1]).clone())
    }

    #[allow(dead_code)]
    pub(crate) fn peek_many(&mut self, size: usize) -> Result<Vec<u8>, DecodeError> {
        if self.remaining() < size {
            return Err(DecodeError::UnexpectedEnd { expected: size as usize, found: self.0.len() });
        }
        let c = self.0[self.1..self.1+size].to_vec();
        Ok(c)
    }

    pub(crate) fn read_u8(&mut self) -> Result<u8, DecodeError> { self.read_one() }
    pub(crate) fn read_i8(&mut self) -> Result<i8, DecodeError> { Ok(self.read_one()? as i8) }
    pub(crate) fn read_u16(&mut self) -> Result<u16, DecodeError> { Ok(BigEndian::read_u16(&self.read_many(2)?)) }
    pub(crate) fn read_i16(&mut self) -> Result<i16, DecodeError> { Ok(BigEndian::read_i16(&self.read_many(2)?)) }
    pub(crate) fn read_u32(&mut self) -> Result<u32, DecodeError> { Ok(BigEndian::read_u32(&self.read_many(4)?)) }
    pub(crate) fn read_i32(&mut self) -> Result<i32, DecodeError> { Ok(BigEndian::read_i32(&self.read_many(4)?)) }
    pub(crate) fn read_u64(&mut self) -> Result<u64, DecodeError> { Ok(BigEndian::read_u64(&self.read_many(8)?)) }
    pub(crate) fn read_i64(&mut self) -> Result<i64, DecodeError> { Ok(BigEndian::read_i64(&self.read_many(8)?)) }
    pub(crate) fn read_u128(&mut self) -> Result<u128, DecodeError> { Ok(BigEndian::read_u128(&self.read_many(16)?)) }
    pub(crate) fn read_i128(&mut self) -> Result<i128, DecodeError> { Ok(BigEndian::read_i128(&self.read_many(16)?)) }
    pub(crate) fn read_f32(&mut self) -> Result<f32, DecodeError> { Ok(BigEndian::read_f32(&self.read_many(4)?)) }
    pub(crate) fn read_f64(&mut self) -> Result<f64, DecodeError> { Ok(BigEndian::read_f64(&self.read_many(8)?)) }
}
