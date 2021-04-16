use crate::utils::errors::ApiResult;
use crate::models::service::safe::{Safe, Address};

pub async fn get_safe(safe_address: String) -> ApiResult<Safe> {
    Ok(Safe{
        address: Address {
            value: safe_address,
            name: None,
            logo_url: None
        },
        nonce: 0,
        threshold: 0,
        owners: vec![],
        implementation: Address {
            value: "implementation".to_string(),
            name: None,
            logo_url: None
        },
        modules: None,
        fallback_handler: None,
        version: None
    })
}
