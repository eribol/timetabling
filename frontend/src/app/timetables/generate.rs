use crate::app::timetables::*;
use shared::msgs::activities::Activity;
use zoon::static_ref;

#[static_ref]
fn acts()->&'static Vec<Activity>{
    let mut acts: Vec<Activity> =vec![]; 
    for a in activities().lock_mut().to_vec(){
        acts.push(Activity{
            subject: a.subject,
            id: a.id,
            classes: a.classes,
            teachers: a.teachers,
            hour: a.hour
        })
    }
    acts
}