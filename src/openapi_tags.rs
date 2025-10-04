use poem_openapi::Tags;

#[derive(Tags)]
pub enum APITags {
    Health,
    Users,
}
