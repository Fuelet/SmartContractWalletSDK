script;

use std::constants::ZERO_B256;
use shared::RecoveryChecker;

configurable {
    RECOVERY_CHECKER_CONTRACT: ContractId = ContractId::from(ZERO_B256),
}

fn main() {
    let recovery_checker_contract = abi(RecoveryChecker, RECOVERY_CHECKER_CONTRACT.into());
    recovery_checker_contract.check_cooldown_passed();
}
