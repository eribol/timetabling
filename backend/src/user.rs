use crate::connection;
use bcrypt::{hash, verify};
use chrono;
use moon::*;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(crate = "serde")]
pub struct LoginUser {
    pub id: i32,
    pub first_name: String,
    password: String,
}

pub async fn login(email: String, password: String) -> sqlx::Result<LoginUser> {
    let db = connection::sql::POSTGRES.read().await;
    let user: sqlx::Result<LoginUser> =
        sqlx::query_as(r#"select id, first_name, password from users where email = $1"#)
            .bind(&email)
            //.bind(verify(password, ))
            .fetch_one(&*db)
            .await;
    match user {
        Ok(u) => {
            if verify(&password, &u.password).unwrap() {
                println!("bulundu");
                Ok(u)
            } else {
                println!("bulunmadı");
                Err(sqlx::Error::RowNotFound)
            }
        }
        Err(_e) => Err(sqlx::Error::RowNotFound),
    }
    //user
}
