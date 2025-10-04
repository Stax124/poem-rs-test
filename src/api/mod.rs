use poem_openapi::{OpenApi, payload::PlainText};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/healthcheck", method = "get")]
    async fn healthcheck(&self) -> PlainText<String> {
        PlainText("HEALTHY".to_string())
    }
}
