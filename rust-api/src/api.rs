use poem::{
    Request,
    web::Data
};
use poem_openapi::{
    param::Path,
    payload::{Json, PlainText},
    Object,
    OpenApi,
    SecurityScheme,
    auth::ApiKey,
};
use serde_json::{json};
use sqlx::{postgres::PgPool, Error};
use std::{fs, path};
use std::path::PathBuf;
use jwt::{VerifyWithKey};
use serde::{Deserialize, Serialize};
use hmac::{Hmac};
use sha2::Sha256;

use crate::forms;
use forms::Form;

type ServerKey = Hmac<Sha256>;
pub struct Api;

#[OpenApi]
impl Api {

    /// Publish a form
    #[oai(path = "/publish", method = "post")]
    async fn publish(&self, data_pool: Data<&PgPool>, request: Json<OurRequest>) -> Json<String> {
        let user = "test user".to_string();
        let key = "test key".to_string();
        if Form::form_exists_for_user(&user, &key, data_pool.0).await {
            sqlx::query!(
                "UPDATE forms
                 SET form = $1
                 WHERE username = $2
                 AND   key = $3;",
                json!(request.configuration),
                user,
                key
            ).fetch_one(data_pool.0)
                .await.unwrap();
        } else {
            Form::new_form(
                &user,
                &request.id,
                &request.id,
                json!(request.configuration),
                data_pool.0,
            ).await
             .expect("new form insert failed");
        }
        Json(request.configuration.to_string())
    }

    /// Get form by its ID
    #[oai(path = "/published/:id", method = "get")]
    async fn published_by_id(&self, data_pool: Data<&PgPool>, id: Path<i64>) -> FormResponse {
        let forms: Vec<Form> = sqlx::query_as!(Form, "SELECT * FROM forms WHERE id=$1;", id.0)
            .fetch_all(data_pool.0)
            .await
            .unwrap();

        let a_form = forms.first();
        return match a_form {
            Some(form) => {
                let published_form = PublishedForm {
                    Key: form.key.as_ref().unwrap().to_string(),
                    DisplayName: form.display_name.as_ref().unwrap().to_string(),
                    FeedbackForm: false
                };
                FormResponse::Ok(Json(published_form))
            }
            None => FormResponse::NotFound,
        }

    }

    /// Create default forms for the user
    #[oai(path = "/seed/:user", method = "get")]
    async fn seed(&self, data_pool: Data<&PgPool>, user: Path<String>) -> PlainText<String> {
        seed_data_for_user(&user.0, data_pool.0)
            .await
            .expect("Seeding data failed for user");

        return PlainText(format!("forms created for user: {}", user.0))
    }

    /// Get all forms for the user
    #[oai(path = "/published", method = "get")]
    async fn published(&self, data_pool: Data<&PgPool>) -> Json<Vec<PublishedForm>> {
        //auth: MyApiKeyAuthorization,
        let user = "jwt"; //auth.0.username;
        let forms = Form::forms_for_user(&user, data_pool.0).await;
        let published_forms = forms
            .iter()
            .map(|form| {
                PublishedForm {
                    Key: form.key.as_ref().unwrap().to_string(),
                    DisplayName: form.display_name.as_ref().unwrap().to_string(),
                    FeedbackForm: false
                }})
                .collect();
        return Json(published_forms)
}

/*
  def authenticated_user
    token = request.env['HTTP_X_API_KEY']
    begin
      decoded_token = JWT.decode token, nil, false
      return decoded_token[0]["user"]
    rescue
      return nil
    end
  end
 */


}



#[derive(Debug, Serialize, Deserialize)]
struct User {
    username: String,
}

/// ApiKey authorization
#[derive(SecurityScheme)]
#[oai(
    type = "api_key",
    key_name = "X-API-KEY",
    in = "header",
    checker = "api_checker"
)]
struct MyApiKeyAuthorization(User);



async fn api_checker(req: &Request, api_key: ApiKey) -> Option<User> {
    let server_key = req.data::<ServerKey>().unwrap();
    VerifyWithKey::<User>::verify_with_key(api_key.key.as_str(), server_key).ok()
}


/// Form objects. The x-gov builder expects pascal case
#[allow(non_snake_case)]
#[derive(Object)]
struct PublishedForm {
    Key: String,
    DisplayName: String,
    FeedbackForm: bool
}

#[derive(poem_openapi::ApiResponse)]
enum FormResponse {
    /// Return the specified form.
    #[oai(status = 200)]
    Ok(Json<PublishedForm>),
    /// Return when the specified form is not found.
    #[oai(status = 404)]
    NotFound
}


/// Returns a vec of the filenames inside `folder`
async fn get_files_in_folder(folder: &str) -> Vec<String> {
    let paths = fs::read_dir(&path::Path::new(folder)).unwrap();

    paths.filter_map(|entry_result| {
        entry_result.ok().and_then(|entry| {
            entry
                .path()
                .file_name()
                .and_then(|name| {
                    name.to_str()
                    .map(String::from)
                })
        })
    }).collect()
}

/// For each json file in /example_forms, add to db, for current user
async fn seed_data_for_user(user: &str, pool: &PgPool) -> Result<(), Error> {

    let file_names = get_files_in_folder("./example_forms").await; 
    for form_file in file_names {
        let path: PathBuf = ["./example_forms", &form_file].iter().collect();
        let file_content = fs::read_to_string(path).unwrap();

        Form::new_form(
            user,
            &form_file,
            &form_file,
            json!(file_content),
            pool,
        ).await?;
    }
    Ok(())
}


#[derive(Object)]
struct OurRequest {
    id: String,
    configuration: String,
}

