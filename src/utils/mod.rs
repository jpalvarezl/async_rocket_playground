use crate::utils::errors::ApiResult;
use crate::models::backend::safes::SafeInfo;
use rocket::State;

pub mod errors;
pub mod context;

pub struct InfoProvider<'a> {
    client: State<'a, reqwest::Client>
}

impl<'a> InfoProvider<'a> {
    pub fn from(client: State<'a, reqwest::Client>) -> Self {
        InfoProvider { client }
    }

    pub async fn safe_info(&self, safe_address: &String) -> ApiResult<SafeInfo> {
        let request_url = format!("https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/safes/{}/", safe_address);
        log::error!("request service URL: {}", &request_url);
        let response = self.client.get(&request_url).send().await?;
        let safe_info = serde_json::from_str::<SafeInfo>(&response.text().await?)?;
        Ok(safe_info)
    }
}
