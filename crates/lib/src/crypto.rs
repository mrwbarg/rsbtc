use ecdsa::{Signature as ECDSASignature, VerifyingKey};
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

mod signkey_serde;

mod private_key;

pub use private_key::PrivateKey;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature(ECDSASignature<Secp256k1>);

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PublicKey(VerifyingKey<Secp256k1>);
