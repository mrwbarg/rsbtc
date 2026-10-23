use ecdsa::SigningKey;
use k256::Secp256k1;
use k256::elliptic_curve::Generate;
use serde::{Deserialize, Serialize};

use super::PublicKey;

use super::signkey_serde;

use crate::util::Saveable;

use std::io::{Error as IoError, ErrorKind as IoErrorKind, Read, Result as IoResult, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateKey(#[serde(with = "signkey_serde")] pub SigningKey<Secp256k1>);

impl PrivateKey {
    pub fn new_key() -> Self {
        PrivateKey(SigningKey::generate())
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.verifying_key().clone())
    }
}

impl Saveable for PrivateKey {
    fn load<I: Read>(reader: I) -> IoResult<Self> {
        ciborium::de::from_reader(reader).map_err(|_| {
            IoError::new(
                IoErrorKind::InvalidData,
                "Failed to deserialize private key",
            )
        })
    }

    fn save<O: Write>(&self, writer: O) -> IoResult<()> {
        ciborium::ser::into_writer(self, writer).map_err(|_| {
            IoError::new(IoErrorKind::InvalidData, "Failed to serialize private key")
        })?;
        Ok(())
    }
}
