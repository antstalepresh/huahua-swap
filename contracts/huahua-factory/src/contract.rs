#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, SubMsg, SubMsgResult};
use prost::Message;
// use cw2::set_contract_version;

use crate::bindings::msg::{DenomUnit, Metadata, OsmosisMsg};
use crate::bindings::pb::osmosis::tokenfactory::v1beta1::MsgCreateDenomResponse;
use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, CURRENT_SUBDENOM_CREATION};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:huahua-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

const CREATE_DENOM_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config{
        bonding_curve_code_id: msg.bonding_curve_code_id,
    };
    CONFIG.save(_deps.storage, &config)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<OsmosisMsg>, ContractError> {
    match msg {
        ExecuteMsg::CreateToken { subdenom } => {
            // create a new token with the given subdenom
            // and return the token_id
            create_token(deps,subdenom)
         
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}


fn create_token(deps: DepsMut, subdenom: String) -> Result<Response<OsmosisMsg>, ContractError>  {
    let create_denom_msg = OsmosisMsg::CreateDenom { subdenom: subdenom.clone() };
    let sub_msg = SubMsg::reply_on_success(create_denom_msg, CREATE_DENOM_REPLY_ID);
    let resp = Response::new()
        .add_attribute("action","create_denom_and_stakedrop")
      //  .add_attribute("token factory params ", format!("{:?}",resp.params))
        .add_submessage(sub_msg);

    CURRENT_SUBDENOM_CREATION.save(deps.storage, &subdenom)?;
    return Ok(resp)
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response<OsmosisMsg>, ContractError> {

    match msg.id {
        // Cas : retour du premier message
        CREATE_DENOM_REPLY_ID => {


            let current_subdenom = CURRENT_SUBDENOM_CREATION.load(deps.storage)?;

            match msg.result {
                SubMsgResult::Ok(res) => {
                    if let Some(resp) = res.msg_responses.get(0) {
                        deps.api.debug(&format!("Raw SubMsg data: {:?}", resp.value));
                        let response: MsgCreateDenomResponse =
                            MsgCreateDenomResponse::decode(resp.value.as_slice())
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
                            amount: 1000000u128.into(),
                            mint_to_address: env.contract.address.to_string(),
                        };

                        let set_denom_msg = OsmosisMsg::SetMetadata {
                            denom: response.new_token_denom.clone(),
                            metadata: Metadata {
                                description: "Subdenom".to_string(),
                                denom_units: DenomUnit {
                                    denom: ["u", &current_subdenom].concat(),
                                    exponent: 6,
                                    aliases: vec![],
                                },
                                base: response.new_token_denom.clone(),
                                display: current_subdenom.clone(),
                                name: current_subdenom.clone(),
                                symbol: current_subdenom.clone(),
                                uri: "".to_string(),
                                uri_hash: "".to_string(),
                            },
                        };


                        return Ok(Response::new()
                            .add_attribute("action", "create_stakedrop")
                            .add_attribute("Denom", response.new_token_denom)
                            .add_message(CosmosMsg::from(create_stakedrop_msg))
                            .add_message(CosmosMsg::from(mint_msg))
                            .add_message(CosmosMsg::from(create_pool_msg)));
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
