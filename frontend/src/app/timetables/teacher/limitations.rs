use shared::UpMsg;
use shared::msgs::teachers::TeacherUpMsgs;
use shared::msgs::timetables::TimetableUpMsgs;
use zoon::*;
use zoon::named_color::*;
use crate::app::timetables::teachers::selected_teacher;
use crate::app::timetables::{selected_timetable_hour, teachers_limitations, create_default_lim, selected_timetable};
use crate::connection::send_msg;
use crate::elements::buttons;

use shared::msgs::{timetables::Schedule, teachers::TeacherLimitation};


#[static_ref]
pub fn teacher_limitations() -> &'static MutableVec<TeacherLimitation> {
    MutableVec::new_with_values(vec!())
}

pub fn get_t_l(){
    if let Some(id) = selected_teacher().get(){
        let lim = teachers_limitations().get_cloned();
        let lim = lim.get(&id);
        if let Some(lims) = lim{
            teacher_limitations().lock_mut().replace_cloned(lims.clone())
        }
        else{
            let _l = create_default_lim(id);
            //teacher_limitations().lock_mut().replace_cloned(l)
        }
    }
    
}

pub fn schedule_table()->impl Element{
    Column::new()
    .s(Gap::new().y(10))
    .s(Padding::new().left(25))
    .item(table())
    .item(bottom_buttons())
}

fn table()->impl Element{
    Row::new()
    .item(hours())
    .items_signal_vec(
        teacher_limitations().signal_vec_cloned().map(|lims| {
            let day = lims.day;
            let hours = lims.hours;
            Column::new().item(
                Button::new()
                .s(
                    Borders::new()
                    .top(Border::new().width(1).color(BLUE_3))
                    .right(Border::new().width(1).color(BLUE_3))
                    .bottom(Border::new().width(1).color(BLUE_3))
                )
                .s(Width::exact(120))
                .s(Height::exact(50))
                .label(lims.day)
                .on_click(move || all_days( day as usize-1))
            ).items(hours.iter().enumerate().map(|h|
                Button::new()
                .s(
                    Borders::new()
                    .right(Border::new().width(1).color(BLUE_3))
                    .bottom(Border::new().width(1).color(BLUE_3))
                )
                .s(Width::exact(120))
                .s(Height::exact(50))
                .on_double_click(move || day_hour(day as usize-1, h.0 as usize))
                .s(
                    Background::new()
                    .color(
                        if *h.1{
                            BLUE_1
                        }
                        else{
                            GRAY_2
                        }
                    ) 
                )
                .label("")
            ))
        })
    )
}

fn hours()->impl Element{
    Column::new()
    .item(
        Button::new()
        .s(
            Borders::new().top(Border::new().width(1).color(BLUE_3))
            .left(Border::new().width(1).color(BLUE_3))
            .right(Border::new().width(1).color(BLUE_3))
            .bottom(Border::new().width(1).color(BLUE_3))
        )
        .s(Width::exact(120))
        .s(Height::exact(50))
        .label("Günler/Saatler")
    )
    .items(
        (0..selected_timetable_hour().lock_mut().len()).map(|h|{
            Button::new()
            .s(
                Borders::new()
                .left(Border::new().width(1).color(BLUE_3))
                .right(Border::new().width(1).color(BLUE_3))
                .bottom(Border::new().width(1).color(BLUE_3))
            )
            .s(Width::exact(120))
            .s(Height::exact(50))
            .label(h+1)
            .on_click(move|| all_hours(h))
        })
    )
}
fn bottom_buttons()->impl Element{
    Row::new()
    .s(Gap::new().x(10))
    .item(
        buttons::default_with_signal(crate::i18n::t!("save-changes"))
        .on_click(save_changes)
    )
}
fn all_days(day_id: usize){
    let mut lims = teacher_limitations().lock_mut().to_vec();
    if let Some(day) = lims.get_mut(day_id){
        if day.hours.iter().any(|h| !*h){
            let l = vec![true; selected_timetable_hour().lock_mut().len()];
            day.hours = l;
        }
        else{
            let l = vec![false; selected_timetable_hour().lock_mut().len()];
            day.hours = l;
        }
    }
    teacher_limitations().lock_mut().replace_cloned(lims);
}

fn all_hours(hour: usize){
    let mut lims = teacher_limitations().lock_mut().to_vec();
    if lims.iter().any(|day| !day.hours[hour]){
        lims.iter_mut().for_each(|day| day.hours[hour] = true)
    }
    else{
        lims.iter_mut().for_each(|day| day.hours[hour] = false)
    }
    teacher_limitations().lock_mut().replace_cloned(lims)
}

fn day_hour(day_id: usize, hour: usize){
    let mut lims = teacher_limitations().lock_mut().to_vec();
    lims[day_id].hours[hour] = !lims[day_id].hours[hour];
    teacher_limitations().lock_mut().replace_cloned(lims);
}

fn save_changes(){
    let lims = teacher_limitations().lock_mut().to_vec();
    let group_id = selected_timetable().get();
    let c_msg = TeacherUpMsgs::UpdateLimitations((group_id, lims.clone()));
    let t_msg = TimetableUpMsgs::Teacher(c_msg);
    let msg = UpMsg::Timetable(t_msg);
    send_msg(msg);
}

