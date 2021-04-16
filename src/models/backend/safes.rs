use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SafeInfo {
    pub address: String,
    pub nonce: u64,
    pub threshold: u64,
    pub owners: Vec<String>,
    pub master_copy: String,
    pub modules: Option<Vec<String>>,
    pub fallback_handler: Option<String>,
    pub version: Option<String>,
}
