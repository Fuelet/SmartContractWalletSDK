use std::io::Write;
use std::ops::Deref;
use std::str::FromStr;

use fuel_tx::{Bytes32, Salt, StorageSlot};
use fuels::core::Configurables;
use fuels::prelude::{Contract, WalletUnlocked};
use sha2::Digest;

use crate::error::CustomResult;
use crate::model;

pub(crate) fn load_contract_from_code(code_hex: &str, configurables: Configurables, storage_slots: Vec<StorageSlot>, salt: Salt) -> CustomResult<Contract> {
    let mut code = hex::decode(code_hex).unwrap();
    configurables.update_constants_in(&mut code);
    Ok(Contract::new(code.clone(), salt, storage_slots))
}

pub(crate) fn load_script_from_code(wallet: &WalletUnlocked, code_hex: &str,
                                    configurables: model::WithdrawalScriptConfigurables) -> model::WithdrawalScript<WalletUnlocked> {
    let script_code = hex::decode(code_hex).unwrap();

    let mut temp_file = tempfile::Builder::new()
        .suffix(".bin")
        .tempfile()
        .unwrap();
    temp_file.write_all(script_code.deref()).unwrap();
    let temp_file_path = temp_file.path().to_str().unwrap();

    model::WithdrawalScript::new(wallet.clone(), temp_file_path)
        .with_configurables(configurables)
}

pub(crate) fn convert_storage_slots(slots: Vec<(&str, &str)>) -> Vec<StorageSlot> {
    slots
        .iter()
        .map(|(k, v)| StorageSlot::new(Bytes32::from_str(k).unwrap(), Bytes32::from_str(v).unwrap()))
        .collect::<Vec<_>>()
}
