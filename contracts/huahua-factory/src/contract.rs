#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply,
    Response, StdResult, SubMsg, SubMsgResult, Uint128, WasmMsg,
};
use cw_storage_plus::Bound;
use prost::Message;
// use cw2::set_contract_version;

use crate::bindings::msg::MsgInstantiateContractResponse;
use crate::bindings::pb::cosmos::bank::v1beta1::{DenomUnit, Metadata};
use crate::bindings::pb::cosmos::base;
use crate::bindings::pb::liquidity::v1beta1::MsgCreatePool;
use crate::bindings::pb::osmosis::tokenfactory::v1beta1::{
    MsgCreateDenom, MsgCreateDenomResponse, MsgCreateStakeDrop, MsgMint, MsgSetDenomMetadata,
};
use crate::error::ContractError;
use crate::msg::{
    BondingCurveInstantiateMsg, ConfigResponse, ExecuteMsg, InstantiateMsg,
    PaginatedTokensResponse, QueryMsg, TokenInfoResponse,
};
use crate::state::{Config, CurrentCreation, Token, CONFIG, CURRENT_CREATION, TOKENS};

const MAX_PAGINATION_LIMIT: u32 = 500;
const DEFAULT_PAGINATION_LIMIT: u32 = 100;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:huahua-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

const CREATE_DENOM_REPLY_ID: u64 = 1;
const INSTANTIATE_BONDING_CURVE_REPLY_ID: u64 = 2;
const WOOF_BLOCK_NUMBER: i64 = 2315156;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let fee_swap_collector_address = deps
        .api
        .addr_validate(msg.fee_swap_collector_address.clone().as_ref())?;
    let reserve_collector_address = deps
        .api
        .addr_validate(msg.reserve_collector_address.clone().as_ref())?;
    let config = Config {
        bonding_curve_code_id: msg.bonding_curve_code_id,
        admin: info.sender,
        fee_swap_collector_address: fee_swap_collector_address,
        reserve_collector_address: reserve_collector_address,
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateToken {
            subdenom,
            description,
            url,
        } => create_token(deps, env, subdenom, description, url, info.sender),
        ExecuteMsg::CompleteBondingCurve { subdenom } => {
            let mut token = TOKENS.load(deps.storage, subdenom.clone())?;
            if token.completed {
                return Err(ContractError::CustomError {
                    msg: "Token already completed".to_string(),
                });
            } else {
                if info.funds.is_empty() {
                    return Err(ContractError::InvalidFunds {});
                }
                if info.funds.len() > 1 {
                    return Err(ContractError::InvalidFunds {});
                }
                let reserve = info.funds[0].clone();
                if reserve.denom != "uhuahua" {
                    return Err(ContractError::InvalidFunds {});
                }
                if reserve.amount <= Uint128::from(20_000_000_000_000u128) {
                    //amount should be 60M but at least 20M
                    return Err(ContractError::InvalidFunds {});
                }
                let config = CONFIG.load(deps.storage)?;
                if token.completed {
                    return Err(ContractError::CustomError {
                        msg: "Token already completed".to_string(),
                    });
                }
                if token.bonding_curve_address != info.sender {
                    return Err(ContractError::Unauthorized {});
                }
                //Stakedrop issued token from bonding curve
                let create_stakedrop_msg = MsgCreateStakeDrop {
                    sender: env.contract.address.to_string(),
                    amount: Some(base::v1beta1::Coin {
                        denom: token.denom.clone(),
                        amount: Uint128::from(5_000_000_000_000u128).to_string(),
                    }),
                    start_block: env.block.height as i64 + 2,
                    end_block: env.block.height as i64 + 2 + WOOF_BLOCK_NUMBER,
                };

                let mut create_stakedrop_data = Vec::new();
                create_stakedrop_msg
                    .encode(&mut create_stakedrop_data)
                    .map_err(|err| ContractError::CustomError {
                        msg: format!("Encode error: {:?}", err),
                    })?;
                let create_stakedrop: CosmosMsg = CosmosMsg::Any(cosmwasm_std::AnyMsg {
                    type_url: "/osmosis.tokenfactory.v1beta1.MsgCreateStakeDrop".to_string(),
                    value: Binary::from(create_stakedrop_data).into(),
                });

                //stakedrop native token
                let create_native_stakedrop_msg = MsgCreateStakeDrop {
                    sender: env.contract.address.to_string(),
                    amount: Some(base::v1beta1::Coin {
                        denom: "uhuahua".to_string(),
                        amount: reserve
                            .amount
                            .saturating_sub(Uint128::from(20_000_000_000_000u128))
                            .to_string(),
                    }),
                    start_block: env.block.height as i64 + 2,
                    end_block: env.block.height as i64 + 2 + WOOF_BLOCK_NUMBER,
                };

                let mut create_native_stakedrop_data = Vec::new();
                create_native_stakedrop_msg
                    .encode(&mut create_native_stakedrop_data)
                    .map_err(|err| ContractError::CustomError {
                        msg: format!("Encode error: {:?}", err),
                    })?;
                let create_native_stakedrop: CosmosMsg = CosmosMsg::Any(cosmwasm_std::AnyMsg {
                    type_url: "/osmosis.tokenfactory.v1beta1.MsgCreateStakeDrop".to_string(),
                    value: Binary::from(create_native_stakedrop_data).into(),
                });

                let create_pool_msg = MsgCreatePool {
                    pool_creator_address: env.contract.address.to_string(),
                    pool_type_id: 1,
                    deposit_coins: vec![
                        base::v1beta1::Coin {
                            denom: token.denom.clone(),
                            amount: Uint128::from(4_000_000_000_000u128).to_string(),
                        },
                        base::v1beta1::Coin {
                            denom: "uhuahua".to_string(),
                            amount: Uint128::from(20_000_000_000_000u128).to_string(),
                        },
                    ],
                };

                let mut create_pool_data = Vec::new();
                create_pool_msg.encode(&mut create_pool_data).unwrap();
                let create_pool: CosmosMsg = CosmosMsg::Any(cosmwasm_std::AnyMsg {
                    type_url: "/liquidity.v1beta1.MsgCreatePool".to_string(),
                    value: Binary::from(create_pool_data).into(),
                });

                token.completed = true;
                TOKENS.save(deps.storage, subdenom.clone(), &token)?;

                let resp = Response::new()
                    .add_attribute("action", "create_denom_and_stakedrop")
                    //  .add_attribute("token factory params ", format!("{:?}",resp.params))
                    .add_message(create_stakedrop)
                    .add_message(create_native_stakedrop)
                    .add_message(create_pool);
                Ok(resp)
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::TokenInfo { subdenom } => {
            let token = TOKENS.load(deps.storage, subdenom)?;
            to_json_binary(&TokenInfoResponse { info: token })
        }
        QueryMsg::GetTokensWithPagination { start_after, limit } => {
            let response = query_tokens_with_pagination(deps, start_after, limit)?;
            cosmwasm_std::to_json_binary(&response)
        }
        QueryMsg::Config {} => {
            let config = CONFIG.load(deps.storage)?;
            let response = ConfigResponse { config };
            to_json_binary(&response)
        }
    }
}

pub fn query_tokens_with_pagination(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<PaginatedTokensResponse> {
    // Calculer la limite réelle
    let limit = limit
        .unwrap_or(DEFAULT_PAGINATION_LIMIT)
        .min(MAX_PAGINATION_LIMIT);

    // Créer la limite de départ
    let start = start_after.map(Bound::exclusive);

    // Obtenir les tokens de la map avec pagination
    let tokens: Vec<Token> = TOKENS
        .range(deps.storage, start, None, cosmwasm_std::Order::Ascending)
        .take(limit as usize)
        .map(|item| item.map(|(_, token)| token)) // Ignorer les clés, ne garder que les valeurs
        .collect::<StdResult<_>>()?;

    Ok(PaginatedTokensResponse { tokens })
}

fn create_token(
    deps: DepsMut,
    env: Env,
    subdenom: String,
    description: String,
    url: String,
    creator: Addr,
) -> Result<Response, ContractError> {
    let create_denom_msg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: subdenom.clone(),
    };
    let mut data = Vec::new();
    create_denom_msg.encode(&mut data).unwrap();
    let stargate_msg = CosmosMsg::Any(cosmwasm_std::AnyMsg {
        type_url: "/osmosis.tokenfactory.v1beta1.MsgCreateDenom".to_string(),
        value: Binary::from(data).into(),
    });
    let sub_msg = SubMsg::reply_on_success(stargate_msg, CREATE_DENOM_REPLY_ID);
    let resp = Response::new()
        .add_attribute("action", "create_denom_and_stakedrop")
        //  .add_attribute("token factory params ", format!("{:?}",resp.params))
        .add_submessage(sub_msg);

    let current_creation = CurrentCreation {
        subdenom: subdenom.clone(),
        description: description.clone(),
        url: url.clone(),
        creator: creator.clone(),
        denom: "".to_string(),
    };
    CURRENT_CREATION.save(deps.storage, &current_creation)?;
    return Ok(resp);
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        // Cas : retour du premier message
        CREATE_DENOM_REPLY_ID => {
            let mut current_creation = CURRENT_CREATION.load(deps.storage)?;

            match msg.result {
                SubMsgResult::Ok(res) => {
                    if let Some(resp) = res.data {
                        deps.api.debug(&format!("Raw SubMsg data: {:?}", resp));
                        let response: MsgCreateDenomResponse =
                            MsgCreateDenomResponse::decode(resp.as_slice()).map_err(|err| {
                                deps.api.debug(&format!(
                                    "Failed to decode MsgCreateDenomResponse: {:?}",
                                    err
                                ));
                                ContractError::DeserializationError
                            })?;
                        deps.api
                            .debug(&format!("Decoded MsgCreateDenomResponse: {:?}", response));

                        deps.api.debug(&format!(
                            "Reply data: {:?}",
                            response.new_token_denom.clone()
                        ));

                        let mint_msg = create_mint_msg(
                            response.new_token_denom.clone(),
                            env.contract.address.to_string(),
                        );

                        let set_metadata_msg = create_set_denom_metadata_msg(
                            current_creation.clone(),
                            response.new_token_denom.clone(),
                            env.contract.address.to_string(),
                        );

                        let config = CONFIG.load(deps.storage)?;
                        let bonding_curve_msg = create_bonding_curve_instantiate_msg(
                            response.new_token_denom.clone(),
                            current_creation.clone(),
                            config.clone(),
                        )?;
                        current_creation.denom = response.new_token_denom.clone();
                        CURRENT_CREATION.save(deps.storage, &current_creation)?;

                        return Ok(Response::new()
                            .add_attribute("action", "create_token")
                            .add_attribute("Denom", response.new_token_denom)
                            .add_message(set_metadata_msg)
                            .add_message(mint_msg)
                            .add_submessage(bonding_curve_msg));
                    } else {
                        return Err(ContractError::EmptyResponse);
                    }
                }
                SubMsgResult::Err(err) => {
                    // Gestion des erreurs du sous-message
                    return Err(ContractError::SubMessageError(
                        ["error from create denom".to_string(), err].concat(),
                    ));
                }
            }
        }
        INSTANTIATE_BONDING_CURVE_REPLY_ID => {
            // Gestion de la réponse du second message
            match msg.result {
                SubMsgResult::Ok(res) => {
                    if let Some(data) = res.data {
                        let current_creation = CURRENT_CREATION.load(deps.storage)?;

                        let instantiate_response: MsgInstantiateContractResponse =
                            MsgInstantiateContractResponse::decode(data.as_slice()).map_err(
                                |err| {
                                    deps.api.debug(&format!(
                                        "Failed to decode contract address: {:?}",
                                        err
                                    ));
                                    ContractError::DeserializationError
                                },
                            )?;
                        let token = Token {
                            subdenom: current_creation.subdenom.clone(),
                            denom: current_creation.denom.clone(),
                            description: current_creation.description.clone(),
                            url: current_creation.url.clone(),
                            creator: current_creation.creator.clone(),
                            bonding_curve_address: Addr::unchecked(
                                instantiate_response.address.clone(),
                            ),
                            completed: false,
                            pool_id: 0,
                        };
                        TOKENS.save(deps.storage, current_creation.subdenom.clone(), &token)?;
                        return Ok(Response::new());
                    } else {
                        return Err(ContractError::EmptyResponse);
                    }
                }
                SubMsgResult::Err(err) => {
                    // Gestion des erreurs du sous-message
                    return Err(ContractError::SubMessageError(
                        ["error from create denom".to_string(), err].concat(),
                    ));
                }
            }
        }
        _ => Err(ContractError::CustomError {
            msg: "Unknown reply id".to_string(),
        }),
    }
}

fn create_mint_msg(new_token_denom: String, contract_address: String) -> CosmosMsg {
    let mint_msg = MsgMint {
        sender: contract_address.clone(),
        amount: Some(base::v1beta1::Coin {
            denom: new_token_denom.clone(),
            amount: Uint128::from(16_000_000_000_000u128).to_string(),
        }),
        mint_to_address: contract_address.clone(),
    };

    let mut mint_msg_data = Vec::new();
    mint_msg.encode(&mut mint_msg_data).unwrap();
    CosmosMsg::Any(cosmwasm_std::AnyMsg {
        type_url: "/osmosis.tokenfactory.v1beta1.MsgMint".to_string(),
        value: Binary::from(mint_msg_data).into(),
    })
}

fn create_bonding_curve_instantiate_msg(
    new_token_denom: String,
    current_creation: CurrentCreation,
    config: Config,
) -> Result<SubMsg, ContractError> {
    let instantiate_msg = BondingCurveInstantiateMsg {
        token_denom: new_token_denom.clone(),
        subdenom: current_creation.subdenom.clone(),
        fee_collector_address: config.fee_swap_collector_address.to_string(),
    };
    let bonding_curve_msg = WasmMsg::Instantiate {
        admin: Some(config.admin.to_string()), // Optionnel : Adresse admin
        code_id: config.bonding_curve_code_id as u64,
        msg: to_json_binary(&instantiate_msg)?, // Sérialise le message d'instanciation
        funds: vec![Coin::new(
            Uint128::from(12_000_000_000_000u128),
            new_token_denom.clone(),
        )], // Envoyer des fonds si nécessaire
        label: [
            current_creation.subdenom.clone(),
            "-bondingcurve".to_string(),
        ]
        .concat(),
    };

    Ok(SubMsg::reply_on_success(
        bonding_curve_msg,
        INSTANTIATE_BONDING_CURVE_REPLY_ID,
    ))
}

fn create_set_denom_metadata_msg(
    current_creation: CurrentCreation,
    new_token_denom: String,
    contract_address: String,
) -> CosmosMsg {
    let set_denom_msg = MsgSetDenomMetadata {
        sender: contract_address,
        metadata: Some(Metadata {
            description: "Subdenom".to_string(),
            denom_units: vec![
                DenomUnit {
                    denom: new_token_denom.clone(),
                    exponent: 0,
                    aliases: vec![["micro", &current_creation.subdenom.clone()].concat()],
                },
                DenomUnit {
                    denom: ["m", &current_creation.subdenom.clone()].concat(),
                    exponent: 3,
                    aliases: vec![["milli", &current_creation.subdenom.clone()].concat()],
                },
                DenomUnit {
                    denom: current_creation.subdenom.clone(),
                    exponent: 6,
                    aliases: vec![],
                },
            ],
            base: new_token_denom.clone(),
            display: current_creation.subdenom.clone(),
            name: current_creation.subdenom.clone(),
            symbol: current_creation.subdenom.clone(),
            uri: current_creation.url.clone(),
            uri_hash: "".to_string(),
        }),
    };

    let mut set_metadata_data = Vec::new();
    set_denom_msg.encode(&mut set_metadata_data).unwrap();
    CosmosMsg::Any(cosmwasm_std::AnyMsg {
        type_url: "/osmosis.tokenfactory.v1beta1.MsgSetDenomMetadata".to_string(),
        value: Binary::from(set_metadata_data).into(),
    })
}

#[cfg(test)]
mod tests {}
