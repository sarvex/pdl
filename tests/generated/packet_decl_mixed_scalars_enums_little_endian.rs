// @generated rust packets from test

#![allow(warnings, missing_docs)]

use bytes::{Buf, BufMut, Bytes, BytesMut};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::sync::Arc;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Packet parsing failed")]
    InvalidPacketError,
    #[error("{field} was {value:x}, which is not known")]
    ConstraintOutOfBounds { field: String, value: u64 },
    #[error("when parsing {obj} needed length of {wanted} but got {got}")]
    InvalidLengthError { obj: String, wanted: usize, got: usize },
    #[error("Due to size restrictions a struct could not be parsed.")]
    ImpossibleStructError,
    #[error("when parsing field {obj}.{field}, {value} is not a valid {type_} value")]
    InvalidEnumValueError { obj: String, field: String, value: u64, type_: String },
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct TryFromError(&'static str);

pub trait Packet {
    fn to_bytes(self) -> Bytes;
    fn to_vec(self) -> Vec<u8>;
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum Enum7 {
    A = 0x1,
    B = 0x2,
}

#[derive(FromPrimitive, ToPrimitive, Debug, Hash, Eq, PartialEq, Clone, Copy)]
#[repr(u64)]
pub enum Enum9 {
    A = 0x1,
    B = 0x2,
}

#[derive(Debug)]
struct FooData {
    x: Enum7,
    y: u8,
    z: Enum9,
    w: u8,
}
#[derive(Debug, Clone)]
pub struct FooPacket {
    foo: Arc<FooData>,
}
#[derive(Debug)]
pub struct FooBuilder {
    pub x: Enum7,
    pub y: u8,
    pub z: Enum9,
    pub w: u8,
}
impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 3
    }
    fn parse(mut bytes: &[u8]) -> Result<Self> {
        if bytes.remaining() < 3 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 3,
                got: bytes.remaining(),
            });
        }
        let chunk = bytes.get_uint_le(3) as u32;
        let x = Enum7::from_u8((chunk & 0x7f) as u8).unwrap();
        let y = ((chunk >> 7) & 0x1f) as u8;
        let z = Enum9::from_u16(((chunk >> 12) & 0x1ff) as u16).unwrap();
        let w = ((chunk >> 21) & 0x7) as u8;
        Ok(Self { x, y, z, w })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        if self.y > 0x1f {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "y", self.y, 0x1f);
        }
        if self.w > 0x7 {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "w", self.w, 0x7);
        }
        let value = (self.x.to_u8().unwrap() as u32)
            | ((self.y as u32) << 7)
            | ((self.z.to_u16().unwrap() as u32) << 12)
            | ((self.w as u32) << 21);
        buffer.put_uint_le(value as u64, 3);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        3
    }
}
impl Packet for FooPacket {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.foo.get_total_size());
        self.foo.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<FooPacket> for Bytes {
    fn from(packet: FooPacket) -> Self {
        packet.to_bytes()
    }
}
impl From<FooPacket> for Vec<u8> {
    fn from(packet: FooPacket) -> Self {
        packet.to_vec()
    }
}
impl FooPacket {
    pub fn parse(mut bytes: &[u8]) -> Result<Self> {
        Ok(Self::new(Arc::new(FooData::parse(bytes)?)).unwrap())
    }
    fn new(root: Arc<FooData>) -> std::result::Result<Self, &'static str> {
        let foo = root;
        Ok(Self { foo })
    }
    pub fn get_x(&self) -> Enum7 {
        self.foo.as_ref().x
    }
    pub fn get_y(&self) -> u8 {
        self.foo.as_ref().y
    }
    pub fn get_z(&self) -> Enum9 {
        self.foo.as_ref().z
    }
    pub fn get_w(&self) -> u8 {
        self.foo.as_ref().w
    }
}
impl FooBuilder {
    pub fn build(self) -> FooPacket {
        let foo = Arc::new(FooData { x: self.x, y: self.y, z: self.z, w: self.w });
        FooPacket::new(foo).unwrap()
    }
}
