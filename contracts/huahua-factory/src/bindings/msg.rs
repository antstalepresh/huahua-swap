#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInstantiateContractResponse {
    #[prost(string, tag = "1")]
    pub address: String,
    #[prost(bytes, tag = "2")]
    pub data: Vec<u8>,
}
