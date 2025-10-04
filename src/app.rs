use poem::Route;
use poem_openapi::OpenApiService;

use crate::api;

pub fn init_app(server_port: u16) -> Route {
    let api_service = OpenApiService::new(api::Api, "My API", "1.0")
        .server(&format!("http://localhost:{}", server_port));
    let ui = api_service.swagger_ui();
    Route::new().nest("/", api_service).nest("/docs", ui)
}
