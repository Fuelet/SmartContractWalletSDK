use std::io;
use std::str::FromStr;

use fuel_crypto::fuel_types::canonical::{Deserialize, Serialize};
use fuel_tx::{AssetId, ContractId, Witness};
use fuel_tx::Transaction as FuelTransaction;
use fuels::accounts::fuel_crypto::{PublicKey, rand, SecretKey};
use fuels::accounts::fuel_crypto::rand::Rng;
use fuels::accounts::predicate::Predicate;
use fuels::prelude::{Account, Bech32Address, BuildableTransaction, Contract, LoadConfiguration, Provider, ScriptTransactionBuilder, Transaction, TransactionType, WalletUnlocked};
use fuels::types::Bits256;
use sha2::{Digest, Sha256};

use crate::{consts, util};
use crate::error::CustomResult;
use crate::model::{SecureEnclavePredicateConfigurables, WithdrawalScript, WithdrawalScriptConfigurables};

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

async fn get_predicate(wallet_public_key: String, script_hash: [u8; 32]) -> Predicate {
    let pub_key = PublicKey::from_str(wallet_public_key.as_str()).unwrap();
    let configurables = SecureEnclavePredicateConfigurables::new()
        .with_SECURE_ENCLAVE_PUBLIC_KEY(Bits256(pub_key.hash().into()))
        .with_EXPECTED_SCRIPT_BYTECODE_HASH(Bits256(script_hash));
    let predicate = Predicate::load_from(consts::PREDICATE_BIN_PATH).unwrap()
        .with_configurables(configurables);
    predicate
}

#[tokio::main]
pub async fn get_predicate_address(wallet_public_key: String, script_hash: [u8; 32]) -> String {
    let predicate = get_predicate(wallet_public_key, script_hash).await;
    predicate.address().to_string()
}

#[tokio::main]
pub async fn gen_transfer_tx_request(node_url: String,
                                     wallet_public_key: String,
                                     script_hash: [u8; 32],
                                     to: String,
                                     amount: u64,
                                     asset: String) -> (Vec<u8>, Vec<u8>) {
    let provider = Provider::connect(node_url).await.unwrap();
    let network_info = provider.network_info().await.unwrap();
    let predicate = get_predicate(wallet_public_key, script_hash).await;

    let asset_id = AssetId::from_str(&asset).unwrap();
    let recipient = Bech32Address::from_str(to.as_str()).unwrap();

    let inputs = predicate.get_asset_inputs_for_amount(asset_id, amount).await.unwrap();
    let outputs = predicate.get_asset_outputs_for_amount(&recipient, asset_id, amount);
    let mut tx_builder = ScriptTransactionBuilder::prepare_transfer(inputs, outputs, util::default_tx_policies(), network_info);

    predicate
        .adjust_for_fee(&mut tx_builder, amount)
        .await.unwrap();

    let tx = tx_builder.build(provider.clone()).await.unwrap();
    let fuel_tx: FuelTransaction = tx.clone().into();

    let encoded_tx: Vec<u8> = fuel_tx.to_bytes();
    let tx_id = tx.id(provider.chain_id());
    (encoded_tx, tx_id.to_vec())
}

fn decode_transaction(encoded_tx: &Vec<u8>) -> CustomResult<TransactionType> {
    let decoded_tx: FuelTransaction = FuelTransaction::from_bytes(&encoded_tx).unwrap();
    Ok(wrap_fuel_transaction(decoded_tx)?)
}

fn wrap_fuel_transaction(value: FuelTransaction) -> CustomResult<TransactionType> {
    match value {
        FuelTransaction::Script(script) => Ok(TransactionType::Script(script.into())),
        FuelTransaction::Create(create) => Ok(TransactionType::Create(create.into())),
        FuelTransaction::Mint(_) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Cannot convert Mint transaction",
        ).into())
    }
}

fn add_signature(tx: &mut TransactionType, signature: &[u8]) -> fuels::prelude::Result<usize> {
    let witness = Witness::from(signature);
    match tx {
        TransactionType::Script(script) => script.append_witness(witness),
        TransactionType::Create(create) => create.append_witness(witness),
        TransactionType::Mint(mint) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Cannot sign Mint transaction",
        ).into())
    }
}


#[tokio::main]
pub async fn send_tx(node_url: String, encoded_tx: Vec<u8>, signature: Vec<u8>) -> String {
    let provider = Provider::connect(node_url).await.unwrap();
    let mut tx = decode_transaction(&encoded_tx).unwrap();
    add_signature(&mut tx, signature.as_slice()).unwrap();
    let tx_id = match tx {
        TransactionType::Script(script) => provider.send_transaction(script).await.unwrap(),
        TransactionType::Create(create) => provider.send_transaction(create).await.unwrap(),
        TransactionType::Mint(_) => panic!()
    };
    tx_id.to_string()
}
