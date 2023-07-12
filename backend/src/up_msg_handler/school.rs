use super::{auth::POSTGRES, teachers::get_teachers};
use moon::tokio_stream::StreamExt;
use shared::DownMsg;
use sqlx::Row;

pub async fn get_school(manager: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut query = sqlx::query(r#"select id, name from school where manager = $1"#)
        .bind(&manager)
        .fetch(&*db);
    if let Ok(row) = query.try_next().await {
        if let Some(row2) = row {
            return DownMsg::GetSchool {
                id: row2.try_get("id").unwrap(),
                name: row2.try_get("name").unwrap(),
            };
        }
    }
    DownMsg::AuthError("Not auth for school".to_string())
}

pub async fn is_teachers_valid(school_id: i32, teachers: &Vec<i32>) -> bool {
    if let DownMsg::GetTeachers(school_teachers) = get_teachers(school_id).await{
        return teachers.iter().all(|a| school_teachers.iter().any(|st| st.id == *a))
    }
    false
}
