use fuels::prelude::TxPolicies;

pub fn default_tx_policies() -> TxPolicies {
    TxPolicies::default().with_gas_price(1).with_max_fee(10000000).with_script_gas_limit(1000000).with_witness_limit(100000)
}
