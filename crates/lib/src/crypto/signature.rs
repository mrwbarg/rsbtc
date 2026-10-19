use ecdsa::{
    Signature as ECDSASignature,
    signature::{Signer, Verifier},
};
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

use crate::crypto::{PrivateKey, PublicKey};
use crate::sha256::Hash;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature(ECDSASignature<Secp256k1>);

impl Signature {
    pub fn sign_output(output_hash: &Hash, private_key: &PrivateKey) -> Self {
        let signing_key = &private_key.0;
        let signature = signing_key.sign(&output_hash.as_bytes());
        Signature(signature)
    }

    pub fn verify(&self, output_hash: &Hash, public_key: &PublicKey) -> bool {
        public_key
            .0
            .verify(&output_hash.as_bytes(), &self.0)
            .is_ok()
    }
}
