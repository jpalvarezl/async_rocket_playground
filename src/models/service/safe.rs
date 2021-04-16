use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Safe {
    pub address: Address,
    pub nonce: u64,
    pub threshold: u64,
    pub owners: Vec<Address>,
    pub implementation: Address,
    pub modules: Option<Vec<Address>>,
    pub fallback_handler: Option<Address>,
    pub version: Option<String>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
}
