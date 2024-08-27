use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

use crate::Error;

pub trait Serialize: Sized {
    fn serialize<W: Write>(&self, w: &mut W);
    fn deserialize<R: Read>(r: &mut R) -> Result<Self, Error>;
}

impl Serialize for u8 {
    fn serialize<W: Write>(&self, w: &mut W) {
        w.write_u8(*self).unwrap();
    }

    fn deserialize<R: Read>(r: &mut R) -> Result<Self, Error> {
        Ok(r.read_u8()?)
    }
}

impl Serialize for i8 {
    fn serialize<W: Write>(&self, w: &mut W) {
        w.write_i8(*self).unwrap();
    }

    fn deserialize<R: Read>(r: &mut R) -> Result<Self, Error> {
        Ok(r.read_i8()?)
    }
}

macro_rules! impl_serialize_for_primitive {
    ($ty:ty, $read:ident, $write:ident) => {
        impl Serialize for $ty {
            fn serialize<W: Write>(&self, w: &mut W) {
                w.$write::<BigEndian>(*self).unwrap();
            }

            fn deserialize<R: Read>(r: &mut R) -> Result<Self, Error> {
                Ok(r.$read::<BigEndian>()?)
            }
        }
    };
}

impl_serialize_for_primitive!(u16, read_u16, write_u16);
impl_serialize_for_primitive!(u32, read_u32, write_u32);
impl_serialize_for_primitive!(u64, read_u64, write_u64);

impl_serialize_for_primitive!(i16, read_i16, write_i16);
impl_serialize_for_primitive!(i32, read_i32, write_i32);
impl_serialize_for_primitive!(i64, read_i64, write_i64);

impl_serialize_for_primitive!(f32, read_f32, write_f32);
impl_serialize_for_primitive!(f64, read_f64, write_f64);

impl Serialize for String {
    fn serialize<W: Write>(&self, w: &mut W) {
        assert!(self.len() <= u16::MAX as usize);
        (self.len() as u16).serialize(w);
        w.write_all(self.as_bytes()).unwrap();
    }

    fn deserialize<R: Read>(r: &mut R) -> Result<Self, Error> {
        let len = u16::deserialize(r)?;
        let mut data = vec![0; len as usize];
        r.read_exact(&mut data)?;
        String::from_utf8(data).map_err(|e| {
            Error::NonUnicodeString(e.into_bytes())
        })
    }
}
