use bitflags::bitflags;
use crate::serialize::Serialize;

#[derive(Debug)]
pub struct AuthMechs(u32);

bitflags! {
    impl AuthMechs: u32 {
        const LEGACY = 1 << 0;
        const SRP = 1 << 1;
        const FIRST_SRP = 1 << 2;
    }
}

impl Serialize for AuthMechs {
    fn serialize<W: std::io::Write>(&self, w: &mut W) {
        self.bits().serialize(w);
    }

    fn deserialize<R: std::io::Read>(r: &mut R) -> Result<Self, crate::Error> {
        Ok(Self::from_bits_truncate(u32::deserialize(r)?))
    }
}
