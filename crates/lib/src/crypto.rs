use ecdsa::VerifyingKey;
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

mod signkey_serde;

mod private_key;
mod signature;

pub use private_key::PrivateKey;
pub use signature::Signature;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PublicKey(VerifyingKey<Secp256k1>);
