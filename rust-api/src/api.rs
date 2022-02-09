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
        if form_exists_for_user("test user".to_string(), "test key".to_string()) {
            //update forms where user and id with config=request.configuration
        } else {
            //update forms where user and id with config=request.configuration and display_name=id
        }
        Json(request.configuration.to_string())
    }

    #[oai(path = "/published/:id", method = "get")]
    async fn published(&self, id:Path<i64>) -> PlainText<String> {
        PlainText(format!("Form id: {}!", id.0))
    }
}

#[derive(Object)]
struct Request {
    id: i64,
    configuration: String,
}


fn form_exists_for_user(_user: String, _key: String) -> bool {
    true
}
