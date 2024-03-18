use std::str::FromStr;

use fuels::accounts::fuel_crypto::SecretKey;
use fuels::prelude::{Provider, WalletUnlocked};

use crate::features::{sway, transaction};

pub struct SmartContractWallet {
    pub bech32_address: String,
    pub r1_public_key: String,
    pub contract_id: String,
    pub(crate) recovery_private_key: String,
    pub(crate) node_url: String,
}

impl SmartContractWallet {
    #[tokio::main]
    pub async fn connect(r1_public_key: String, recovery_private_key: String, node_url: String) -> SmartContractWallet {
        let secret_key = SecretKey::from_str(recovery_private_key.as_str()).unwrap();
        let provider = Provider::connect(node_url.clone()).await.unwrap();
        let wallet = WalletUnlocked::new_from_private_key(secret_key, Some(provider));

        let recovery_contract = sway::get_recovery_checker_contract(&secret_key);
        let predicate = sway::get_predicate(&recovery_contract, &r1_public_key, &wallet);
        let address = predicate.address().to_string();
        SmartContractWallet { bech32_address: address, r1_public_key, contract_id: recovery_contract.contract_id().to_string(), recovery_private_key, node_url }
    }

    async fn get_provider(&self) -> Provider {
        Provider::connect(&self.node_url).await.unwrap()
    }

    fn get_recovery_secret_key(&self) -> SecretKey {
        SecretKey::from_str(self.recovery_private_key.as_str()).unwrap()
    }

    async fn get_recovery_wallet(&self) -> WalletUnlocked {
        let provider = self.get_provider().await;
        let secret_key = self.get_recovery_secret_key();
        WalletUnlocked::new_from_private_key(secret_key, Some(provider))
    }

    #[tokio::main]
    pub async fn deploy_contract(&self) {
        let recovery_secret_key = self.get_recovery_secret_key();
        let recovery_wallet = self.get_recovery_wallet().await;
        let recovery_contract = sway::get_recovery_checker_contract(&recovery_secret_key);
        sway::deploy_contract(&recovery_wallet, recovery_contract).await;
    }

    #[tokio::main]
    pub async fn gen_transfer_tx_request(&self,
                                         to_b256: String,
                                         amount: u64,
                                         asset: String) -> (Vec<u8>, Vec<u8>) {
        let recovery_secret_key = self.get_recovery_secret_key();
        let recovery_wallet = self.get_recovery_wallet().await;
        let recovery_contract = sway::get_recovery_checker_contract(&recovery_secret_key);
        let predicate = sway::get_predicate(&recovery_contract, &self.r1_public_key, &recovery_wallet);
        transaction::get_transfer_request_from_predicate(&predicate, recovery_wallet.provider().unwrap(), to_b256, amount, asset).await
    }


    #[tokio::main]
    pub async fn send_tx(&self, encoded_tx: Vec<u8>, signature: Vec<u8>) -> String {
        let provider = self.get_provider().await;
        transaction::send_tx_using_predicate(&provider, encoded_tx, signature).await
    }
}
