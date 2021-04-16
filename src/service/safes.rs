use crate::utils::errors::ApiResult;
use crate::models::service::safe::{Safe, Address};
use crate::utils::InfoProvider;

pub async fn get_safe(info_provider: InfoProvider<'_>, safe_address: String) -> ApiResult<Safe> {
    let safe_info = info_provider.safe_info(&safe_address).await?;
    log::debug!("{:#?}", safe_info);
    Ok(Safe {
        address: Address {
            value: safe_address,
            name: None,
            logo_url: None,
        },
        nonce: 0,
        threshold: 0,
        owners: vec![],
        implementation: Address {
            value: "implementation".to_string(),
            name: None,
            logo_url: None,
        },
        modules: None,
        fallback_handler: None,
        version: None,
    })
}
