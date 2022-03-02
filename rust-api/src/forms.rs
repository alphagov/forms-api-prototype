use poem_openapi::Object;
use serde_json::{Value};
use sqlx::{postgres::PgPool, Error};

#[derive(Object, Debug, sqlx::Type)]
pub struct Form {
    pub id: i64,
    pub username: Option<String>,
    pub key: Option<String>,
    pub display_name: Option<String>,
    pub form: Option<Value>,
}

impl Form {
    pub async fn new_form(
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

    pub async fn form_exists_for_user(user: &str, key: &String, pool: &PgPool) -> bool {
        sqlx::query_as!(
            Form,
            "SELECT * FROM forms WHERE username=$1 AND key=$2;",
            user,
            key
        ).fetch_all(pool)
            .await
            .unwrap()
            .first()
            .is_some()
    }

    pub async fn forms_for_user(user: &str, pool: &PgPool) -> Vec<Form> {
        sqlx::query_as!(
            Form,
            "SELECT * FROM forms WHERE username=$1;",
            user,
        ).fetch_all(pool)
            .await
            .unwrap()
    }
}
