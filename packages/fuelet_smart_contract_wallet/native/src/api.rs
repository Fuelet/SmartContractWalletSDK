use std::str::FromStr;

use fuel_tx::ContractId;
use fuels::accounts::fuel_crypto::{rand, SecretKey};
use fuels::accounts::fuel_crypto::rand::Rng;
use fuels::prelude::{Contract, LoadConfiguration, Provider, WalletUnlocked};
use fuels::types::Bits256;
use sha2::{Digest, Sha256};

use crate::{consts, util};
use crate::model::{WithdrawalScript, WithdrawalScriptConfigurables};

#[tokio::main]
pub async fn deploy_contract(private_key: String, node_url: String) -> String {
    let secret_key = SecretKey::from_str(private_key.as_str()).unwrap();
    let provider = Provider::connect(node_url).await.unwrap();
    let wallet = WalletUnlocked::new_from_private_key(secret_key, Some(provider));

    let mut rng = rand::thread_rng();
    let config = LoadConfiguration::default();

    let id = Contract::load_from(consts::CONTRACT_BIN_PATH, config)
        .unwrap()
        .with_salt(rng.gen::<[u8; 32]>())
        .deploy(&wallet, util::default_tx_policies())
        .await
        .unwrap();
    id.to_string()
}

#[tokio::main]
pub async fn get_script(private_key: String, node_url: String, contract_id_str: String) -> [u8; 32] {
    let secret_key = SecretKey::from_str(private_key.as_str()).unwrap();
    let provider = Provider::connect(node_url).await.unwrap();
    let wallet = WalletUnlocked::new_from_private_key(secret_key, Some(provider));

    let contract_id = ContractId::from_str(contract_id_str.as_str()).unwrap();
    let configurables = WithdrawalScriptConfigurables::new()
        .with_RECOVERY_CHECKER_CONTRACT(contract_id);
    let script = WithdrawalScript::new(wallet.clone(), consts::SCRIPT_BIN_PATH)
        .with_configurables(configurables);

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
