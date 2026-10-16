use serde::{Deserialize, Serialize};
use uint::construct_uint;

pub mod crypto;
pub mod sha256;
pub mod types;
pub mod util;

construct_uint! {
    // Unsigned 256-bit integer
    // consisting of four 64-bit words.
    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}
