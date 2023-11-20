use crate::connection::sql::POSTGRES;
use moon::tokio_stream::StreamExt;
use shared::msgs::activities::{ActivityDownMsgs, AddActivity, FullActivity};
use shared::msgs::classes::ClassUpMsgs;
use shared::msgs::teachers::TeacherUpMsgs;
use shared::msgs::timetables::*;
use shared::DownMsg;
use sqlx::Row;

use super::classes::update_class_limitations;
use super::school::is_teachers_valid;
use super::teachers::update_teacher_limitations;

pub async fn timetable_msg(msg: TimetableUpMsgs, school_id: i32)->DownMsg{
    match msg{
        TimetableUpMsgs::GetClasses(group_id)=> get_classes(group_id).await,
        TimetableUpMsgs::GetActivities(group_id) => get_activities(school_id, group_id).await,
        TimetableUpMsgs::GetClassesLimitations(group_id) => get_classes_limitations(group_id).await,
        TimetableUpMsgs::GetTeachersLimitations(group_id) => get_teachers_limitations(group_id).await,
        TimetableUpMsgs::Class(c_msg) => {
            match c_msg{
                ClassUpMsgs::UpdateLimitations((group_id, form)) => update_class_limitations(group_id, form).await,
            }
        }
        TimetableUpMsgs::Teacher(t_msg) => {
            match t_msg{
                TeacherUpMsgs::UpdateLimitations((group_id, form)) => update_teacher_limitations(school_id, group_id, form).await,
            }
        }
        TimetableUpMsgs::GetSchedules(group_id) => get_schedules(school_id, group_id).await,
        TimetableUpMsgs::DelSchedules(acts) => del_schedules(acts).await,
        TimetableUpMsgs::UpdateSchedules(schedules) => update_schedules(schedules).await
    }
}
pub async fn get_classes(id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut row = sqlx::query(
        r#"select id, kademe, sube, group_id from classes
                        where group_id = $1"#,
    )
    .bind(&id)
    .fetch(&*db);
    let mut classes = vec![];
    while let Some(class) = row.try_next().await.unwrap() {
        let c = shared::msgs::classes::Class {
            id: class.try_get("id").unwrap(),
            sube: class.try_get("sube").unwrap(),
            kademe: class.try_get("kademe").unwrap(),
            group_id: class.try_get("group_id").unwrap(),
        };
        classes.push(c);
    }
    DownMsg::Timetable(TimetableDownMsgs::GetClasses(classes))
}
pub async fn get_classes_limitations(group_id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut row = sqlx::query(r#"select * from class_available inner join classes on class_available.class_id = classes.id
                        where classes.group_id = $1"#)
        .bind(&group_id)
        .fetch(&*db);
    let mut limitations = vec![];
    while let Some(class) = row.try_next().await.unwrap() {
        let c = shared::msgs::classes::ClassLimitation {
            class_id: class.try_get("class_id").unwrap(),
            day: class.try_get("day").unwrap(),
            hours: class.try_get("hours").unwrap(),
        };
        limitations.push(c);
    }
    println!("classes_limitations");
    let t_msg = TimetableDownMsgs::GetClassesLimitations(limitations);
    DownMsg::Timetable(t_msg)
}
pub async fn get_teachers_limitations(group_id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut row = sqlx::query(r#"select * from teacher_available
                        where group_id = $1"#)
        .bind(&group_id)
        .fetch(&*db);
    let mut limitations = vec![];
    while let Some(class) = row.try_next().await.unwrap() {
        let c = shared::msgs::teachers::TeacherLimitation{
            user_id: class.try_get("user_id").unwrap(),
            school_id: class.try_get("school_id").unwrap(),
            group_id: class.try_get("group_id").unwrap(),
            day: class.try_get("day").unwrap(),
            hours: class.try_get("hours").unwrap(),
        };
        limitations.push(c);
    }
    println!("classes_limitations");
    let t_msg = TimetableDownMsgs::GetTeachersLimitations(limitations);
    DownMsg::Timetable(t_msg)
}
pub async fn get_class_groups(school_id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let mut groups_query = sqlx::query(
        r#"select class_groups.id, class_groups.name, class_groups.hour from class_groups
        inner join school on school.id = class_groups.school where school.id = $1"#,
    )
        .bind(&school_id)
        .fetch(&*db);
    let mut groups = vec![];
    while let Some(g) = groups_query.try_next().await.unwrap() {
        let group = Timetable {
            id: g.try_get("id").unwrap(),
            name: g.try_get("name").unwrap(),
            hour: g.try_get("hour").unwrap(),
        };
        groups.push(group)
    }
    DownMsg::GetTimetables(groups)
}

pub async fn get_activities(school_id: i32, group_id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    println!("school:{school_id}, group_id:{group_id}");
    let ids = get_ids(school_id, group_id).await.unwrap();
    let mut groups_query = 
        sqlx::query(r#"select activities.id, activities.subject, activities.hour, activities.teachers, activities.partner_activity, activities.classes
                        from activities where classes && $1::integer[]"#)

        .bind(&ids)
        .fetch(&*db);
    let mut activities = vec![];
    while let Some(g) = groups_query.try_next().await.unwrap() {
        let act = shared::msgs::activities::FullActivity {
            id: g.try_get("id").unwrap(),
            subject: g.try_get("subject").unwrap(),
            hour: g.try_get("hour").unwrap(),
            classes: g.try_get("classes").unwrap(),
            teachers: g.try_get("teachers").unwrap(),
            blocks: None,
            partner_activity: None
        };
        activities.push(act)
    }
    DownMsg::Timetable(TimetableDownMsgs::GetActivities(activities))
}

pub async fn get_schedules(school_id: i32, group_id: i32) -> DownMsg {
    let db = POSTGRES.read().await;
    let ids = get_activities(school_id, group_id).await;
    let mut schedules = vec![];
    if let DownMsg::Timetable(TimetableDownMsgs::GetActivities(acts)) = ids{
        let acts = acts.iter().map(|a| a.id).collect::<Vec<i32>>();
        let mut groups_query = sqlx::query(r#"select day_id, hour, locked, activity
                from class_timetable where activity = any($1)"#)
            .bind(&acts)
            .fetch(&*db);
        while let Some(g) = groups_query.try_next().await.unwrap() {
            let act = shared::msgs::timetables::Schedule {
                day_id: g.try_get("day_id").unwrap(),
                hour: g.try_get("hour").unwrap(),
                activity: g.try_get("activity").unwrap(),
                locked: g.try_get("locked").unwrap()
            };
            schedules.push(act)
        }    
    }
    DownMsg::Timetable(TimetableDownMsgs::GetSchedules(schedules))
}

pub async fn update_schedules(schedules: Vec<Schedule>) -> DownMsg {
    let db = POSTGRES.read().await;
    let acts: Vec<i32> = schedules.iter().map(|s| s.activity).collect();
    let _del_schedules = sqlx::query(r#"delete from class_timetable where activity = any($1)"#)
        .bind(&acts)
        .execute(&*db).await;
    for s in &schedules{
        let mut _groups_query = 
        sqlx::query(r#"insert into class_timetable(activity, day_id, hour, locked) values($1, $2, $3, $4)"#)
        .bind(&s.activity)
        .bind(&s.day_id)
        .bind(&s.hour)
        .bind(&s.locked)
        .execute(&*db).await;
    }
    DownMsg::Timetable(TimetableDownMsgs::GetSchedules(schedules))
}

pub async fn del_schedules(acts: Vec<i32>) -> DownMsg {
    let db = POSTGRES.read().await;
    let _del_schedules = sqlx::query(r#"delete from class_timetable where activity = any($1)"#)
        .bind(&acts)
        .execute(&*db).await.unwrap();
    println!("del sch");
    DownMsg::Timetable(TimetableDownMsgs::DelSchedules)
}

pub async fn add_activity(school_id: i32, group_id: i32, form: AddActivity) -> DownMsg {
    let db = POSTGRES.read().await;
    let ids = get_ids(school_id, group_id).await.unwrap();
    if !ids.iter().any(|i| form.classes.iter().any(|c| c == i) ){
        return DownMsg::Activity(ActivityDownMsgs::AddActError("No classes".to_string()))
    }
    if !is_teachers_valid(school_id, &form.teachers).await{
        return DownMsg::Activity(ActivityDownMsgs::AddActError("Not valid teachers".to_string()))
    }
    let mut groups_query = 
        sqlx::query(r#"insert into activities(subject, teachers, classes, hour) values ($1, $2, $3, $4)
                    returning id, subject, teachers, classes, hour"#)

        .bind(&form.subject)
        .bind(&form.teachers)
        .bind(&form.classes)
        .bind(&form.hour)
        .fetch(&*db);
    if let Some(act) = groups_query.try_next().await.unwrap() {
        let act_id = act.try_get("id").unwrap();
        let _ = sqlx::query(r#"insert into school_acts(school_id, group_id, act_id) values ($1, $2, $3)"#)
        .bind(&school_id)
        .bind(&group_id)
        .bind(&act_id)
        .execute(&*db).await;
        let act = shared::msgs::activities::FullActivity {
            id: act_id,
            subject: act.try_get("subject").unwrap(),
            hour: act.try_get("hour").unwrap(),
            classes: act.try_get("classes").unwrap(),
            teachers: act.try_get("teachers").unwrap(),
            blocks: None,
            partner_activity: None
        };
        return DownMsg::Activity(ActivityDownMsgs::AddedAct(act))
    }
    DownMsg::Activity(ActivityDownMsgs::AddActError("Database error".to_string()))
}

pub async fn del_act(school_id: i32, group_id: i32, form: FullActivity) -> DownMsg {
    let db = POSTGRES.read().await;
    let ids = get_ids(school_id, group_id).await.unwrap();
    if !ids.iter().any(|i| form.classes.iter().any(|c| c == i) ){
        return DownMsg::Activity(ActivityDownMsgs::AddActError("No classes".to_string()))
    }
    let mut groups_query = 
        sqlx::query(r#"delete from activities where id = $1
                    returning id"#)

        .bind(&form.id)
        .fetch(&*db);
    if let Some(_) = groups_query.try_next().await.unwrap() {
        return DownMsg::Activity(ActivityDownMsgs::DeletedAct(form.id))
    }
    DownMsg::Activity(ActivityDownMsgs::DeleteActError("Database error for delete act".to_string()))
}

pub async fn get_ids(school_id: i32, group_id: i32)-> Option<Vec<i32>>{
    let ids: sqlx::Result<(Option<Vec<i32>>, )> = sqlx::query_as(r#"select array_agg(id) from  classes where school= $1 and group_id = $2"#)
            .bind(&school_id)
            .bind(group_id)
            .fetch_one(&*POSTGRES.read().await).await;
    match ids{
        Ok(id)=> {
            //println!("{:?}", id.clone());
            id.0
        },
        Err(e) => {
            return None
        }
    }
}

