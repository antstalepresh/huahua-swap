use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub bonding_curve_code_id: u128,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateToken{
        subdenom: String,
        description:String,
        url:String,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}



#[cw_serde]
pub struct BondingCurveInstantiateMsg {
    pub token_denom: String,
    pub fee_collector_address: String,
}