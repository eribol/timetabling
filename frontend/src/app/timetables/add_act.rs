use shared::UpMsg;
use shared::msgs::activities::{ActivityUpMsgs, FullActivity};
use shared::msgs::{classes::Class, activities::AddActivity};
use shared::msgs::lectures::Lecture;
use shared::msgs::teachers::Teacher;
use zoon::{named_color::*, *};

use crate::i18n::t;
use crate::connection::send_msg;
use crate::elements::buttons;
use super::{activities, schedules};
use super::class::selected_class;
use super::{lectures, classes, selected_timetable, teachers::{teachers, selected_teacher}};

#[static_ref]
pub fn add_act()->&'static Mutable<bool>{
    Mutable::new(true)
}

#[static_ref]
fn teachers_modal()->&'static Mutable<bool>{
    Mutable::new(false)
}
#[static_ref]
fn classes_modal()->&'static Mutable<bool>{
    Mutable::new(false)
}
#[static_ref]
fn lecture_modal()->&'static Mutable<bool>{
    Mutable::new(false)
}
#[static_ref]
fn selected_lecture()->&'static Mutable<Option<Lecture>>{
    Mutable::new(None)
}
#[static_ref]
fn filtered_lectures()->&'static MutableVec<Lecture>{
    MutableVec::new_with_values(vec![])
}
#[static_ref]
fn act_hour()->&'static Mutable<String>{
    Mutable::new("".to_string())
}
#[static_ref]
fn filtered_teachers()->&'static MutableVec<Teacher>{
    MutableVec::new_with_values(vec![])
}
#[static_ref]
fn filtered_classes()->&'static MutableVec<Class>{
    MutableVec::new_with_values(vec![])
}
#[static_ref]
fn act_classes()->&'static MutableVec<Class>{
    MutableVec::new_with_values(vec![])
    //MutableVec::new_with_values(vec![])
}

pub fn change_act_classes(){
    let class = super::class::selected_class();
    if let Some(cls) = class.get_cloned(){
        act_classes().lock_mut().replace_cloned(vec![cls.clone()])
    }
}
#[static_ref]
fn classes_len_signal()->&'static Mutable<bool>{
    Mutable::new(false)
}
#[static_ref]
fn act_teachers()->&'static MutableVec<Teacher>{
    MutableVec::new_with_values(vec![])
}
pub fn change_act_teachers(){
    let t = selected_teacher().get();
    if let Some(teacher) = t{
        let teachers = teachers().lock_mut().to_vec();
        let teacher = teachers.iter().find(|t2| t2.id == teacher).unwrap();
        act_teachers().lock_mut().replace_cloned(vec![teacher.clone()])
    }
}
#[static_ref]
fn teachers_len_signal()->&'static Mutable<bool>{
    Mutable::new(false)
}

fn filter_lectures(value: String){
    if value.is_empty(){
        lecture_modal().set(false);
    }
    else{
        lecture_modal().set(true);
        let lects = lectures().lock_mut().to_vec();
        let f_lects = lects.into_iter().filter(|lec| lec.name.to_uppercase().contains(&value.to_uppercase())).collect::<Vec<Lecture>>();
        filtered_lectures().lock_mut().replace_cloned(f_lects)
    }
    
}

fn filter_classes(value: String){
    if value.len() > 0{
        classes_modal().set(true);
        filtered_classes()
        .lock_mut()
        .replace_cloned(
            classes()
            .lock_mut()
            //.to_vec()
            .iter()
            .filter(|class| 
                class.label().to_uppercase().contains(&value.to_uppercase())
            ).map(|c| c.clone()).collect()
        )
    }
    else{
        classes_modal().set(false)
    }
}
fn filter_teachers(value: String){
    if value.len() > 0{
        teachers_modal().set(true);
        filtered_teachers()
        .lock_mut()
        .replace_cloned(
            teachers()
            .lock_mut()
            .to_vec()
            .into_iter()
            .filter(|teacher|{
                let a_teachers = act_teachers().lock_mut().to_vec();
                teacher.label_full().to_uppercase().contains(&value.to_uppercase()) && a_teachers.iter().all(|t2| t2.id != teacher.id)    
            }
            ).collect()
        )
    }
    else{
        teachers_modal().set(false)
    }
}

pub fn change_add_act(){
    add_act().set_neq(!add_act().get())
}
fn change_hour(value: String){
    act_hour().set(value)   
}

fn add_act_view()->impl Element{
    Column::new()
        .s(Gap::new().y(10))
        .item(lectures_view())
        .item(hour_view())
        .item(
            activity_classes()
        ).item(activity_teachers())
}

pub fn home()->impl Element{
    let mut id = 0;
    if let Some(i) = selected_class().get_cloned(){
        id = i.id;
    }
    else{
        if let Some(i) = selected_teacher().get_cloned(){
            id = i;
        }
    }
    Column::new()
    .item_signal(
        add_act().signal().map_true(add_act_view)
    )
    .item({
        Row::new()
        .item_signal(t!("total-act-hours"))
        .item_signal(activities()
            .signal_vec_cloned()
            .filter_signal_cloned(move |acts| 
                Mutable::new(
                    acts.classes.iter()
                    .any(|c| c == &id) && !schedules().lock_ref().iter().any(|s| s.activity == acts.id)
                )
                .signal()
            ).len()
        )
    })
    .item(alt_buttons())
}

fn alt_buttons()->impl Element{
    Column::new()
    .s(Align::center())
    .s(Width::fill())
    .item(
        buttons::default_with_signal(t!("add"))
        .on_click(send_act)
    )
}

fn lectures_view()->impl Element{
    Column::new()
    .item(
        Label::new().label_signal(t!("lecture"))
    ).item(
        Label::new().s(Font::new().weight(FontWeight::ExtraLight)).label_signal(t!("select-lecture"))
    )
    .item_signal(
        selected_lecture().signal_ref(|lec|
            match &lec{
                Some(l) => Row::new()
                .s(Align::new().left())
                .s(Padding::new().left(3).right(3))
                .s(Gap::new().x(10))
                .s(RoundedCorners::all(50))
                .s(Background::new().color(GRAY_3))
                .s(Borders::all(Border::new().width(2).solid().color(GRAY_1)))
                .item(
                    Button::new().label(&l.name)
                ).item(
                        RawHtmlEl::new("i")
                        .style("cursor", "pointer")
                        .attr("class", "fa-solid fa-xmark").event_handler(|_event: events::Click|{
                            selected_lecture().set(None);
                            //close_lectures_modal()
                        })
                ),
                None => Row::new()
                    .s(Align::new().left())
                    .item(
                    TextInput::new()
                    .s(Borders::all(Border::new().width(1).dotted().color(GRAY_5)))
                    .id("subject").s(Align::new().left())
                    .on_change(filter_lectures))
            })
    ).element_below_signal(
        lecture_modal().signal().map_true(||
            lectures_modal_view("lectures")
        )
    )
}
fn hour_view()->impl Element{
    Column::new().item(
        Label::new().label_signal(t!("act-hour"))
    )
    .item(
        Label::new()
        .s(Font::new().weight(FontWeight::ExtraLight))
        .label_signal(t!("use-space-for-blocks"))
    )
    .item(
        TextInput::new()
        .s(Borders::all(Border::new().width(1).dotted().color(GRAY_5)))
        .id("hour").s(Align::new().left())
        .on_change(change_hour)
    )
}
fn activity_classes()-> impl Element{
    let a = Mutable::new(false);
    Column::new()
    .s(Borders::all_signal(
        a.signal().map_bool(|| Border::new().width(1).color(BLUE_2).solid(), || Border::new().width(1).color(BLUE_1).solid())
    ))
    .s(RoundedCorners::new().bottom(2).top(2))
    .on_hovered_change(move |hovered| a.set_neq(hovered))
    .item(
        Label::new().label_signal(t!("activity-classes"))
    )
    .item(
        Label::new()
        .s(Font::new().weight(FontWeight::ExtraLight))
        .label_signal(t!("select-act-classes"))
    )
    .item(
        Row::new()
        .s(Gap::new().x(10))
        .items_signal_vec(
            act_classes().signal_vec_cloned().map(|c| 
                Row::new()
                .s(Padding::new().left(3).right(3))
                .s(Gap::new().x(10))
                .s(RoundedCorners::all(50))
                .s(Background::new().color(GRAY_3))
                .s(Borders::all(Border::new().width(2).solid().color(GRAY_1)))
                .item(
                    Button::new().label(c.label())
                ).item_signal(
                    selected_class().signal_cloned().map_option(move |c2|{
                        if c2.id == c.id{
                            RawHtmlEl::new("i")    
                        }
                        else{
                            RawHtmlEl::new("i")
                            .style("cursor", "pointer")
                            .attr("class", "fa-solid fa-xmark")
                            .event_handler(move |_event: events::Click|
                                {
                                    classes_len_signal().set(false);
                                    act_classes().lock_mut().retain(|c2| c2.id != c.id)
                                }
                            )
                        }},
                        move || 
                            RawHtmlEl::new("i")
                            .style("cursor", "pointer")
                            .attr("class", "fa-solid fa-xmark")
                            .event_handler(move |_event: events::Click|
                                {
                                    classes_len_signal().set(false);
                                    act_classes().lock_mut().retain(|c2| c2.id != c.id)    
                                }
                            )
                    )
                )
        )).item(
            Row::new().item_signal(
                classes_len_signal().signal().map_false(||   
                TextInput::new()
                .s(
                    Borders::all(Border::new().width(1).dotted().color(GRAY_5))
                )
                .id("classes")
                .on_change(filter_classes))
            ).element_below_signal(classes_modal().signal().map_true(||
                class_modal_view("classes"))
            )
        )
    )
}

fn activity_teachers()-> impl Element{
    let a = Mutable::new(false);
    Column::new()
    .s(Borders::all_signal(
        a.signal().map_bool(|| Border::new().width(1).color(BLUE_2).solid(), || Border::new().width(1).color(BLUE_1).solid())
    ))
    .s(RoundedCorners::new().bottom(2).top(2))
    .on_hovered_change(move |hovered| a.set_neq(hovered))
    .item(
        Label::new().label_signal(t!("activity-teachers"))
    )
    .item(
        Label::new()
        .s(Font::new().weight(FontWeight::ExtraLight))
        .label_signal(t!("select-act-teachers"))
    )
    .item(
        Row::new()
        .s(Gap::new().x(10))
        .items_signal_vec(
            act_teachers().signal_vec_cloned()
            .map(|t| 
                Row::new()
                .s(Padding::new().left(3).right(3))
                .s(Gap::new().x(10))
                .s(RoundedCorners::all(50))
                .s(Background::new().color(GRAY_3))
                .s(Borders::all(Border::new().width(2).solid().color(GRAY_5)))
                .item(
                    Button::new().label(t.first_name)
                )
                .item_signal(
                    selected_teacher().signal_cloned().map_option(move |t2|{
                        if t2 == t.id{
                            RawHtmlEl::new("i")    
                        }
                        else{
                            RawHtmlEl::new("i")
                            .style("cursor", "pointer")
                            .attr("class", "fa-solid fa-xmark")
                            .event_handler(move |_event: events::Click|
                                {
                                    teachers_len_signal().set(false);
                                    act_teachers().lock_mut().retain(|t2| t2.id != t.id)
                                }
                            )
                        }},
                        move || 
                            RawHtmlEl::new("i")
                            .style("cursor", "pointer")
                            .attr("class", "fa-solid fa-xmark")
                            .event_handler(move |_event: events::Click|
                                {
                                    teachers_len_signal().set(false);
                                    act_teachers().lock_mut().retain(|t2| t2.id != t.id)    
                                }
                            )
                    )
                )
            )
        ).item(
            Row::new().item_signal(
                teachers_len_signal().signal().map_false(||   
                TextInput::new()
                .s(
                    Borders::all(Border::new().width(1).dotted().color(GRAY_1))
                )
                .id("classes")
                .on_change(filter_teachers))
            ).element_below_signal(teachers_modal().signal().map_true(||
                teacher_modal_view("teachers"))
            )
        )
    )
}

pub fn lectures_modal_view(id: &str) -> impl zoon::Element {
    run_once!(|| {
        global_styles().style_group(StyleGroup::new(".below > *").style("pointer-events", "auto"));
    });
    //use zoon::HasIds;
    zoon::Column::new()
        .id("class_modal")
        .s(Background::new().color(hsluv!(200,100,100)))
        .s(Borders::all(Border::new().width(1).solid()))
        //.s(zoon::Width::exact(50))
        .s(zoon::Align::new().right())
        .s(zoon::Padding::all(5))
        .on_click_outside_with_ids(close_lectures_modal, [id])
        //.after_remove(|_| crate::header::close_menu())
        .items_signal_vec(
            filtered_lectures().signal_vec_cloned()
            .map(|lecture|{
                Button::new()
                .label(
                    format!("{}({})", &lecture.name, &lecture.kademe)
                )
                .s(Height::exact(15))
                .on_click(move ||{
                    lecture_modal().set(false);
                    selected_lecture().set(Some(lecture.clone()));
                })
            })
        )
        //.on_click(|| set_lang("a"))
        .update_raw_el(|raw_el| {
            raw_el
                .class("below")
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("position", "absolute")
                .style("top", "100%")
                .style("left", "0")
                //.style("width", "100%")
                .style("pointer-events", "none")
                .style("z-index", "100")
        })
}
pub fn class_modal_view(id: &str) -> impl zoon::Element {
    run_once!(|| {
        global_styles().style_group(StyleGroup::new(".below > *").style("pointer-events", "auto"));
    });
    //use zoon::HasIds;
    zoon::Column::new()
        .id("class_modal")
        .s(Background::new().color(hsluv!(200,100,100)))
        .s(Borders::all(Border::new().width(1).solid()))
        //.s(zoon::Width::exact(50))
        .s(zoon::Align::new().right())
        .s(zoon::Padding::all(5))
        .on_click_outside_with_ids(close_classes_modal, [id])
        //.after_remove(|_| crate::header::close_menu())
        .items_signal_vec(
            filtered_classes().signal_vec_cloned()
            .map(|class|{
                Button::new()
                .label(
                    class.label()
                )
                .s(Height::exact(15))
                .on_click(move ||{
                    classes_modal().set(false);
                    act_classes().lock_mut().push_cloned(class.clone());
                    if act_classes().lock_mut().to_vec().len() >= 2{
                        classes_len_signal().set(true);
                    }
                })
            })
        )
        //.on_click(|| set_lang("a"))
        .update_raw_el(|raw_el| {
            raw_el
                .class("below")
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("position", "absolute")
                .style("top", "100%")
                .style("left", "0")
                //.style("width", "100%")
                .style("pointer-events", "none")
                .style("z-index", "100")
        })
}
pub fn teacher_modal_view(id: &str) -> impl zoon::Element {
    run_once!(|| {
        global_styles().style_group(StyleGroup::new(".below > *").style("pointer-events", "auto"));
    });
    //use zoon::HasIds;
    zoon::Column::new()
        .id("class_modal")
        .s(Background::new().color(hsluv!(200,100,100)))
        .s(
            Borders::new()
            .top(Border::new()
                .width(1).solid()
            ).right(Border::new()
                .width(1).solid())
            .left(Border::new()
                .width(1).solid())
        )
        .s(zoon::Width::exact(150))
        .s(zoon::Align::new().right())
        //.s(zoon::Padding::all(5))
        .on_click_outside_with_ids(close_teachers_modal, [id])
        //.after_remove(|_| crate::header::close_menu())
        .items_signal_vec(
            filtered_teachers().signal_vec_cloned()
            .map(|teacher|{
                Button::new()
                .s(Height::exact(25))
                .s(Borders::new().bottom(Border::new().width(1).solid()))
                .label(
                    Label::new()
                    .s(Align::center())
                    .s(Cursor::new(CursorIcon::Pointer))
                    .label(
                        format!("{} {}", teacher.first_name, teacher.last_name)
                    )
                )
                .on_click(move ||{
                    teachers_modal().set(false);
                    act_teachers().lock_mut().push_cloned(teacher.clone());
                    if act_teachers().lock_mut().to_vec().len() >= 2{
                        teachers_len_signal().set(true);
                    }
                })
            })
        )
        //.on_click(|| set_lang("a"))
        .update_raw_el(|raw_el| {
            raw_el
                .class("below")
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("position", "absolute")
                .style("top", "100%")
                .style("left", "0")
                //.style("width", "100%")
                .style("pointer-events", "none")
                .style("z-index", "100")
        })
}
fn close_lectures_modal(){
    lecture_modal().set(!lecture_modal().get())
}
fn close_classes_modal(){
    classes_modal().set(false)
}
fn close_teachers_modal(){
    classes_modal().set(false)
}

pub fn send_act(){
    let clss = act_classes().lock_mut().to_vec().iter().map(|c| c.id).collect::<Vec<i32>>();
    let tchrs = act_teachers().lock_mut().to_vec().iter().map(|c| c.id).collect::<Vec<i32>>();
    let lec = selected_lecture().get_cloned().unwrap().id;
    act_hour().get_cloned().trim_end().split(" ").for_each(|hour|{
        if let Ok(h) = hour.parse::<i16>(){
            let a = AddActivity{
                subject: lec,
                hour: h,
                classes: clss.clone(),
                teachers: tchrs.clone()
            };
            let a_msg = ActivityUpMsgs::AddAct((selected_timetable().get(), a));
            let msg = UpMsg::Activity(a_msg);
            send_msg(msg);    
        }
    });
}
pub fn lecture_name(act: FullActivity) -> String {
    //let mut name = "A".to_string();
    let tchrs = super::lectures().lock_mut().to_vec();
    let lec = tchrs
        .iter()
        .find(|t1| t1.id == act.subject).unwrap();
    let name = format!("{}", lec.short_name);
    name
}

pub fn teacher_short_name(act: FullActivity) -> String {
    let tchrs = teachers().lock_mut().to_vec();
    let tec = tchrs
        .iter()
        .filter(|t1| act.teachers.iter().any(|t2| t2 == &t1.id))
        .map(|teacher| teacher.label()).collect::<String>();
    tec
}

pub fn teachers_full_name(act: FullActivity) -> String {
    //let mut name = "A".to_string();
    let tchrs = teachers().lock_mut().to_vec();
    let tec = tchrs
        .iter()
        .filter(|t1| act.teachers.iter().any(|t2| t2 == &t1.id))
        .map(|teacher| teacher.label_full()+" ").collect::<String>();
    tec
}

pub fn classes_full_name(act: FullActivity) -> String {
    //let mut name = "A".to_string();
    let clss = classes().lock_mut().to_vec();
    let full_cl = act.classes.iter()
    .map(|id|{
        let cl = clss.iter().find(|c| c.id == *id)
        .unwrap();
        format!("{}{}",cl.kademe, cl.sube)
    }).collect::<String>();
    full_cl
}

pub fn del_act(act: FullActivity){
    let timetable = selected_timetable().get();
    let a_msg = ActivityUpMsgs::DelAct((timetable, act));
    let msg = UpMsg::Activity(a_msg);
    send_msg(msg)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ActCol{
    pub act: Mutable<FullActivity>,
    pub del: Mutable<bool>,
}