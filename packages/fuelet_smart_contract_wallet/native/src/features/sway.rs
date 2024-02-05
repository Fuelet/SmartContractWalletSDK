use std::str::FromStr;

use fuel_crypto::{PublicKey, SecretKey};
use fuel_crypto::fuel_types::ContractId;
use fuels::accounts::predicate::Predicate;
use fuels::prelude::{Contract, LoadConfiguration, WalletUnlocked};
use fuels::types::Bits256;
use sha2::{Digest, Sha256};

use crate::{consts, model, util};

pub fn get_recovery_checker_contract(recovery_secret_key: &SecretKey) -> Contract {
    // TODO: derive from WalletUnlocked
    let pub_key = recovery_secret_key.public_key();
    let configurables = model::RecoveryCheckerContractConfigurables::new()
        .with_OWNER_PUBLIC_KEY(Bits256(pub_key.hash().into()));
    let config = LoadConfiguration::default()
        .with_configurables(configurables);

    Contract::load_from(consts::CONTRACT_BIN_PATH, config).unwrap()
}

pub async fn deploy_contract(wallet: &WalletUnlocked, contract: Contract) {
    contract.deploy(wallet, util::default_tx_policies()).await.unwrap();
}

pub fn get_script(wallet: &WalletUnlocked, contract_id: ContractId) -> model::WithdrawalScript<WalletUnlocked> {
    let configurables = model::WithdrawalScriptConfigurables::new()
        .with_RECOVERY_CHECKER_CONTRACT(contract_id);
    model::WithdrawalScript::new(wallet.clone(), consts::SCRIPT_BIN_PATH)
        .with_configurables(configurables)
}

pub fn get_script_hash(script: &model::WithdrawalScript<WalletUnlocked>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(
        script
            .main()
            .script_call
            .script_binary,
    );
    let b256 = Bits256(hasher.finalize().into());
    b256.0
}

pub fn get_predicate_from_script_hash(r1_public_key: String, script_hash: [u8; 32]) -> Predicate {
    let pub_key = PublicKey::from_str(r1_public_key.as_str()).unwrap();
    let configurables = model::SecureEnclavePredicateConfigurables::new()
        .with_SECURE_ENCLAVE_PUBLIC_KEY(Bits256(pub_key.hash().into()))
        .with_EXPECTED_SCRIPT_BYTECODE_HASH(Bits256(script_hash));
    let predicate = Predicate::load_from(consts::PREDICATE_BIN_PATH).unwrap()
        .with_configurables(configurables);
    predicate
}

pub fn get_predicate(r1_public_key: &String, recovery_secret_key: &SecretKey, recovery_wallet: &WalletUnlocked) -> Predicate {
    let recovery_contract = get_recovery_checker_contract(recovery_secret_key);
    let withdrawal_script = get_script(recovery_wallet, recovery_contract.contract_id());
    let script_hash = get_script_hash(&withdrawal_script);
    get_predicate_from_script_hash(r1_public_key.clone(), script_hash)
}
