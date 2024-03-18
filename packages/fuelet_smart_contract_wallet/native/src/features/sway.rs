use fuel_crypto::fuel_types::ContractId;
use fuel_crypto::SecretKey;
use fuel_tx::Salt;
use fuels::accounts::predicate::Predicate;
use fuels::prelude::{Contract, Provider, WalletUnlocked};
use fuels::types::Bits256;
use sha2::{Digest, Sha256};

use crate::{gen_consts, model, util};
use crate::features::forc_utils;

pub fn get_recovery_checker_contract(recovery_secret_key: &SecretKey) -> Contract {
    // TODO: derive from WalletUnlocked
    let pub_key = recovery_secret_key.public_key();
    let configurables = model::RecoveryCheckerContractConfigurables::new()
        .with_OWNER_PUBLIC_KEY(Bits256(pub_key.hash().into()));
    let storage_slots = forc_utils::convert_storage_slots(gen_consts::RECOVERY_CHECKER_STORAGE_SLOTS.to_vec());
    forc_utils::load_contract_from_code(gen_consts::RECOVERY_CHECKER_CONTRACT_CODE, configurables.into(),
                                        storage_slots, Salt::default()).unwrap()
}

pub async fn deploy_contract(wallet: &WalletUnlocked, contract: Contract) {
    contract.deploy(wallet, util::default_tx_policies()).await.unwrap();
}

pub fn get_script(wallet: &WalletUnlocked, contract_id: ContractId) -> model::WithdrawalScript<WalletUnlocked> {
    let configurables = model::WithdrawalScriptConfigurables::new()
        .with_RECOVERY_CHECKER_CONTRACT(contract_id);
    forc_utils::load_script_from_code(wallet, gen_consts::WITHDRAWAL_SCRIPT_CODE, configurables)
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

pub fn get_predicate_from_script_hash(r1_public_key: String, script_hash: [u8; 32], provider: &Provider) -> Predicate {
    let pub_key = util::b512_from_hex_str(r1_public_key.as_str()).unwrap();
    let configurables = model::SecureEnclavePredicateConfigurables::new()
        .with_SECURE_ENCLAVE_PUBLIC_KEY(pub_key)
        .with_EXPECTED_SCRIPT_BYTECODE_HASH(Bits256(script_hash));
    let predicate_code = hex::decode(gen_consts::SECURE_ENCLAVE_PREDICATE_CODE).unwrap();
    let predicate = Predicate::from_code(predicate_code)
        .with_configurables(configurables)
        .with_provider(provider.clone());
    predicate
}

pub fn get_predicate(recovery_contract: &Contract, r1_public_key: &String, recovery_wallet: &WalletUnlocked) -> Predicate {
    let withdrawal_script = get_script(recovery_wallet, recovery_contract.contract_id());
    let script_hash = get_script_hash(&withdrawal_script);
    get_predicate_from_script_hash(r1_public_key.clone(), script_hash, recovery_wallet.provider().unwrap())
}
