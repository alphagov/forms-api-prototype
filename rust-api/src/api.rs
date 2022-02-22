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
use std::{
    fs,
    path,
};
use std::path::PathBuf;

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
        /*
        if form_exists_for_user("test user".to_string(), "test  key".to_string()) {
            //update forms where user and id with config=request.configuration
        } else {
            //update forms where user and id with config=request.configuration and display_name=id
        }
        /
        */
        Json(request.configuration.to_string())
    }

    #[oai(path = "/published/:id", method = "get")]
    async fn published(&self, data_pool: Data<&PgPool>, _id: Path<i64>) -> PlainText<String> {
        let forms: Vec<Form> = sqlx::query_as!(Form, "SELECT * FROM forms")
            .fetch_all(data_pool.0)
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

    #[oai(path = "/seed/:user", method = "get")]
    async fn seed(&self, data_pool: Data<&PgPool>, user: Path<String>) -> PlainText<String> {
        //let contents = fs::read_to_string(filename)
        //    .expect("Something went wrong reading the file");
        // For each json file in /example_forms, add to db, for current user
        // insert into airport values (‘San Francisco’,’SFO’,ARRAY[23.42,-34.42, 23.34]);


        seed_data_for_user(&user.0, data_pool.0)
            .await
            .expect("Seeding data failed for user");

        PlainText(format!("forms created for user: {}", user.0))
    }
}

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

/*
fn form_exists_for_user(_user: String, _key: String) -> bool {
    true
}
*/
