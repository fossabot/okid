use std::fmt::Display;

use crate::{BinaryType, Digest, IntoOkId, OkId};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct Blake3(pub(super) [u8; 32]);

impl From<blake3::Hasher> for OkId {
    fn from(value: blake3::Hasher) -> Self {
        let data = value.finalize();
        let data = data.as_bytes();
        let mut buf = [0; 32];
        if data.len() == 32 {
            buf.copy_from_slice(data);
        }

        Self {
            hash_type: BinaryType::Blake3,
            digest: Digest::Blake3(Blake3(buf)),
        }
    }
}

impl IntoOkId for blake3::Hasher {}

impl Display for Blake3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.0;
        let buf = hex::encode(data);
        f.write_str(&buf)?;
        Ok(())
    }
}

impl std::str::FromStr for Blake3 {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = hex::decode(s)?;
        let mut hash: [u8; 32] = [0; 32];
        hash.copy_from_slice(&buf);
        Ok(Blake3(hash))
    }
}
