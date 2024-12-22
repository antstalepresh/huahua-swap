use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::{Config, Token};

#[cw_serde]
pub struct InstantiateMsg {
    pub bonding_curve_code_id: u128,
    pub fee_swap_collector_address: String,
    pub reserve_collector_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateToken {
        subdenom: String,
        description: String,
        url: String,
    },
    CompleteBondingCurve {
        subdenom: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TokenInfoResponse)]
    TokenInfo { subdenom: String },
    #[returns(PaginatedTokensResponse)]
    GetTokensWithPagination {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct TokenInfoResponse {
    pub info: Token,
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct PaginatedTokensResponse {
    pub tokens: Vec<Token>,
}

#[cw_serde]
pub struct BondingCurveInstantiateMsg {
    pub token_denom: String,
    pub subdenom: String,
    pub fee_collector_address: String,
}
