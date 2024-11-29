use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub bonding_curve_code_id: u128,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateToken{
        subdenom: String,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
