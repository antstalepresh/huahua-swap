#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, BankQuery, Binary, Coin, Decimal, Deps, DepsMut, Env,
    MessageInfo, QueryRequest, Response, StdError, StdResult, Uint128, WasmMsg,
};
// use cw2::set_contract_version;

use crate::domain::bonding_curve::BondingCurve;
use crate::error::ContractError;
use crate::msg::{
    CompleteBondingCurve, CompleteBondingCurveMsg, CurveState, ExecuteMsg, InstantiateMsg, QueryMsg,
};
use crate::state::{Config, CONFIG};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:bonding-curve";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    //this contract accepts only 12_000_000_000_000 token_denom for instantiation

    if info.funds.len() != 1 {
        return Err(ContractError::InvalidFunds {});
    } else {
        if info.funds[0].denom != msg.token_denom {
            return Err(ContractError::InvalidFunds {});
        }
        if info.funds[0].amount != Uint128::from(12_000_000_000_000u128) {
            return Err(ContractError::InvalidFunds {});
        }
    }

    match deps.api.addr_validate(&msg.fee_collector_address) {
        Ok(fee_collector_address) => {
            let config = Config {
                token_denom: msg.token_denom.clone(),
                subdenom: msg.subdenom.clone(),
                manager_contract: info.sender.clone(),
                completed: false,
                fee_percent: Decimal::from_ratio(1u128, 100u128),
                fee_collector_address: fee_collector_address,
                token_sold: 0,
                reserve_token_amount: 0,
            };
            // Stockez la configuration dans le state
            CONFIG.save(deps.storage, &config)?;

            Ok(Response::new()
                .add_attribute("action", "instantiate")
                .add_attribute("manager_contract", info.sender)
                .add_attribute("token_denom", config.token_denom))
        }
        Err(_) => {
            return Err(ContractError::InvalidAddress {});
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Buy {} => {
            if info.funds.len() != 1 {
                return Err(ContractError::InvalidFunds {});
            } else {
                if info.funds[0].denom != "uhuahua" {
                    return Err(ContractError::InvalidFunds {});
                }
            }
            execute_buy(deps, env, info.funds[0].amount, info.sender)
        }
        ExecuteMsg::Sell {} => {
            let config = CONFIG.load(deps.storage)?;
            if info.funds.len() != 1 {
                return Err(ContractError::InvalidFunds {});
            } else {
                if info.funds[0].denom != config.token_denom {
                    return Err(ContractError::InvalidFunds {});
                }
            }
            execute_sell(deps, config, env, info.funds[0].amount, info.sender)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TokenPrice {} => {
            let config = CONFIG.load(_deps.storage)?;
            let bonding_curve = BondingCurve::exp_bonding_curve(
                Uint128::from(config.token_sold),
                Uint128::from(config.reserve_token_amount),
            );
            let token_price = bonding_curve.current_price();
            let price = Coin::new(token_price, "uhuahua".to_string());
            to_json_binary(&price)
        }
        QueryMsg::CurveState {} => {
            let config = CONFIG.load(_deps.storage)?;
            let bonding_curve = BondingCurve::exp_bonding_curve(
                Uint128::from(config.token_sold),
                Uint128::from(config.reserve_token_amount),
            );
            let token_price = bonding_curve.current_price();
            let price = Coin::new(token_price, "uhuahua".to_string());
            let token_sold = Coin::new(config.token_sold, config.token_denom);
            let reserve_token_amount =
                Coin::new(config.reserve_token_amount, "uhuahua".to_string());
            to_json_binary(&CurveState {
                sold: token_sold,
                collected: reserve_token_amount,
                completed: config.completed,
                price: price,
            })
        }
        QueryMsg::CalculateBuyAmount { token_amount } => {
            let config = CONFIG.load(_deps.storage)?;
            if token_amount.denom != "uhuahua" {
                return Err(StdError::generic_err("Invalid input token denom"));
            }
            
            let bonding_curve = BondingCurve::exp_bonding_curve(
                Uint128::from(config.token_sold),
                Uint128::from(config.reserve_token_amount),
            );
            
            let fee_amount = calculate_fee(config.clone(), token_amount.amount);
            let amount = token_amount.amount.saturating_sub(fee_amount);
            
            match bonding_curve.calculate_buy_amount(amount) {
                Ok(tokens_bought) => {
                    let output = Coin::new(tokens_bought.u128(), config.token_denom);
                    to_json_binary(&output)
                }
                Err(e) => Err(StdError::generic_err(e)),
            }
        }
        QueryMsg::CalculateSellAmount { token_amount } => {
            let config = CONFIG.load(_deps.storage)?;
            if token_amount.denom != config.token_denom {
                return Err(StdError::generic_err("Invalid token denom"));
            }
            
            let bonding_curve = BondingCurve::exp_bonding_curve(
                Uint128::from(config.token_sold),
                Uint128::from(config.reserve_token_amount),
            );
            
            match bonding_curve.calculate_sell_amount(token_amount.amount) {
                Ok(base_amount) => {
                    // Calculate the fee amount that will be deducted
                    let fee_amount = calculate_fee(config.clone(), base_amount);
                    let final_amount = base_amount.saturating_sub(fee_amount);
                    
                    let output = Coin::new(final_amount.u128(), "uhuahua".to_string());
                    to_json_binary(&output)
                }
                Err(e) => Err(StdError::generic_err(e)),
            }
        }
    }
}

fn execute_buy(
    deps: DepsMut,
    env: Env,
    amount: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    let token_balance = config.token_sold;
    let reserve_token_balance = config.reserve_token_amount;
    let mut bonding_curve = BondingCurve::exp_bonding_curve(
        Uint128::from(token_balance),
        Uint128::from(reserve_token_balance),
    );
    let fee_amount = calculate_fee(config.clone(), amount);
    let amount = amount.saturating_sub(fee_amount);
    let buy_event = bonding_curve.buy(amount);
    match buy_event {
        Ok(bought) => {
            let token_to_send = Coin {
                denom: config.token_denom.clone(),
                amount: bought.tokens_bought,
            };

            // Construire le message pour envoyer des tokens
            let send_msg = BankMsg::Send {
                to_address: sender.to_string(), // Adresse de l'utilisateur
                amount: vec![token_to_send.clone()],
            };

            let send_fee_msg = BankMsg::Send {
                to_address: config.fee_collector_address.to_string(),
                amount: vec![Coin {
                    denom: "uhuahua".to_string(),
                    amount: fee_amount,
                }],
            };

            let mut response = Response::new()
                .add_message(send_msg)
                .add_message(send_fee_msg) // Ajouter le message BankMsg::Send
                .add_attribute("action", "buy")
                .add_attribute("buyer", sender.to_string())
                .add_attribute("amount", token_to_send.amount.to_string())
                .add_attribute("denom", token_to_send.denom);

            config.reserve_token_amount += amount.u128();
            if bought.rest_native_amount > Uint128::zero() {
                //return rest to user
                let send_msg = BankMsg::Send {
                    to_address: sender.to_string(), // Adresse de l'utilisateur
                    amount: vec![Coin {
                        denom: "uhuahua".to_string(),
                        amount: bought.rest_native_amount,
                    }],
                };
                response = response.add_message(send_msg);
                config.reserve_token_amount -= bought.rest_native_amount.u128();
            }

            config.token_sold += bought.tokens_bought.u128();

            if config.token_sold == 12_000_000_000_000u128 {
                config.completed = true;
                let complete_msg = CompleteBondingCurveMsg {
                    complete_bonding_curve: CompleteBondingCurve {
                        subdenom: config.subdenom.to_string(),
                    },
                };

                let huahua_balance: Coin = deps
                    .querier
                    .query_balance(env.contract.address, "uhuahua".to_string())?;
                let execute_complete_bonding_curve_msg = WasmMsg::Execute {
                    contract_addr: config.manager_contract.to_string(),
                    msg: to_json_binary(&complete_msg)?,
                    funds: vec![Coin::new(
                        huahua_balance
                            .amount
                            .saturating_sub(bought.rest_native_amount)
                            .saturating_sub(fee_amount),
                        "uhuahua".to_string(),
                    )],
                };
                response = response.add_message(execute_complete_bonding_curve_msg);
            }
            CONFIG.save(deps.storage, &config)?;

            Ok(response)
        }
        Err(error) => {
            return Err(ContractError::CustomError(error));
        }
    }
}

fn execute_sell(
    deps: DepsMut,
    mut config: Config,
    _env: Env,
    amount: Uint128,
    sender: Addr,
) -> Result<Response, ContractError> {
    let token_balance = config.token_sold;
    let reserve_token_balance = config.reserve_token_amount;
    let mut bonding_curve = BondingCurve::exp_bonding_curve(
        Uint128::from(token_balance),
        Uint128::from(reserve_token_balance),
    );
    let sell_event = bonding_curve.sell(amount);

    match sell_event {
        Ok(sold) => {
            let fee_amount = calculate_fee(config.clone(), sold.reserve_token_bought);
            let amount_to_send = sold.reserve_token_bought.saturating_sub(fee_amount);

            config.token_sold -= amount.u128();
            config.reserve_token_amount -= sold.reserve_token_bought.u128();
            CONFIG.save(deps.storage, &config)?;

            let token_to_send = Coin {
                denom: "uhuahua".to_string(),
                amount: amount_to_send,
            };

            let send_fee_msg = BankMsg::Send {
                to_address: config.fee_collector_address.to_string(),
                amount: vec![Coin {
                    denom: "uhuahua".to_string(),
                    amount: fee_amount,
                }],
            };

            // Construire le message pour envoyer des tokens
            let send_msg = BankMsg::Send {
                to_address: sender.to_string(), // Adresse de l'utilisateur
                amount: vec![token_to_send.clone()],
            };

            let response = Response::new()
                .add_message(send_msg) // Ajouter le message BankMsg::Send
                .add_message(send_fee_msg)
                .add_attribute("action", "sell")
                .add_attribute("seller", sender.to_string())
                .add_attribute("amount", token_to_send.amount.to_string())
                .add_attribute("denom", token_to_send.denom);

            Ok(response)
        }
        Err(error) => {
            return Err(ContractError::CustomError(error));
        }
    }
}

fn calculate_fee(config: Config, amount: Uint128) -> Uint128 {
    let fee_decimal = Decimal::from_ratio(amount, Uint128::one()) * config.fee_percent;
    let fee = fee_decimal.to_uint_floor();
    fee
}

#[cfg(test)]
mod tests {}
