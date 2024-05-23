contract;

use shared::RecoveryChecker;
use std::{constants::ZERO_B256, auth::msg_sender, b512::B512, ecr::{ec_recover_address}, tx::{tx_id, tx_witnesses_count, tx_witness_data}};

configurable {
    OWNER_PUBLIC_KEY: b256 = ZERO_B256,
}

pub enum RecoveryCheckerError {
    NotRecovered: (),
}

pub enum AccessError {
    NotOwner: (),
}

storage {
	recovered: bool = false,
}

fn only_owner() {
    require(
        Identity::Address(Address::from(OWNER_PUBLIC_KEY)) == msg_sender().unwrap(),
        AccessError::NotOwner,
    );
}

fn witness_matches_owner_signature(signature: B512, message: b256) -> bool {
    match ec_recover_address(signature, message) {
        Err(_) => false,
        Ok(address) => OWNER_PUBLIC_KEY == address.into(),
    }
}

// TODO: do not iterate over all witnesses. Provide an index instead
fn contains_owner_signature() -> bool {
    let message: b256 = tx_id();

    let witnesses_count = tx_witnesses_count();
    let mut witness_index = 0;
    while witness_index < witnesses_count {
        let witness_data: B512 = tx_witness_data(witness_index);
        if (witness_matches_owner_signature(witness_data, message)) {
            return true;
        }
        witness_index += 1;
    }
    false
}

fn check_owner_signature() {
    require(
        contains_owner_signature(),
        AccessError::NotOwner,
    );
}

impl RecoveryChecker for Contract {
    #[storage(read)]
    fn check_cooldown_passed() {
        // msg_sender checks inputs to determine the sender. We need to derive sender from his signature
        check_owner_signature();
		require(
			storage.recovered.read(),
			RecoveryCheckerError::NotRecovered,
		);
    }

    #[storage(write)]
    fn start_recovery() {
		only_owner();
		storage.recovered.write(true);
    }

    #[storage(write)]
    fn stop_recovery() {
    	only_owner();
    	storage.recovered.write(false);
    }
}
