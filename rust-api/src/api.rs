use poem::web::Data;
use poem_openapi::{param::Path, payload::{Json, PlainText}, Object, OpenApi};
use serde_json::{Value, json};
use sqlx::{postgres::PgPool, Error};
use std::{fs, path};
use std::path::PathBuf;

/// # API design
/// - [x] post "/publish"
/// - [x] def form_exists_for_user?(user, key)
/// - [ ] get "/published"
/// - [x] get "/published/:id"
/// - [ ] def authenticated_user
/// - [ ] def forms_for_user(user)
/// - [x] def seed_data_for_user(user)
/// - [x] get "/seed/:user" (optional, designer expects forms to exist)

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/publish", method = "post")]
    async fn publish(&self, data_pool: Data<&PgPool>, request: Json<Request>) -> Json<String> {
        let user = "test user".to_string();
        let key = "test key".to_string();
        if form_exists_for_user(&user, &key, data_pool.0).await {
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
            new_form(
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

    #[oai(path = "/published/:id", method = "get")]
    async fn published(&self, data_pool: Data<&PgPool>, id: Path<i64>) -> FormResponse {
        let forms: Vec<Form> = sqlx::query_as!(Form, "SELECT * FROM forms WHERE id=$1;", id.0)
            .fetch_all(data_pool.0)
            .await
            .unwrap();

        let a_form = forms.first();
        return match a_form {
            Some(form) => FormResponse::Ok(Json(form.form.as_ref().unwrap().to_string())),
            None => FormResponse::NotFound,
        }

    }

    #[oai(path = "/seed/:user", method = "get")]
    async fn seed(&self, data_pool: Data<&PgPool>, user: Path<String>) -> PlainText<String> {
        seed_data_for_user(&user.0, data_pool.0)
            .await
            .expect("Seeding data failed for user");

        return PlainText(format!("forms created for user: {}", user.0))
    }
}

#[derive(poem_openapi::ApiResponse)]
enum FormResponse {
    /// Return the specified form.
    #[oai(status = 200)]
    Ok(Json<String>),
    /// Return when the specified form is not found.
    #[oai(status = 404)]
    NotFound
}

/// For each json file in /example_forms, add to db, for current user
/// insert into airport values (‘San Francisco’,’SFO’,ARRAY[23.42,-34.42, 23.34]);
async fn seed_data_for_user(user: &str, pool: &PgPool) -> Result<(), Error> {
    let paths = fs::read_dir(&path::Path::new("./example_forms")).unwrap();

    let file_names = paths.filter_map(|entry| {
        entry.ok().and_then(|e| {
            let file_name = e.path()
                .file_name()
                .and_then(|n| {
                    n.to_str()
                    .map(String::from)
                });
            file_name
        })
    }).collect::<Vec<String>>();

    for form_file in file_names {
        let path: PathBuf = ["./example_forms", &form_file].iter().collect();
        let file_content = fs::read_to_string(path).unwrap();

        new_form(
            user,
            &form_file,
            &form_file,
            json!(file_content),
            pool,
        ).await?;
    }
    Ok(())
}

async fn new_form(
    username: &str,
    key: &str,
    display_name: &str,
    form: Value,
    pool: &PgPool,
) -> Result<Form, Error> {
        sqlx::query_as!(
            Form, 
            "INSERT INTO forms
             VALUES ($1, $2, $3, $4, $5)
             RETURNING *;",
            rand::random::<i64>(),
            username,
            key,
            display_name,
            form)
            .fetch_one(pool)
            .await
}

#[derive(Object)]
struct Request {
    id: String,
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


async fn form_exists_for_user(user: &String, key: &String, pool: &PgPool) -> bool {

    let forms: Vec<Form> = sqlx::query_as!(Form,
        "SELECT * FROM forms WHERE username=$1 AND key=$2;",
        user, key
    ).fetch_all(pool)
        .await
        .unwrap();

    forms.first().is_some()
}
