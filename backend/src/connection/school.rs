use moon::{tokio_stream::StreamExt, *};
use shared::*;
use sqlx::{FromRow, Row};

use super::sql::POSTGRES;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(crate = "serde")]
pub struct UpdateSchool {
    name: Option<String>,
    manager: Option<i32>,
    tel: Option<String>,
    location: Option<String>,
}

pub async fn get_school(manager: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut school =
        sqlx::query(r#"select id, name from school where manager = $1"#)
            .bind(&manager)
            .fetch(&*db);
    if let Some(school) = school.try_next().await.unwrap(){
        return DownMsg::GetSchool { id: school.try_get("id").unwrap(), name: school.try_get("name").unwrap() };
    }
    DownMsg::GetSchoolError("There is no school".to_string())
}

pub async fn _auth(auth_token: Option<AuthToken>) -> Result<i32, String> {
    match auth_token {
        Some(auth) => {
            use crate::connection::get_user;
            let user_id: redis::RedisResult<i32> = get_user(&auth.into_string()).await;
            match user_id {
                Ok(id) => Ok(id),
                Err(e) => Err(e.to_string()),
            }
        }
        None => Err("Not auth".to_string()),
    }
}
