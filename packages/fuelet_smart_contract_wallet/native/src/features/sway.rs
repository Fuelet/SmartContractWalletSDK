use std::io::Write;
use std::ops::Deref;
use std::str::FromStr;

use fuel_crypto::fuel_types::ContractId;
use fuel_crypto::SecretKey;
use fuel_tx::{Bytes32, Salt, StorageSlot};
use fuels::accounts::predicate::Predicate;
use fuels::core::Configurables;
use fuels::prelude::{Contract, StorageConfiguration, WalletUnlocked};
use fuels::types::Bits256;
use sha2::{Digest, Sha256};

use crate::{gen_consts, model, util};
use crate::error::CustomResult;

fn load_contract_from_code(code: &mut Vec<u8>, configurables: Configurables, storage_slots: Vec<StorageSlot>, salt: Salt) -> CustomResult<Contract> {
    configurables.update_constants_in(code);
    Ok(Contract::new(code.clone(), salt, storage_slots))
}

fn get_storage_slots() -> Vec<StorageSlot> {
    gen_consts::RECOVERY_CHECKER_STORAGE_SLOTS
        .to_vec()
        .iter()
        .map(|(k, v)| StorageSlot::new(Bytes32::from_str(k).unwrap(), Bytes32::from_str(v).unwrap()))
        .collect::<Vec<_>>()
}

pub fn get_recovery_checker_contract(recovery_secret_key: &SecretKey) -> Contract {
    // TODO: derive from WalletUnlocked
    let pub_key = recovery_secret_key.public_key();
    let configurables = model::RecoveryCheckerContractConfigurables::new()
        .with_OWNER_PUBLIC_KEY(Bits256(pub_key.hash().into()));

    StorageConfiguration::new(false, vec![]);

    let mut contract_code = hex::decode(gen_consts::RECOVERY_CHECKER_CONTRACT_CODE).unwrap();
    let storage_slots = get_storage_slots();

    load_contract_from_code(&mut contract_code, configurables.into(), storage_slots, Salt::default()).unwrap()
}

pub async fn deploy_contract(wallet: &WalletUnlocked, contract: Contract) {
    contract.deploy(wallet, util::default_tx_policies()).await.unwrap();
}

pub fn get_script(wallet: &WalletUnlocked, contract_id: ContractId) -> model::WithdrawalScript<WalletUnlocked> {
    let configurables = model::WithdrawalScriptConfigurables::new()
        .with_RECOVERY_CHECKER_CONTRACT(contract_id);
    let script_code = hex::decode(gen_consts::WITHDRAWAL_SCRIPT_CODE).unwrap();

    let mut temp_file = tempfile::Builder::new()
        .suffix(".bin").tempfile().unwrap();
    temp_file.write_all(script_code.deref()).unwrap();
    let path = temp_file.path().to_str().unwrap();

    model::WithdrawalScript::new(wallet.clone(), path)
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
    let pub_key = Bits256::from_hex_str(r1_public_key.as_str()).unwrap();// PublicKey::from_str(r1_public_key.as_str()).unwrap();
    let configurables = model::SecureEnclavePredicateConfigurables::new()
        .with_SECURE_ENCLAVE_PUBLIC_KEY(pub_key)
        .with_EXPECTED_SCRIPT_BYTECODE_HASH(Bits256(script_hash));
    let predicate_code = hex::decode(gen_consts::SECURE_ENCLAVE_PREDICATE_CODE).unwrap();
    let predicate = Predicate::from_code(predicate_code)
        .with_configurables(configurables);
    predicate
}

pub fn get_predicate(r1_public_key: &String, recovery_secret_key: &SecretKey, recovery_wallet: &WalletUnlocked) -> Predicate {
    let recovery_contract = get_recovery_checker_contract(recovery_secret_key);
    let withdrawal_script = get_script(recovery_wallet, recovery_contract.contract_id());
    let script_hash = get_script_hash(&withdrawal_script);
    get_predicate_from_script_hash(r1_public_key.clone(), script_hash)
}
