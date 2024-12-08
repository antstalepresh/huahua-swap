use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Empty response from sub-message")]
    EmptyResponse,

    #[error("Invalid funds")]
    InvalidFunds {},

    #[error("Error during sub-message execution: {0}")]
    SubMessageError(String),

    #[error("Custom Error val: {msg:?}")]
    CustomError { msg: String },

    #[error("Failed to deserialize response")]
    DeserializationError,
}
