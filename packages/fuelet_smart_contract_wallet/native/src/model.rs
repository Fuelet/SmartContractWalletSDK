use fuels::prelude::*;

abigen!(
    Predicate(
        name = "SecureEnclavePredicate",
        abi = "sway/secure_enclave_predicate/out/debug/secure_enclave_predicate-abi.json"
    ),
    Contract(
        name = "RecoveryCheckerContract",
        abi = "sway/recovery_checker/out/debug/recovery_checker-abi.json"
    ),
    Script(
        name = "WithdrawalScript",
        abi = "sway/withdrawal_script/out/debug/withdrawal_script-abi.json"
    ),
);
