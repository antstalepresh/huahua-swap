
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Config {
    pub bonding_curve_code_id: u128,
}

// Crée un singleton pour stocker la configuration
pub const CONFIG: Item<Config> = Item::new("config");

pub const CURRENT_SUBDENOM_CREATION : Item<String> = Item::new("current_subdenom_creation");
