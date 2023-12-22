use std::collections::HashMap;

use shared::UpMsg;
use shared::msgs::activities::{FullActivity, Activity};
use shared::msgs::teachers::TeacherUpMsgs;
use shared::msgs::timetables::TimetableUpMsgs;
use zoon::*;
use zoon::named_color::*;
use crate::app::timetables::add_act::{lecture_name, classes_full_name};
use crate::app::timetables::class::limitations::{LIM_HEIGHT, LIM_WIDTH, show_lim_view};
use crate::app::timetables::generator::data;
use crate::app::timetables::teachers::selected_teacher;
use crate::app::timetables::{selected_timetable_hour, teachers_limitations, create_default_lim, selected_timetable, schedules, activities};
use crate::connection::send_msg;
use crate::elements::buttons;
use crate::i18n::t;

use shared::msgs::{timetables::Schedule, teachers::TeacherLimitation};

use super::activities::{self, move_select};


#[static_ref]
pub fn teacher_limitations() -> &'static MutableVec<TeacherLimitation> {
    MutableVec::new_with_values(vec!())
}

#[static_ref]
pub fn tat() -> &'static Mutable<HashMap<i32, Vec<TeacherLimitation>>> {
    Mutable::new(HashMap::new())
}

pub fn get_t_l(){
    if let Some(id) = selected_teacher().get(){
        let lim = teachers_limitations().get_cloned();
        let lim = lim.get(&id);
        loop{
            if let Some(lims) = lim{
                teacher_limitations().lock_mut().replace_cloned(lims.clone());
                break;
            }
        }
    }
}
#[static_ref]
pub fn loaded_lims()->&'static Mutable<bool>{
    Mutable::new(true)
}
pub fn limitations_view()->impl Element{
    El::new()
    .s(Align::new().top())
    .child_signal(show_lim_view().signal().map_true(|| schedule_table()))
}
pub fn schedule_table() -> impl Element {
    Column::new()
    .item(
        Row::new()
        .s(Align::new().left())
        .item(hours_column_view())
        .items(
            crate::DAYS
            .iter()
            .enumerate()
            .map(|day| El::new()
                .child(lim_col_view(day.0+1))
            )
        )
    )
    .item(
        bottom_buttons()
    )
}
fn hours_column_view()-> impl Element{
    Column::new()
        .s(Align::new().top())
        .item(Button::new()
            .s(Height::exact(LIM_HEIGHT))
            .s(Width::exact(LIM_WIDTH))
            .label("Günler/Saatler")
            .s(Borders::all(Border::new().width(1).solid().color(BLUE_3))),
        )
        .items_signal_vec(super::super::selected_timetable_hour()
            .signal_vec_cloned()
            .enumerate()
            .map(|hour| {
                let h = hour.0.get().unwrap_throw();
                Button::new()
                .on_click(move|| all_hours(h))
                .label(h+1)
                .s(Height::exact(LIM_HEIGHT))
                .s(Width::exact(LIM_WIDTH))
                .s(Borders::new()
                    .bottom(Border::new().width(1).solid().color(BLUE_3))
                    .left(Border::new().width(1).solid().color(BLUE_3))
                    .right(Border::new().width(1).solid().color(BLUE_3)))
            }),
        )
}
fn lim_col_view(day: usize)-> impl Element{
    Column::new()
        .s(Align::new().top())
        .item(Button::new()
            //.s(Align::new())
            .s(Height::exact(LIM_HEIGHT))        
            .s(Width::exact(LIM_WIDTH))
            .label_signal(t!(crate::DAYS[day-1]))
            .s(Borders::new()        
                .bottom(Border::new().width(1).solid().color(BLUE_3))
                .top(Border::new().width(1).solid().color(BLUE_3))
                .right(Border::new().width(1).solid().color(BLUE_3))
            ).on_click(move || all_days(day-1))
        )
        .items_signal_vec(
            teacher_limitations().signal_vec_cloned()
                .enumerate()
                .filter_signal_cloned(
                    move |lims| 
                    Mutable::new(lims.1.day == day as i32).signal()
                ).map(|day| 
                    Column::new().items(
                        day.1.hours.iter().enumerate().map(|h| 
                            {
                                hour_view(h.1.clone(), day.1.clone(), h.0)
                                
                            }
                        )
                    )
                )
        ).update_raw_el(|raw_el|{
            raw_el.event_handler_with_options(
                    EventOptions::new().preventable(),
                    |event: events::Drop| {
                        event.stop_propagation();
                        event.prevent_default();
                        //drop_zone_active().set_neq(false);
                        
                        print!("drop")
                    },
                )
        })
}

fn placed(act: &FullActivity, hour: usize, day: usize){
    let h = act.hour;
    for i in 0..h{
        schedules().lock_mut()
        .push_cloned(
            Schedule{
                day_id: day as i32,
                locked: false,
                activity: act.id.clone(),
                hour: (hour as i16+i) 
            });
    };
    let acts = activities().lock_mut().to_vec();
    activities().lock_mut().replace_cloned(acts);
    move_select().set(None);
}

fn hour_view(h: bool, day: TeacherLimitation, hour: usize)->impl Element{
    //let d_id = d_clone.day;
    let s: Mutable<bool> = Mutable::new(h);
    Column::new()
    .s(Background::new()
        .color_signal(s.signal().map_bool(|| BLUE_1, || GRAY_2)))
    .s(Height::exact(LIM_HEIGHT))        
    .s(Width::exact(LIM_WIDTH))
    .s(Borders::new()        
        .bottom(Border::new().width(1).solid().color(BLUE_3))
        .right(Border::new().width(1).solid().color(BLUE_3))
    )
    .on_double_click(move || day_hour(day.day as usize-1, hour))
    .item_signal(
        activities::move_select()
        .signal_cloned().map_some(move |act|{
            Column::new()
            .item(
                Button::new()
                .s(Cursor::new(CursorIcon::Pointer))
                .label("Yerleştir")
                .on_press(move || placed(&act, hour, day.day as usize))
            )
        })
    )
    .item_signal(
        schedules()
        .signal_vec_cloned()
        .to_signal_map(move |s|{
            match if_schedule(s, day.day, hour){
                Some(s) => {
                    let acts = activities().lock_mut().to_vec();
                    let act = acts.iter().find(|a| a.id == s.activity).unwrap();
                    schedule_view(act).into_raw()
                },
                None => Column::new()
                    .item(Button::new().label("".to_string())).into_raw()
                }
        })
    )
}

fn if_schedule(schs: &[Schedule], day: i32, hour: usize)-> Option<&Schedule>{
    let a = schs.into_iter().find(|s2| &s2.day_id == &day && s2.hour == hour as i16 
        && activities()
        .lock_ref()
        .iter()
        .any(|a2| s2.activity == a2.id && a2.teachers
            .iter()
            .any(|c| c == &selected_teacher()
                .get_cloned().unwrap()
            )
        )
    );
    a
}

fn schedule_view(act: &FullActivity)->impl Element{
    let act = act.clone();
    Column::new()
    .item(
        Button::new()
        .s(Font::new().weight(FontWeight::Light))
        .label(
            lecture_name(act.clone())
        )
    ).item(
        Button::new()
        .label(
            classes_full_name(act.clone())
        )
    ).item(
        Row::new()
        .s(Align::center())
        .s(Gap::new().x(5))
        .s(Font::new().weight(FontWeight::ExtraLight))
        .item(
            Button::new()
            .label("Kaldır")
            .on_click(move || {
                let mut schs = schedules().lock_mut().to_vec();
                schs.retain(|s| s.activity != act.id);
                schedules().lock_mut().replace_cloned(schs);
            })
        ).item(Button::new().label("Kilitle").on_click(|| ()))
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
fn get_t_acts()->Vec<FullActivity>{
    let teacher = selected_teacher().get_cloned().unwrap();
    let acts = activities().lock_mut().to_vec();
    let t_acts = acts.into_iter().filter(|a| a.teachers.iter().any(|t| t == &teacher)).collect::<Vec<FullActivity>>();
    t_acts
}
fn save_changes(){
    let lims = teacher_limitations().lock_mut().to_vec();
    let group_id = selected_timetable().get();
    let c_msg = TeacherUpMsgs::UpdateLimitations((group_id, lims.clone()));
    let t_msg = TimetableUpMsgs::Teacher(c_msg);
    let msg = UpMsg::Timetable(t_msg);
    send_msg(msg);
    change_tat(lims);
}
fn change_tat(t_lim: Vec<TeacherLimitation>){
    use zoon::println;
    let teacher = selected_teacher().get_cloned().unwrap();
    let schdls = schedules().lock_mut().to_vec();
    let t_acts = get_t_acts();
    let mut dt = data().get_cloned();
    let mut tat2 = tat().lock_mut();
    let tat2 = tat2.get_mut(&teacher).unwrap();
    println!("geld");
    for t_l in t_lim{
        for h in t_l.hours.iter().enumerate(){
            if !h.1{
                
                let t_sch = schdls.clone().into_iter()
                .enumerate()
                .find(|sc| sc.1.day_id == t_l.day && sc.1.hour as usize == h.0 && t_acts.iter().any(|c_a| c_a.id == sc.1.activity));
                
                if let Some(ts) = t_sch{
                    println!("geldi");
                    let act = t_acts.iter().find(|a| a.id == ts.1.activity).unwrap();
                    let act = Activity{
                        id: act.id,
                        classes: act.classes.clone(),
                        teachers: act.teachers.clone(),
                        subject: act.subject,
                        hour: act.hour    
                    };
                    dt.delete_activity(&act);
                    if let Some(tt) = tat2.iter_mut().find(|tt| tt.day == t_l.day){
                        println!("geldii");
                        tt.hours[h.0] = false;
                    }
                }
            }
        }
    }
    println!("g");
    tat().replace(*dt.tat);
    println!("g2");
    schedules().lock_mut().replace_cloned(*dt.timetables);
}
