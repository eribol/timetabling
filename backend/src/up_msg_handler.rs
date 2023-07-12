use crate::{
    connection::{
        get_user, school::get_school
    },
    user::{self}, up_msg_handler::{timetables::{timetable_msg, get_class_groups}, teachers::get_teachers},
};
use moon::*;
use shared::{
    msgs::{
        classes::{ClassUpMsgs},
        lectures::{LecturesUpMsg, LecturesDownMsg}, timetables::TimetableUpMsgs, activities::ActivityUpMsgs, users::User,
    },
    *,
};
pub mod auth;
pub mod classes;
pub mod lectures;
pub mod school;
pub mod teachers;
pub mod timetables;

pub async fn up_msg_handler(req: UpMsgRequest<UpMsg>) {
    println!("{req:?}");
    let UpMsgRequest {
        up_msg,
        cor_id,
        session_id,
        auth_token,
    } = req;

    let down_msg = match up_msg {
        UpMsg::Login { email, password } => {
            let user = user::login(email, password);
            match user.await {
                Ok(u) => {
                    let auth_token = AuthToken::new(format!("{}:{}", u.id, EntityId::new()));
                    //println!("{:?}", &auth_token.clone().into_string());
                    crate::connection::set_user(u.id, &auth_token)
                        .await
                        .expect("Not set user");
                    let user2 = User {
                        id: u.id,
                        first_name: u.first_name,
                        auth_token,
                    };

                    DownMsg::LoggedUser(user2)
                }
                Err(_e) => DownMsg::AuthError("Sorry, invalid name or password.".to_owned()),
            }
        }
        UpMsg::Logout => {
            let auth = auth_token.unwrap().into_string();
            let u: Result<i32, redis::RedisError> = crate::connection::get_user(&auth).await;
            match u {
                Ok(id) => match crate::connection::del_user(id, auth).await {
                    Ok(_) => DownMsg::LoggedOut,
                    Err(e) => DownMsg::LoggedOutError(e.to_string()),
                },
                Err(e) => DownMsg::LoggedOutError(e.to_string()),
            }
        }
        UpMsg::GetSchool => {
            if let Some(auth_token) = auth_token {
                let manager = get_user(&auth_token.into_string()).await;
                get_school(manager.unwrap()).await 
            } else {
                DownMsg::AuthError("Not auth".to_string())
            }
        }
        UpMsg::GetTeachers => {
            if let Some(auth_token) = auth_token {
                let manager = get_user(&auth_token.into_string()).await;
                let school_msg = crate::up_msg_handler::school::get_school(manager.unwrap()).await;
                if let DownMsg::GetSchool { id, .. } = school_msg {
                    get_teachers(id).await
                } else {
                    school_msg
                } 
            } else {
                DownMsg::AuthError("Not auth".to_string())
            }
        }
        UpMsg::Timetable(tt_msg) => {
            if let Some(auth) = auth_token {
                let manager = get_user(&auth.into_string()).await;
                let school_msg = crate::up_msg_handler::school::get_school(manager.unwrap()).await;
                if let DownMsg::GetSchool { id, .. } = school_msg {
                    timetable_msg(tt_msg, id).await
                } else {
                    school_msg
                }
            } 
            else {
                DownMsg::AuthError("Not Auth".to_string())
            }
        }
        UpMsg::Activity(act_msg) => {
            if let Some(auth) = auth_token {
                let manager = get_user(&auth.into_string()).await;
                let manager = manager.unwrap();
                let school_msg = crate::up_msg_handler::school::get_school(manager).await;
                if let DownMsg::GetSchool { id, .. } = school_msg {
                    match act_msg {
                        //ActivitiesUpMsgs::GetActivities(group_id) => timetables::get_activities(id, group_id).await,
                        ActivityUpMsgs::AddAct(act) => timetables::add_activity(id, act.0, act.1).await,
                        ActivityUpMsgs::DelAct((group_id, form)) => timetables::del_act(id, group_id, form).await
                    }
                } else {
                    school_msg
                }
            } 
            else {
                DownMsg::AuthError("Not Auth".to_string())
            }
        }
        UpMsg::GetLectures => {
            if let Some(auth) = auth_token {
                let manager = get_user(&auth.into_string()).await;
                let school_msg = crate::up_msg_handler::school::get_school(manager.unwrap()).await;
                if let DownMsg::GetSchool { id, ..} = school_msg{
                    lectures::get_lectures(id).await
                }
                else{
                    school_msg
                }
            } 
            else {
                DownMsg::AuthError("Not Auth".to_string())
            }
        }
        UpMsg::GetTimetables => {
            if let Some(auth) = auth_token {
                let manager = get_user(&auth.into_string()).await;
                let school_msg = crate::up_msg_handler::school::get_school(manager.unwrap()).await;
                if let DownMsg::GetSchool { id, .. } = school_msg{
                    get_class_groups(id).await
                }
                else{
                    school_msg
                }
            } 
            else {
                DownMsg::AuthError("Not Auth".to_string())
            }
        }
    };
    if let Some(session) = sessions::by_session_id().wait_for(session_id).await {
        session.send_down_msg(&down_msg, cor_id).await;
    } else {
        eprintln!("cannot find the session with id `{}`", session_id);
    }
}
