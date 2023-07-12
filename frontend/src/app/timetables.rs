use std::collections::HashMap;
use crate::connection::send_msg;
use crate::i18n::t;
use crate::app::*;
use shared::UpMsg;
use shared::msgs::teachers::{TeacherLimitation, TeacherUpMsgs};
use zoon::strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use zoon::named_color::*;
use shared::msgs::timetables::*;
use shared::msgs::classes::*;
use shared::msgs::activities::*;
use shared::msgs::lectures::*;


use self::add_act::{change_act_classes, change_act_teachers};
use self::class::selected_class;
use self::classes::{classes_page_view, classes};
use self::teachers::{teachers, selected_teacher};



pub mod classes;
pub mod teacher;
pub mod teachers;
pub mod generator;
pub mod class;
pub mod generate;
pub mod add_act;

#[static_ref]
pub fn school() -> &'static Mutable<Option<School>> {
    Mutable::new(None)
}

#[static_ref]
fn selected_page() -> &'static Mutable<TimetablePages> {
    Mutable::new(TimetablePages::default())
}

#[static_ref]
pub fn timetables() -> &'static MutableVec<Timetable> {
    MutableVec::new_with_values(vec![])
}

pub fn get_timetables() {
    use crate::connection::*;
    use shared::*;
    //let tt_msg = TimetableUpMsgs::GetTimetables;
    send_msg(UpMsg::GetTimetables);
}

#[static_ref]
pub fn selected_timetable() -> &'static Mutable<i32> {
    Mutable::new(0)
}

#[static_ref]
pub fn selected_timetable_hour() -> &'static MutableVec<i32> {
    MutableVec::new_with_values(vec![])
}
#[static_ref]
pub fn activities() -> &'static MutableVec<FullActivity> {
    MutableVec::new_with_values(vec![])
}

#[static_ref]
pub fn lectures() -> &'static MutableVec<Lecture> {
    MutableVec::new_with_values(vec![])
}

pub fn get_activities() {
    use shared::*;
    let t_msg = TimetableUpMsgs::GetActivities(selected_timetable().get());
    let msg = UpMsg::Timetable(t_msg);
    send_msg(msg);
}
pub fn get_lectures() {
    use shared::*;
    let msg = UpMsg::GetLectures;
    send_msg(msg);
}
#[static_ref]
pub fn schedules() -> &'static MutableVec<Schedule> {
    MutableVec::new_with_values(vec![])
}
pub fn get_schedules() {
    use shared::*;
    let t = selected_timetable().get();
    let msg = UpMsg::Timetable(TimetableUpMsgs::GetSchedules(t));
    send_msg(msg);
}
pub fn change_timetable(value: String) {
    let id = value.parse::<i32>().unwrap();
    if let Some(timetable) = timetables().lock_ref().iter().find(|t| t.id == id) {
        //println!("{}", &timetable.hour);
        selected_timetable_hour()
            .lock_mut()
            .replace_cloned(vec![0; timetable.hour as usize]);
    };
    selected_timetable().set(id);
    change_page(TimetablePages::Classes);
    self::classes::get_classes();
}

fn change_page(p: TimetablePages) {
    selected_class().set(None);
    selected_teacher().set(None);
    change_act_classes();
    change_act_teachers();
    selected_page().set(p)
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct School {
    pub id: i32,
    pub name: String,
}

pub fn timetable_tabs() -> impl Element {
    Row::new()
        .s(Padding::new().bottom(25))
        .s(Gap::new().x(50))
        .s(Align::center())
        .s(Font::new().weight(FontWeight::Medium))
        .item(
            RawHtmlEl::new("select").children_signal_vec(timetables().signal_vec_cloned().map(
                |group| {
                    RawHtmlEl::new("option")
                        .event_handler(move |_event: events::Click| {
                            change_timetable(group.id.to_string())
                        })
                        .child(format!("{}({})", group.name, group.id))
                },
            )),
        )
        .items(TimetablePages::iter().map(|page| {
            Button::new()
                .s(
                    Borders::new().bottom_signal(selected_page().signal_ref(move |p| {
                        if p == &page {
                            Border::new().width(2).solid().color(BLUE_5)
                        } else {
                            Border::new().width(0).solid().color(GRAY_0)
                        }
                    })),
                )
                //.s(Width::exact(150))
                .on_click(move || change_page(page))
                .label_signal(t!(format!("{}", page.label())))
        }))
}

pub fn pages_view()-> impl Element{
    Column::new().item_signal(selected_page().signal().map(|page| {
        match page{
            TimetablePages::Classes => classes_page_view().into_raw_element(),
            TimetablePages::Teachers => teachers::home().into_raw_element(),
            TimetablePages::Generator => generator::home().into_raw_element(),
        }
    }))
}
#[derive(Clone, Copy, IntoStaticStr, EnumIter, Debug, Default, PartialEq)]
#[strum(crate = "strum")]
enum TimetablePages {
    #[default]
    Classes,
    Teachers,
    Generator,
}

impl TimetablePages {
    fn label<'a>(&'a self) -> &'a str {
        match self {
            Self::Classes => "classes",
            Self::Teachers => "teachers",
            Self::Generator => "generator",
        }
    }
}

pub fn get_school() {
    let msg = shared::UpMsg::GetSchool;
    send_msg(msg)   
}

#[static_ref]
pub fn classes_limitations() -> &'static Mutable<HashMap<i32, Vec<ClassLimitation>>> {
    Mutable::new(HashMap::new())
}
#[static_ref]
pub fn teachers_limitations() -> &'static Mutable<HashMap<i32, Vec<TeacherLimitation>>> {
    Mutable::new(HashMap::new())
}
pub fn get_classes_limitations() {
    use shared::msgs::*;
    let a = selected_timetable().get();
    let msg = shared::UpMsg::Timetable(timetables::TimetableUpMsgs::GetClassesLimitations(a));
    crate::connection::send_msg(msg);
}

pub fn get_teachers_limitations() {
    use shared::msgs::*;
    let a = selected_timetable().get();
    let msg = shared::UpMsg::Timetable(timetables::TimetableUpMsgs::GetTeachersLimitations(a));
    crate::connection::send_msg(msg);
}
pub fn create_teachers_limitations(mut lims: Vec<TeacherLimitation>){
    let clss = teachers().lock_mut().to_vec();
    let t_len = selected_timetable_hour().lock_mut().to_vec().len();
    for c in &clss{
        let c_lims = lims.iter().filter(|l| l.user_id == c.id.clone()).collect::<Vec<&TeacherLimitation>>();
        let mut shifted = false;
        for i in 1..=7{
            if let Some(c_l) = c_lims.iter().find(|cl| cl.day == i){
                if c_l.hours.len() < t_len{
                    shifted = true;
                    break;
                }
            }
            else{
                shifted = true;
                break;
            }
        }
        if shifted{
            lims.retain(|l| l.user_id != c.id);
            lims.append(&mut create_teacher_lim(c.id));
        }
    }
    let mut new_lims: HashMap<i32, Vec<TeacherLimitation>> = HashMap::new();
    for c in &clss{
        let mut c_lims = lims.clone().into_iter().filter(|l| l.user_id == c.id.clone()).collect::<Vec<TeacherLimitation>>();
        c_lims.sort_by(|a,b| a.day.cmp(&b.day));
        new_lims.insert(c.id, c_lims.clone());
    }
    timetables::teachers_limitations().set(new_lims);
}
pub fn create_classes_limitations(mut lims: Vec<ClassLimitation>){
    let clss = classes::classes().lock_mut().to_vec();
    //let clss = clss.values();
    let t_len = selected_timetable_hour().lock_mut().to_vec().len();
    for c in &clss{
        let c_lims = lims.iter().filter(|l| l.class_id == c.id.clone()).collect::<Vec<&ClassLimitation>>();
        let mut shifted = false;
        for i in 1..=7{
            if let Some(c_l) = c_lims.iter().find(|cl| cl.day == i){
                if c_l.hours.len() < t_len{
                    shifted = true;
                    break;
                }
            }
            else{
                shifted = true;
                break;
            }
        }
        if shifted{
            lims.retain(|l| l.class_id != c.id);
            lims.append(&mut create_default_lim(c.id));
        }
    }
    let mut new_lims: HashMap<i32, Vec<ClassLimitation>> = HashMap::new();
    for c in &clss{
        let c_lims = lims.clone().into_iter().filter(|l| l.class_id == c.id.clone()).collect::<Vec<ClassLimitation>>();
        new_lims.insert(c.id, c_lims.clone());
    }
    timetables::classes_limitations().set(new_lims);
}

fn create_default_lim(class_id: i32)-> Vec<ClassLimitation>{
    let t_len = selected_timetable_hour().lock_mut().len();
    let mut lims: Vec<ClassLimitation> = vec![];
    for day in 1..=7{
        let new_lim = ClassLimitation{
            class_id,
            day,
            hours: vec![true; t_len]
        };
        lims.push(new_lim);
    }
    let c_msg = ClassUpMsgs::UpdateLimitations((class_id, lims.clone()));
    let t_msg = TimetableUpMsgs::Class(c_msg);
    let msg = UpMsg::Timetable(t_msg);
    send_msg(msg);
    lims
}

fn create_teacher_lim(user_id: i32)-> Vec<TeacherLimitation>{
    let t_len = selected_timetable_hour().lock_mut().len();
    let group_id = selected_timetable().get();
    let mut lims: Vec<TeacherLimitation> = vec![];
    for day in 1..=7{
        let new_lim = TeacherLimitation{
            user_id,
            school_id: school().get_cloned().unwrap().id,
            group_id: selected_timetable().get(),
            day,
            hours: vec![true; t_len]
        };
        lims.push(new_lim);
    }
    let c_msg = TeacherUpMsgs::UpdateLimitations((group_id, lims.clone()));
    let t_msg = TimetableUpMsgs::Teacher(c_msg);
    let msg = UpMsg::Timetable(t_msg);
    send_msg(msg);
    lims
}

