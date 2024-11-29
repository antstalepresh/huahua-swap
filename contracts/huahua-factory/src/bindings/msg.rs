use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg, Uint128};




/// A number of Custom messages that can call into the Osmosis bindings
#[cw_serde]
pub enum OsmosisMsg {
    /// CreateDenom creates a new factory denom, of denomination:
    /// factory/{creating contract bech32 address}/{Subdenom}
    /// Subdenom can be of length at most 44 characters, in [0-9a-zA-Z./]
    /// Empty subdenoms are valid.
    /// The (creating contract address, subdenom) pair must be unique.
    /// The created denom's admin is the creating contract address,
    /// but this admin can be changed using the UpdateAdmin binding.
    CreateDenom { subdenom: String },
    /// ChangeAdmin changes the admin for a factory denom.
    /// Can only be called by the current contract admin.
    /// If the NewAdminAddress is empty, the denom will have no admin.
    ChangeAdmin {
        denom: String,
        new_admin_address: String,
    },
    /// Contracts can mint native tokens for an existing factory denom
    /// that they are the admin of.
    MintTokens {
        denom: String,
        amount: Uint128,
        mint_to_address: String,
    },
    /// Contracts can burn native tokens for an existing factory denom
    /// that they are the admin of.
    /// Currently, the burn from address must be the admin contract.
    BurnTokens {
        denom: String,
        amount: Uint128,
        burn_from_address: String,
    },
    CreateStakedrop {
        denom: String,
        amount: Uint128,
        start_block: i64,
        end_block: i64,
    },
    CreatePool {
        pool_creator_address: String,
        pool_type_id    : u32,
        amount1: Uint128,
        denom1: String,
        amount2: Uint128,
        denom2: String
    },
    SetMetadata {
        denom: String,
        metadata: Metadata,
    }


}

#[cw_serde]
pub struct Metadata {
    pub description : String,
    pub denom_units: DenomUnit,
    pub base: String,
    pub display: String,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub uri_hash: String,
}

#[cw_serde]
pub struct DenomUnit {
    pub denom: String,
    pub exponent: u32,
    pub aliases: Vec<String>,
}

impl OsmosisMsg {


    pub fn mint_contract_tokens(denom: String, amount: Uint128, mint_to_address: String) -> Self {
        OsmosisMsg::MintTokens {
            denom,
            amount,
            mint_to_address,
        }
    }

    pub fn burn_contract_tokens(
        denom: String,
        amount: Uint128,
        _burn_from_address: String,
    ) -> Self {
        OsmosisMsg::BurnTokens {
            denom,
            amount,
            burn_from_address: "".to_string(), // burn_from_address is currently disabled.
        }
    }
}

impl From<OsmosisMsg> for CosmosMsg<OsmosisMsg> {
    fn from(msg: OsmosisMsg) -> CosmosMsg<OsmosisMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for OsmosisMsg {}