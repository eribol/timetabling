use shared::msgs::teachers::*;
use shared::DownMsg;
use shared::msgs::timetables::TimetableDownMsgs;
use sqlx::Row;

use super::auth::POSTGRES;
use moon::tokio_stream::StreamExt;

pub async fn get_teachers(id: i32) -> shared::DownMsg {
    //use moon::tokio_stream::StreamExt;
    let mut teachers_query = sqlx::query(
        r#"select id, first_name, last_name, short_name from users 
        inner join school_users on user_id = id where school_users.school_id = $1"#,
    )
    .bind(&id)
    .fetch(&*POSTGRES.write().await);
    let mut teachers = vec![];
    while let Some(teacher) = teachers_query.try_next().await.unwrap() {
        let t = Teacher {
            id: teacher.try_get("id").unwrap(),
            first_name: teacher.try_get("first_name").unwrap(),
            last_name: teacher.try_get("last_name").unwrap(),
            short_name: teacher.try_get("short_name").unwrap(),
        };
        teachers.push(t);
    }
    shared::DownMsg::GetTeachers(teachers)
}

pub async fn update_teacher_limitations(school_id: i32, group_id: i32, mut form: Vec<TeacherLimitation>) -> DownMsg {
    let db = POSTGRES.write().await;
    form.sort_by(|a,b| a.day.cmp(&b.day));
    if form.len() != 7{
        let c_msg = TeacherDownMsgs::UpdateLimitationsError("No valid form".to_string());
        let t_msg = TimetableDownMsgs::Teacher(c_msg);
        return DownMsg::Timetable(t_msg)
    }
    let user_id = form[0].user_id;
    if !form.iter().enumerate().all(|l| l.1.user_id == user_id && l.0+1==l.1.day as usize){
        let c_msg = TeacherDownMsgs::UpdateLimitationsError("No valid form".to_string());
        let t_msg = TimetableDownMsgs::Teacher(c_msg);
        return DownMsg::Timetable(t_msg)
    }
    let mut lims: Vec<TeacherLimitation> = Vec::new();
    for l in form{
        let mut insert = sqlx::query(
            r#"insert into teacher_available(user_id, school_id, group_id, day, hours) values($1, $2, $3, $4, $5) 
                on conflict(user_id, group_id, day, school_id) where user_id = $1 and  day = $4 and group_id = $3 do update set hours = $5
                returning user_id, school_id, group_id, day, hours"#)
            .bind(&user_id)
            .bind(&school_id)
            .bind(&group_id)
            .bind(&l.day)
            .bind(&l.hours)
            .fetch(&*db);
        while let Some(row) = insert.try_next().await.unwrap(){
            let new_lim = TeacherLimitation{
                user_id: row.try_get("user_id").unwrap(),
                group_id,
                school_id,
                day: row.try_get("day").unwrap(),
                hours: row.try_get("hours").unwrap()
            };
            lims.push(new_lim);
        }

    }
    let teacher_msg = TeacherDownMsgs::UpdateLimitations(lims);
    let t_msg = TimetableDownMsgs::Teacher(teacher_msg);
    DownMsg::Timetable(t_msg)
}