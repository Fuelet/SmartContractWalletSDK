predicate;

use std::{constants::ZERO_B256, b512::B512, ecr::{ec_recover_r1}, tx::{tx_id, tx_witnesses_count, tx_witness_data, tx_script_bytecode_hash}};

configurable {
    SECURE_ENCLAVE_PUBLIC_KEY: B512 = B512::new(),
    EXPECTED_SCRIPT_BYTECODE_HASH: b256 = ZERO_B256,
}

fn witness_matches_signature(signature: B512, message: b256) -> bool {
    match ec_recover_r1(signature, message) {
        Err(_) => false,
        Ok(pub_key) => pub_key == SECURE_ENCLAVE_PUBLIC_KEY,
    }
}

fn contains_required_signature() -> bool {
    let message: b256 = tx_id();

    let witnesses_count = tx_witnesses_count();
    let mut witness_index = 0;
    while witness_index < witnesses_count {
        let witness_data: B512 = tx_witness_data(witness_index);
        if (witness_matches_signature(witness_data, message)) {
            return true;
        }
        witness_index += 1;
    }
    false
}

fn uses_predefined_script() -> bool {
    let script_bytecode_hash: b256 = tx_script_bytecode_hash();
    script_bytecode_hash == EXPECTED_SCRIPT_BYTECODE_HASH
}

// TODO: provide index of the input to avoid iteration?
fn main() -> bool {
    contains_required_signature() || uses_predefined_script()
}
