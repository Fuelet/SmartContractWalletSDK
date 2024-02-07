use std::io;
use std::str::FromStr;

use fuel_crypto::fuel_types::AssetId;
use fuel_crypto::fuel_types::canonical::Deserialize;
use fuel_crypto::fuel_types::canonical::Serialize;
use fuel_tx::Witness;
use fuels::accounts::predicate::Predicate;
use fuels::prelude::{Account, Address, Bech32Address, BuildableTransaction, Provider, ScriptTransactionBuilder, Transaction, TransactionType};
use fuels::tx::FuelTransaction;

use crate::error::CustomResult;
use crate::util;

pub async fn get_transfer_request_from_predicate(predicate: &Predicate,
                                                 provider: &Provider,
                                                 to_b256: String,
                                                 amount: u64,
                                                 asset: String) -> (Vec<u8>, Vec<u8>) {
    let network_info = provider.network_info().await.unwrap();

    let asset_id = AssetId::from_str(&asset).unwrap();
    let b256_address = Address::from_str(to_b256.as_str()).unwrap();
    let recipient = Bech32Address::from(b256_address);

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
        TransactionType::Mint(_) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Cannot sign Mint transaction",
        ).into())
    }
}

pub async fn send_tx_using_predicate(provider: &Provider, encoded_tx: Vec<u8>, signature: Vec<u8>) -> String {
    let mut tx = decode_transaction(&encoded_tx).unwrap();
    add_signature(&mut tx, signature.as_slice()).unwrap();
    let tx_id = match tx {
        TransactionType::Script(script) => provider.send_transaction(script).await.unwrap(),
        TransactionType::Create(create) => provider.send_transaction(create).await.unwrap(),
        TransactionType::Mint(_) => panic!()
    };
    tx_id.to_string()
}
