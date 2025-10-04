use crate::openapi_tags::APITags;
use poem_openapi::{OpenApi, payload::Json};

pub struct HealthCheckApi;

#[OpenApi(tag = "APITags::Health")]
impl HealthCheckApi {
    #[oai(path = "/healthcheck", method = "get")]
    async fn healthcheck(&self) -> Json<serde_json::Value> {
        Json(serde_json::json!({"status": "ok"}))
    }
}
