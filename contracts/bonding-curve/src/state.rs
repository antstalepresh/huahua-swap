use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Config {
    pub token_denom: String,
    pub manager_contract: Addr,
    pub completed: bool,
    pub fee_percent: Decimal,
    pub fee_collector_address: Addr,
    pub token_sold: u128,
    pub reserve_token_amount: u128,
}

// Crée un singleton pour stocker la configuration
pub const CONFIG: Item<Config> = Item::new("config");
