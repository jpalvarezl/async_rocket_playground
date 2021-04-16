// use crate::cache::cache_operations::CacheResponse;
// use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use crate::service::safes::get_safe;

#[get("/safes/<safe_address>")]
//context: Context,
pub async fn safe_info(safe_address: String) -> ApiResult<content::Json<String>> {
    // CacheResponse::new(context.uri())
    //     .resp_generator(|| get_safe_info_ex(&context, &safe_address))
    //     .execute(context.cache())
    Ok(content::Json(serde_json::to_string(&get_safe(safe_address).await?)?))
}
