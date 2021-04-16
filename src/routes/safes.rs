// use crate::cache::cache_operations::CacheResponse;
// use crate::services::safes::get_safe_info_ex;
// use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/safes/<safe_address>")]
//context: Context,
pub fn safe_info(safe_address: String) -> ApiResult<content::Json<String>> {
    // CacheResponse::new(context.uri())
    //     .resp_generator(|| get_safe_info_ex(&context, &safe_address))
    //     .execute(context.cache())
    Ok(content::Json(
        json!({
            "output" : "reached"
        })
        .to_string(),
    ))
}