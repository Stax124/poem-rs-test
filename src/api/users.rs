use crate::{models, openapi_tags::APITags};
use argon2::password_hash::{PasswordHasher, SaltString, rand_core::OsRng};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use poem::web::Data;
use poem_openapi::{OpenApi, payload::Json};

pub struct UserApi;

#[OpenApi(tag = "APITags::Users")]
impl UserApi {
    /// Create a new user
    #[oai(path = "/users", method = "post")]
    async fn create_user(
        &self,
        connection: Data<&Pool<ConnectionManager<diesel::PgConnection>>>,
        body: Json<models::user::NewUser>,
    ) -> Json<serde_json::Value> {
        let conn = &mut connection.get().unwrap();
        let new_user = body.0;

        // Check if email or username already exists
        let existing_user = crate::schema::users::table
            .filter(
                crate::schema::users::email
                    .eq(&new_user.email)
                    .or(crate::schema::users::username.eq(&new_user.username)),
            )
            .first::<models::user::User>(conn)
            .optional()
            .expect("Error checking for existing user");

        if existing_user.is_some() {
            return Json(
                serde_json::json!({ "status": "error", "message": "Email or username already exists"}),
            );
        }

        // Hash the password before storing with argon2
        let argon2 = argon2::Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = argon2
            .hash_password(new_user.password.as_bytes(), &salt)
            .expect("Error hashing password")
            .to_string();

        let new_user = models::user::NewUser {
            email: new_user.email,
            username: new_user.username,
            password: hashed_password,
        };

        // Insert the new user into the database
        diesel::insert_into(crate::schema::users::table)
            .values(&new_user)
            .execute(conn)
            .expect("Error inserting new user");

        Json(serde_json::json!({"status": "user created"}))
    }
    /// Get user by ID
    /// This endpoint retrieves a user by their unique ID.
    #[oai(path = "/users/:id", method = "get")]
    async fn get_user(&self) -> Json<serde_json::Value> {
        Json(serde_json::json!({"id": 1, "name": "John Doe"}))
    }
}
