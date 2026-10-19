use ecdsa::SigningKey;
use k256::Secp256k1;
use k256::elliptic_curve::Generate;
use serde::{Deserialize, Serialize};

use super::PublicKey;

use super::signkey_serde;

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
