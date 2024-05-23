use fuels::prelude::TxPolicies;
use fuels::types::{B512, Bits256};
use hex::FromHexError;

use crate::error::CustomResult;

// TODO: use adequate policies
pub fn default_tx_policies() -> TxPolicies {
    TxPolicies::default().with_max_fee(10000000).with_script_gas_limit(1000000).with_witness_limit(100000)
}

pub fn b512_from_hex_str(hex_str: &str) -> CustomResult<B512> {
    let hex = if let Some(stripped_hex) = hex_str.strip_prefix("0x") {
        stripped_hex
    } else {
        hex_str
    };

    if hex.len() != 128 {
        return Err(FromHexError::InvalidStringLength.into());
    }

    let (hi, lo) = hex.split_at(64);
    match (Bits256::from_hex_str(hi), Bits256::from_hex_str(lo)) {
        (Ok(hi_bits), Ok(lo_bits)) => Ok(B512::from((hi_bits, lo_bits))),
        (Err(e), _) => Err(e.into()),
        (_, Err(e)) => Err(e.into()),
    }
}
