use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub bonding_curve_code_id: u128,
    pub admin: Addr,
    pub fee_swap_collector_address: Addr,
    pub reserve_collector_address: Addr,
}
#[cw_serde]
pub struct CurrentCreation {
    pub name: String,
    pub subdenom: String,
    pub denom: String,
    pub description: String,
    pub url: String,
    pub creator: Addr,
}

#[cw_serde]
pub struct Token {
    pub name: String,
    pub subdenom: String,
    pub denom: String,
    pub description: String,
    pub url: String,
    pub creator: Addr,
    pub bonding_curve_address: Addr,
    pub completed: bool,
    pub pool_id: u64,
    pub created_at: u64,
}

// Crée un singleton pour stocker la configuration
pub const CONFIG: Item<Config> = Item::new("config");

pub const CURRENT_CREATION: Item<CurrentCreation> = Item::new("current_creation");

pub const TOKENS: Map<String, Token> = Map::new("tokens");
