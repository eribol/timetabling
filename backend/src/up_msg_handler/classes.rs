use shared::msgs::classes::*;
use shared::DownMsg;
use shared::msgs::timetables::{TimetableUpMsgs, TimetableDownMsgs};
use sqlx::Row;

use super::auth::POSTGRES;
use super::timetables::get_ids;
use moon::tokio_stream::StreamExt;

pub async fn get_class_limitations(class_id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut row = sqlx::query(
        r#"select * from class_available where class_id = $1"#)
        .bind(&class_id)
        .fetch(&*db);
    let mut limitations = vec![];
    while let Some(class) = row.try_next().await.unwrap() {
        let c = ClassLimitation {
            class_id: class.try_get("class_id").unwrap(),
            day: class.try_get("day").unwrap(),
            hours: class.try_get("hours").unwrap(),
        };
        limitations.push(c);
    }
    DownMsg::GetClassesLimitations(limitations)
}

pub async fn update_class_limitations(class_id: i32, mut form: Vec<ClassLimitation>) -> DownMsg {
    let db = POSTGRES.write().await;
    form.sort_by(|a,b| a.day.cmp(&b.day));
    if form.len() != 7{
        let c_msg = ClassDownMsgs::UpdateClassLimitationsError("No valid form".to_string());
        let t_msg = TimetableDownMsgs::Class(c_msg);
        return DownMsg::Timetable(t_msg)
    }
    if !form.iter().enumerate().all(|l| l.1.class_id == class_id && l.0+1==l.1.day as usize){
        let c_msg = ClassDownMsgs::UpdateClassLimitationsError("No valid form".to_string());
        let t_msg = TimetableDownMsgs::Class(c_msg);
        return DownMsg::Timetable(t_msg)
    }
    let mut lims: Vec<ClassLimitation> = Vec::new();
    for l in form{
        let mut insert = sqlx::query(
            r#"insert into class_available(class_id, day, hours) values($1, $2, $3) 
                on conflict(class_id, day) where class_id = $1 and  day = $2 do update set hours = $3
                returning class_id, day, hours"#)
            .bind(&class_id)
            .bind(&l.day)
            .bind(&l.hours)
            .fetch(&*db);
        while let Some(row) = insert.try_next().await.unwrap(){
            let new_lim = ClassLimitation{
                class_id: row.try_get("class_id").unwrap(),
                day: row.try_get("day").unwrap(),
                hours: row.try_get("hours").unwrap()
            };
            lims.push(new_lim);
        }

    }
    let c_msg = ClassDownMsgs::UpdateClassLimitations(lims);
    let t_msg = TimetableDownMsgs::Class(c_msg);
    DownMsg::Timetable(t_msg)
}
