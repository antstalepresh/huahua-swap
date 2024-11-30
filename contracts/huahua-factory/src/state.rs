
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Config {
    pub bonding_curve_code_id: u128,
    pub admin :Addr,
}
#[cw_serde]
pub struct CurrentCreation {
    pub subdenom: String,
    pub denom: String,
    pub description: String,
    pub url: String,
    pub creator: Addr,
}

#[cw_serde]
pub struct Token {
    pub subdenom: String,
    pub denom: String,
    pub description: String,
    pub url: String,
    pub creator: Addr,
    pub bonding_curve_address: Addr,
    pub pool_id: u64,

}

// Crée un singleton pour stocker la configuration
pub const CONFIG: Item<Config> = Item::new("config");

pub const CURRENT_CREATION : Item<CurrentCreation> = Item::new("current_creation");

pub const TOKENS: Map<String,Token> = Map::new("tokens");
