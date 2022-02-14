use poem_openapi::{
    param::{
        Path
    },
    payload::{
        Json,
        PlainText
    },
    OpenApi,
    Object
};
use poem::{
    web::Data
};
use sqlx::postgres::PgPoolOptions;

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
    async fn published(&self, pool: Data<&PgPoolOptions>, id:Path<i64>) -> PlainText<String> {

    let countries = sqlx::query_as!(Country,
            "
            SELECT country, COUNT(*) as count
            FROM users
            GROUP BY country
            WHERE organization = ?
            ",
            "test"
        )
        .fetch_all(&pool) // -> Vec<Country>
        .await;

        PlainText(format!("Form id: {}!", id.0))
    }


}

#[derive(Object)]
struct Request {
    id: i64,
    configuration: String,
}

struct Country { country: String, count: i64 }

fn form_exists_for_user(_user: String, _key: String) -> bool {
    true
}
