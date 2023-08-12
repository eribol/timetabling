use moon::*;
use shared::msgs::lectures::*;
use shared::DownMsg;
use sqlx::Row;

use super::auth::POSTGRES;
use moon::tokio_stream::StreamExt;

pub async fn get_lectures(id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut lectures: Vec<Lecture> = vec![];
    let mut row = sqlx::query(
        r#"select id, kademe, name, short_name from subjects
        where school = $1"#,
    )
        .bind(&id)
        .fetch(&*db);
    while let Some(lec) = row.try_next().await.unwrap() {
        lectures.push(Lecture {
            id: lec.try_get("id").unwrap(),
            kademe: lec.try_get("kademe").unwrap(),
            name: lec.try_get("name").unwrap(),
            short_name: lec.try_get("short_name").unwrap(),
        })
    }
    DownMsg::GetLectures(lectures)
}
