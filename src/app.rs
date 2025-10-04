use poem::Route;
use poem_openapi::OpenApiService;

use crate::api;

pub fn init_app(server_port: u16) -> Route {
    let api_service = OpenApiService::new(
        (api::healthcheck::HealthCheckApi, api::users::UserApi),
        "My API",
        "1.0",
    )
    .server(&format!("http://localhost:{}/api", server_port));
    let ui = api_service.swagger_ui();
    Route::new().nest("/api", api_service).nest("/api/docs", ui)
}
