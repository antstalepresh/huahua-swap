use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub token_denom: String,
    pub subdenom: String,
    pub fee_collector_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Buy {},
    Sell {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Coin)]
    TokenPrice {},
    #[returns(CurveState)]
    CurveState {},
    #[returns(Coin)]
    CalculateBuyAmount { token_amount: Coin },
    #[returns(Coin)]
    CalculateSellAmount { token_amount: Coin },
}

#[cw_serde]
pub struct CurveState {
    pub sold: Coin,
    pub collected: Coin,
    pub completed: bool,
    pub price: Coin,
}

#[cw_serde]
pub struct CompleteBondingCurveMsg {
    pub complete_bonding_curve: CompleteBondingCurve,
}
#[cw_serde]
pub struct CompleteBondingCurve {
    pub subdenom: String,
}
