use std::fmt::format;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Addr, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response, StdResult, SubMsg, SubMsgResult, Uint128, WasmMsg};
use prost::Message;
// use cw2::set_contract_version;

use crate::bindings::msg::{AppMsg, DenomUnit, Metadata, OsmosisMsg};
use crate::bindings::pb::osmosis::tokenfactory::v1beta1::MsgCreateDenomResponse;
use crate::error::ContractError;
use crate::msg::{BondingCurveInstantiateMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CurrentCreation, Token, CONFIG, CURRENT_CREATION, TOKENS};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:huahua-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

const CREATE_DENOM_REPLY_ID: u64 = 1;
const INSTANTIATE_BONDING_CURVE_REPLY_ID: u64 = 2;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config{
        bonding_curve_code_id: msg.bonding_curve_code_id,
        admin:info.sender
    };
    CONFIG.save(_deps.storage, &config)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<OsmosisMsg>, ContractError> {
    match msg {
        ExecuteMsg::CreateToken { subdenom,description,url } => {
            // create a new token with the given subdenom
            // and return the token_id
            create_token(deps,subdenom,description,url,info.sender)
         
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}


fn create_token(deps: DepsMut, subdenom: String,description:String,url:String,creator:Addr) -> Result<Response<OsmosisMsg>, ContractError>  {
    let create_denom_msg = OsmosisMsg::CreateDenom { subdenom: subdenom.clone() };
    let sub_msg = SubMsg::reply_on_success(create_denom_msg, CREATE_DENOM_REPLY_ID);
    let resp = Response::new()
        .add_attribute("action","create_denom_and_stakedrop")
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
    return Ok(resp)
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response<AppMsg>, ContractError> {

    match msg.id {
        // Cas : retour du premier message
        CREATE_DENOM_REPLY_ID => {


            let mut current_creation = CURRENT_CREATION.load(deps.storage)?;

            match msg.result {
                SubMsgResult::Ok(res) => {
                    if let Some(resp) = res.data {
                        deps.api.debug(&format!("Raw SubMsg data: {:?}", resp));
                        let response: MsgCreateDenomResponse =
                            MsgCreateDenomResponse::decode(resp.as_slice())
                                .map_err(|err| {
                                    deps.api.debug(&format!(
                                        "Failed to decode MsgCreateDenomResponse: {:?}",
                                        err
                                    ));
                                    ContractError::DeserializationError
                                })?;
                        deps.api.debug(&format!(
                            "Decoded MsgCreateDenomResponse: {:?}",
                            response
                        ));

                        deps.api.debug(&format!("Reply data: {:?}", response.new_token_denom.clone()));
                        // Utilisez la valeur retournée pour envoyer un second message


                        let mint_msg = OsmosisMsg::MintTokens {
                            denom: response.new_token_denom.clone(),
                            amount: 21_000_000_000_000u128.into(),
                            mint_to_address: env.contract.address.to_string(),
                        };

                        let set_denom_msg = OsmosisMsg::SetMetadata {
                            denom: response.new_token_denom.clone(),
                            metadata: Metadata {
                                description: "Subdenom".to_string(),
                                denom_units: vec![
                                    DenomUnit {
                                        denom: response.new_token_denom.clone(),
                                        exponent: 0,
                                        aliases: vec![["micro",&current_creation.subdenom.clone()].concat()],
                                    },
                                    DenomUnit {
                                        denom: ["m", &current_creation.subdenom.clone()].concat(),
                                        exponent: 3,
                                        aliases: vec![["milli",&current_creation.subdenom.clone()].concat()],
                                    },
                                    DenomUnit {
                                        denom: current_creation.subdenom.clone(),
                                        exponent: 6,
                                        aliases: vec![],
                                    },
                                ],
                                base: response.new_token_denom.clone(),
                                display: current_creation.subdenom.clone(),
                                name: current_creation.subdenom.clone(),
                                symbol: current_creation.subdenom.clone(),
                                uri: current_creation.url.clone(),
                                uri_hash: "".to_string(),
                            },
                        };
                        let config = CONFIG.load(deps.storage)?;

                        let instantiate_msg = BondingCurveInstantiateMsg {
                            token_denom: response.new_token_denom.clone(),
                            fee_collector_address: config.admin.to_string(),
                        };
                        let bonding_curve_msg = WasmMsg::Instantiate {
                            admin: Some(config.admin.to_string()), // Optionnel : Adresse admin
                            code_id: config.bonding_curve_code_id as u64,
                            msg: to_json_binary(&instantiate_msg)?, // Sérialise le message d'instanciation
                            funds: vec![Coin::new(Uint128::from(12_000_000_000_000u128),response.new_token_denom.clone())],                     // Envoyer des fonds si nécessaire
                            label: "My New Contract".to_string(),
                        };

                        
                        
                        let cosmos_mint_msg:CosmosMsg<AppMsg> = AppMsg::from(mint_msg).into();
                        let cosmos_setmetadata_msg: CosmosMsg<AppMsg> = AppMsg::from(set_denom_msg).into();
                        let cosmos_bonding_curve_msg: CosmosMsg<AppMsg> = AppMsg::from(bonding_curve_msg).into();
                        //let sub_msg: SubMsg<cosmwasm_std::Empty> = SubMsg::reply_on_success(cosmos_bonding_curve_msg, INSTANTIATE_BONDING_CURVE_REPLY_ID);
                        
                        let sub_msg = SubMsg::reply_on_success(
                            cosmos_bonding_curve_msg,
                            INSTANTIATE_BONDING_CURVE_REPLY_ID,
                        );
                        current_creation.denom = response.new_token_denom.clone();
                        CURRENT_CREATION.save(deps.storage, &current_creation)?;


                        return Ok(Response::new()
                            .add_attribute("action", "create_token")
                            .add_attribute("Denom", response.new_token_denom)
                            .add_message(cosmos_setmetadata_msg)
                            .add_message(cosmos_mint_msg)
                            .add_submessage(sub_msg));
                    }else{
                        return Err(ContractError::EmptyResponse);
                    }
                }
                SubMsgResult::Err(err) => {
                    // Gestion des erreurs du sous-message
                    return Err(ContractError::SubMessageError(err))
                }
            }


        }
        INSTANTIATE_BONDING_CURVE_REPLY_ID => {
            // Gestion de la réponse du second message
            match msg.result {
                SubMsgResult::Ok(res) => {
                   
                    if let Some(data) = res.data {
                        let current_creation = CURRENT_CREATION.load(deps.storage)?;

                        let decoded_address: String = String::from_utf8(data.to_vec()).map_err(|err| {
                            deps.api.debug(&format!("Failed to decode contract address: {:?}", err));
                            ContractError::DeserializationError
                        })?;
                        let token = Token {
                            subdenom: current_creation.subdenom.clone(),
                            denom: current_creation.denom.clone(),
                            description: current_creation.description.clone(),
                            url: current_creation.url.clone(),
                            creator: current_creation.creator.clone(),
                            bonding_curve_address: Addr::unchecked(decoded_address.clone()),
                            pool_id: 0,
                        };
                        TOKENS.save(deps.storage, current_creation.subdenom.clone(), &token)?;
                        return Ok(Response::new());
                    }else{
                        return Err(ContractError::EmptyResponse);
                    }
                }
                SubMsgResult::Err(err) => {
                    // Gestion des erreurs du sous-message
                    return Err(ContractError::SubMessageError(err))
                }
            }
        }
        _ => Err(ContractError::CustomError {
            msg: "Unknown reply id".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {}
