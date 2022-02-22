use poem::web::Data;
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    Object, OpenApi,
};
use serde_json::{
    Value,
    json
};
use sqlx::{postgres::PgPool, Error};

/// # API design
/// - [ ] post "/publish"
/// - [ ] def form_exists_for_user?(user, key)
/// - [ ] get "/published"
/// - [ ] get "/published/:id"
/// - [ ] def authenticated_user
/// - [ ] def forms_for_user(user)
///
/// - [ ] def seed_data_for_user(user)
/// - [ ] get "/seed/:user" (optional, designer expects forms to exist)

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/publish", method = "post")]
    async fn create_user(&self, request: Json<Request>) -> Json<String> {
        if form_exists_for_user("test user".to_string(), "test  key".to_string()) {
            //update forms where user and id with config=request.configuration
        } else {
            //update forms where user and id with config=request.configuration and display_name=id
        }
        Json(request.configuration.to_string())
    }

    #[oai(path = "/published/:id", method = "get")]
    async fn published(&self, data_pool: Data<&PgPool>, id: Path<i64>) -> FormResponse {
        let forms: Vec<Form> = sqlx::query_as!(Form, "SELECT * FROM forms WHERE id=$1;", id.0)
            .fetch_all(data_pool.0)
            .await
            .unwrap();

        let a_form = forms.first();
        match a_form {
            Some(form) => FormResponse::Ok(Json(form.form.as_ref().unwrap().to_string())),
            None => FormResponse::NotFound,
        }

    }

    #[oai(path = "/seed/:user", method = "get")]
    async fn seed(&self, data_pool: Data<&PgPool>, user: Path<String>) -> PlainText<String> {
        // For each json file in /example_forms, add to db, for current user
        // insert into airport values (‘San Francisco’,’SFO’,ARRAY[23.42,-34.42, 23.34]);

        new_form(
            data_pool.0,
            1, //TODO: Use the filename for the ID
            "test user",
            "key",
            "display_name",
            json!({"one": "two"})
        ).await
        .expect("Form not inserted into db");


        PlainText(format!("forms created for user: {}", user.0))
    }
}

#[derive(poem_openapi::ApiResponse)]
enum FormResponse {
    /// Return the specified user.
    #[oai(status = 200)]
    Ok(Json<String>),
    /// Return when the specified user is not found.
    #[oai(status = 404)]
    NotFound
}

async fn new_form(
    pool: &PgPool,
    id: i64,
    username: &str,
    key: &str,
    display_name: &str,
    form: Value,
) -> Result<sqlx::postgres::PgRow, Error> {
        sqlx::query_as!(
            Form, 
            "INSERT INTO forms VALUES ($1, $2, $3, $4, $5);",
            id,
            username,
            key,
            display_name,
            form)
            .fetch_one(pool)
            .await
}

#[derive(Object)]
struct Request {
    id: i64,
    configuration: String,
}

#[derive(Debug, sqlx::Type)]
struct Form {
    id: i64,
    username: Option<String>,
    key: Option<String>,
    display_name: Option<String>,
    form: Option<Value>,
}

fn form_exists_for_user(_user: String, _key: String) -> bool {
    true
}
