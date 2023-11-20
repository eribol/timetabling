use crate::{*, 
    app::{
        set_page_id, 
        timetables::{
            self, 
            teachers::{teachers, selected_teacher}}, Pages
        }
    };
use shared::{msgs::{timetables::TimetableDownMsgs, activities::ActivityDownMsgs, classes::ClassDownMsgs, teachers::TeacherDownMsgs}, DownMsg, UpMsg};
use crate::app::timetables::*;
#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, cor_id| {
        //println!("DownMsg received: {:?}", down_msg);

    app::unfinished_mutations().update_mut(|cor_ids| {
        cor_ids.remove(&cor_id);
    });
    match down_msg {
        DownMsg::LoggedUser(user) => {
            get_school();
            crate::app::login::login_error().set(None);
            crate::app::login::set_and_store_logged_user(user)
        }
        DownMsg::GetSchool { id, name } => {
            use crate::app::timetables::{school, School};
            set_page_id(Pages::Home);
            school().set(Some(School { id, name }));
            app::timetables::get_timetables();
            app::timetables::teachers::get_teachers();
            get_lectures();
        }
        DownMsg::GetLectures(mut lectures) => {
            lectures.sort_by(|a,b| a.short_name.cmp(&b.short_name));
            crate::app::timetables::lectures()
            .lock_mut()
            .replace_cloned(lectures);
        },
        DownMsg::Timetable(msg) => {
            match msg {
                TimetableDownMsgs::GetClasses(mut classes)=>{
                    classes.sort_by(|a, b| a.label().cmp(&b.label())); 
                    classes::classes().lock_mut().replace_cloned(classes);
                },
                TimetableDownMsgs::GetClassesLimitations(lims)=>{
                    create_classes_limitations(lims);
                    class::limitations::loaded_lims().set(true);
                },
                TimetableDownMsgs::GetActivities(acts)=>{
                    timetables::activities().lock_mut().replace_cloned(acts);
                },
                TimetableDownMsgs::GetSchedules(schedules) => {
                    timetables::schedules().lock_mut().replace_cloned(schedules);
                }
                TimetableDownMsgs::GetTeachersLimitations(t_lims) => {
                    create_teachers_limitations(t_lims);
                    //println!("geldi lim");
                }
                TimetableDownMsgs::Class(c_msg) => {
                    match c_msg{
                        ClassDownMsgs::UpdateClassLimitations(lims)=>{
                            let id = lims.get(0).unwrap().class_id;
                            classes_limitations().lock_mut().insert(id, lims);
                        }
                        _ => {}
                    }
                }
                TimetableDownMsgs::Teacher(t_msg) => {
                    match t_msg{
                        TeacherDownMsgs::UpdateLimitations(lims)=>{
                            if let Some(id) = selected_teacher().get(){
                                teachers_limitations().lock_mut().insert(id, lims);
                            }
                        }
                        _ => {}
                    }
                }
                _ => (),
            };
        },
        DownMsg::Activity(a_msg) =>{
            match a_msg{
                ActivityDownMsgs::AddedAct(act) => {
                    activities().lock_mut().push_cloned(act.clone());
                    let teachers = act.teachers;
                    let classes = act.classes;
                    let hour = act.hour as u32;
                    for t in &teachers{
                        let tot = total_act_hours_for_teacher().lock_mut();
                        let tot = tot.get(t);
                        total_act_hours_for_teacher().lock_mut().insert_cloned(t.clone(), tot.unwrap()+hour);  
                    }
                     for c in &classes{
                        let c_tot = total_act_hours_for_classes().lock_mut();
                        let c_tot = c_tot.get(c);
                        total_act_hours_for_teacher().lock_mut().insert_cloned(c.clone(), c_tot.unwrap()+hour);  
                    }
                },
                ActivityDownMsgs::DeletedAct(act_id) => {
                    let mut acts = activities().lock_mut().to_vec();
                    let act = acts.iter().enumerate().find(|a| a.1.id == act_id).unwrap();
                    let teacher = act.1.teachers[0];
                    let hour = act.1.hour as u32;
                    acts.remove(act.0);
                    activities().lock_mut().replace_cloned(acts);
                    let mut tots = total_act_hours_for_teacher().lock_mut();
                    let mut tots2 = tots.clone();
                    let tot = tots2.get_mut(&teacher).unwrap();
                    tots.insert(teacher, *tot-hour);
                },
                _ => ()
            } 
        },
        DownMsg::GetTimetables(ts)=>{
            crate::app::timetables::timetables()
            .lock_mut()
            .replace_cloned(ts.clone());
            if let Some(group) = ts.get(0) {
                set_page_id(app::Pages::Home);
                crate::app::timetables::change_timetable(group.id.to_string());
                //app::timetables::classes::get_classes();
                get_classes_limitations();
                get_teachers_limitations();
                get_schedules();
                get_activities();
            }
        }
        DownMsg::GetTeachers(mut tchrs)=>{
            tchrs.sort_by(|a, b| a.label_full().cmp(&b.label_full()));
            teachers().lock_mut().replace_cloned(tchrs);
        }
        _ => (),        
        }
    })
    .auth_token_getter(app::auth_token)
}

pub fn send_msg(msg: UpMsg) {
    Task::start(async {
        match connection().send_up_msg(msg).await {
            Err(_error) => {}
            Ok(_msg) => (),
        }
    });
}
