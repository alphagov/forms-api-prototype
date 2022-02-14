use poem::web::Data;
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    Object, OpenApi,
};
use serde_json::Value;
use sqlx::postgres::PgPool;

/// # API design
/// post "/publish"
/// def form_exists_for_user?(user, key)
/// get "/published"
/// get "/published/:id"
/// def authenticated_user
/// def forms_for_user(user)
///
/// def seed_data_for_user(user)
/// get "/seed/:user" (optional, designer expects forms to exist)

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
    async fn published(&self, pool: Data<&PgPool>, id: Path<i64>) -> PlainText<String> {
        let forms = sqlx::query_as!(Forms, "SELECT * FROM forms")
            .fetch_all(pool.0)
            .await
            .unwrap();

        let username = forms
            .first()
            .expect("No forms in DB")
            .username
            .as_ref()
            .unwrap();
        PlainText(format!("Form id: {}!", username))
    }
}

#[derive(Object)]
struct Request {
    id: i64,
    configuration: String,
}

#[derive(Debug, sqlx::Type)]
struct Forms {
    id: i64,
    username: Option<String>,
    key: Option<String>,
    display_name: Option<String>,
    form: Option<Value>,
}

fn form_exists_for_user(_user: String, _key: String) -> bool {
    true
}
