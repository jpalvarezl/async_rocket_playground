// use crate::cache::cache_operations::CacheResponse;
// use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use crate::service::safes::get_safe;
use crate::utils::context::Context;

#[get("/safes/<safe_address>")]
//context: Context,
pub async fn safe_info(context: Context<'_>, safe_address: String) -> ApiResult<content::Json<String>> {
    Ok(content::Json(serde_json::to_string(&get_safe(context.info_provider(), safe_address).await?)?))
}
