use std::ops::{Deref, DerefMut};

use shared::msgs::activities::FullActivity;
use zoon::{eprintln, *};
use zoon::named_color::*;
use crate::app::timetables::add_act::{lecture_name, teachers_full_name};
use crate::app::timetables::classes::classes;
use crate::app::timetables::{schedules, activities};
use crate::connection::*;
use crate::i18n::t;
use crate::elements::*;
use shared::msgs::{classes::*, timetables::{Schedule, TimetableUpMsgs}};

use super::selected_class;
use super::activities::{self, move_select};

pub const LIM_HEIGHT: u32 = 75;
pub const LIM_WIDTH: u32 = 120;
#[static_ref]
pub fn class_limitations() -> &'static MutableVec<ClassLimitation> {
    MutableVec::new_with_values(vec![])
}
pub fn create_class_lims(){
    let class_lim = super::super::classes_limitations().lock_mut();
    let id = selected_class().get_cloned().unwrap().id;
    let lim = class_lim.get(&id);
    if let Some(lim) = lim{
        class_limitations().lock_mut().replace_cloned(lim.clone()); 
    }
}

#[static_ref]
pub fn loaded_lims()->&'static Mutable<bool>{
    Mutable::new(false)
}

#[static_ref]
pub fn show_lim_view()->&'static Mutable<bool>{
    Mutable::new(true)
}
pub fn change_view(){
    let s = show_lim_view().get();
    show_lim_view().set(!s);
}
pub fn limitations_view()->impl Element{
    El::new()
    .s(Align::new().top())
    .child_signal(show_lim_view().signal().map_true(|| schedule_table()))
}
fn schedule_table() -> impl Element {
    Column::new()
    //.s(Align::new().top())
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
        alt_buttons()
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
                .label(h+1)
                .s(Height::exact(LIM_HEIGHT))
                .s(Width::exact(LIM_WIDTH))
                .s(Borders::new()
                    .bottom(Border::new().width(1).solid().color(BLUE_3))
                    .left(Border::new().width(1).solid().color(BLUE_3))
                    .right(Border::new().width(1).solid().color(BLUE_3))
                ).on_click(move||all_hours(h))
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
            ).on_click(move || change_day_lim(day))
        )
        .items_signal_vec(
            class_limitations().signal_vec_cloned()
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
fn placed(act: &FullActivity, hour: usize, day: i32){
    for i in 0..act.hour{
        schedules().lock_mut()
        .push_cloned(
            Schedule{
                day_id: day as i32,
                locked: false,
                activity: act.id.clone(),
                hour: (hour as i16+i) as i32
        });
    }
    let acts = activities().lock_mut().to_vec();
    activities().lock_mut().replace_cloned(acts);
    move_select().set(None)
}
fn hour_view(h: bool, day: ClassLimitation, hour: usize)->impl Element{
    let d_clone = day.clone();
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
    .on_double_click(move || change_lim(day.day, hour))
    .item_signal({
        activities::move_select()
        .signal_cloned().map(move |ms|{
            match ms{
                Some(s)=>{
                    Column::new().item(
                    Button::new()
                    .s(Cursor::new(CursorIcon::Pointer))
                    .label("Yerleştir")
                    .on_press(move || {
                        placed(&s, hour, day.day);
                    }))
                },
                None =>{
                    Column::new()
                    .item_signal(
                        schedules()
                        .signal_vec_cloned()
                        .to_signal_map(move |s|{
                            let a = s.iter().find(|s2| &s2.day_id == &d_clone.day && s2.hour == hour as i32 
                                && activities()
                                .lock_ref()
                                .iter()
                                .any(|a| s2.activity == a.id && a.classes
                                    .iter()
                                    .any(|c| c == &selected_class()
                                        .get_cloned().unwrap().id
                                    )
                                )
                            );
                            match a{
                                Some(s) => {
                                    let act_id = s.activity;
                                    let acts = activities().lock_mut().to_vec();
                                    let act = acts.iter().find(|a| a.id == s.activity).unwrap();
                                    Column::new()
                                    .item(
                                        Button::new()
                                        .label(
                                            lecture_name(act.clone())
                                        )
                                    ).item(
                                        Button::new()
                                        .label(
                                            //"A"
                                            teachers_full_name(act.clone())
                                        )
                                    )
                                    .item(
                                        Row::new()
                                        .s(Align::center())
                                        .s(Gap::new().x(5))
                                        .s(Font::new().weight(FontWeight::Light))
                                        .item(
                                            Button::new()
                                            .label("Kaldır")
                                            .on_click(move || {
                                                let mut schs = schedules().lock_mut().to_vec();
                                                //let s: Vec<(usize, Schedule)> = schs.into_iter().enumerate().filter(|sc| act_id == sc.1.activity).collect();
                                                schs.retain(|s| s.activity != act_id);
                                                schedules().lock_mut().replace_cloned(schs);
                                            })
                                        )
                                        .item(Button::new().label("Kilitle").on_click(|| ()))
                                    )
                                },
                                None => Column::new()
                                .item(Button::new().label("".to_string()))
                            }
                        })
                    )
                }
            }
        })
    })
}

fn alt_buttons()->impl Element{
    Row::new()
    .s(Align::center())
    .s(Padding::new().top(15))
    .s(Gap::new().x(10))
    .item(buttons::default_with_signal(t!("save-changes"))
        .on_click(add_lim)
    ).item(
        buttons::default_with_signal(t!("save-changes-for-all-classes"))
        .on_click(add_lim_classes)
    ).item(
        buttons::_default("Ders  programını kaydet")
        .on_click(add_lim_classes)
    )
}

fn change_lim(day_index: i32, hour: usize){
    let mut hours: Vec<bool> = vec![];
    if let Some(day) = class_limitations().lock_mut().get((day_index-1) as usize){
        hours = day.hours.clone();
        hours[hour] = !hours[hour];
    }
    class_limitations().lock_mut().set_cloned((day_index-1) as usize, ClassLimitation { class_id: selected_class().get_cloned().unwrap().id, day: day_index, hours});
}
fn change_day_lim(day_index: usize){
    let mut hours: Vec<bool> = vec![];
    if let Some(day) = class_limitations().lock_mut().get(day_index-1){
        hours = day.hours.clone();
        if hours.iter().any(|h| !h){
            hours.iter_mut().for_each(|h| *h=true);
        }
        else{
            hours.iter_mut().for_each(|h| *h=false);
        }
    }
    class_limitations().lock_mut().set_cloned(day_index-1, ClassLimitation { class_id: super::selected_class().get_cloned().unwrap().id, day: day_index as i32, hours});
}
fn all_hours(hour: usize){
    let mut lims = class_limitations().lock_mut().to_vec();
    if !lims.iter().all(|day| day.hours[hour]){
        lims.iter_mut().for_each(|day| day.hours[hour] = true);
    }
    else{
        lims.iter_mut().for_each(|day| day.hours[hour] = false);
    }
    class_limitations().lock_mut().replace_cloned(lims);
}

pub fn add_lim(){
    let form = class_limitations().lock_mut().to_vec();
    let class_id = selected_class().get_cloned().unwrap().id;
    let msg = ClassUpMsgs::UpdateLimitations((class_id, form));
    let t_msg = TimetableUpMsgs::Class(msg);
    send_msg(shared::UpMsg::Timetable(t_msg));
}

pub fn add_lim_classes(){
    let clss = classes().lock_mut().to_vec();
    let mut form = class_limitations().lock_mut().to_vec();
    for c in clss{
        form.iter_mut().for_each(|f| f.class_id = c.id);
        let c_msg = ClassUpMsgs::UpdateLimitations((c.id, form.clone()));
        let t_msg = TimetableUpMsgs::Class(c_msg);
        send_msg(shared::UpMsg::Timetable(t_msg));
    }
}