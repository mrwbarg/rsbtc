use core::fmt;

use ecdsa::VerifyingKey;
use k256::{Secp256k1, pkcs8::spki};
use serde::{Deserialize, Serialize};

use crate::util::Saveable;

use spki::EncodePublicKey;
use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult, Write};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PublicKey(pub VerifyingKey<Secp256k1>);

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let compressed_bytes = &self.0.to_sec1_bytes();
        write!(
            f,
            "PublicKey({})",
            hex::encode(&compressed_bytes).to_string()
        )
    }
}

impl Saveable for PublicKey {
    fn load<I: Read>(mut reader: I) -> IoResult<Self> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;

        let public_key = buf
            .parse()
            .map_err(|_| IoError::new(IoErrorKind::InvalidData, "Failed to parse public key"))?;
        Ok(PublicKey(public_key))
    }

    fn save<O: Write>(&self, mut writer: O) -> IoResult<()> {
        let s = self.0.to_public_key_pem(Default::default()).map_err(|_| {
            IoError::new(IoErrorKind::InvalidData, "Failed to serialize public key")
        })?;
        writer.write_all(s.as_bytes())?;
        Ok(())
    }
}
