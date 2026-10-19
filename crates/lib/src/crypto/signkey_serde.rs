use ecdsa::SigningKey;
use k256::Secp256k1;
use serde::Deserialize;

pub fn serialize<S>(key: &SigningKey<Secp256k1>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let bytes = key.to_bytes();
    serializer.serialize_bytes(&bytes)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<SigningKey<Secp256k1>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let bytes: Vec<u8> = Vec::<u8>::deserialize(deserializer)?;
    Ok(SigningKey::from_slice(&bytes).unwrap())
}
