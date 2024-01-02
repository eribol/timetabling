use shared::UpMsg;
use shared::msgs::activities::{ActivityUpMsgs, FullActivity};
use shared::msgs::{classes::Class, activities::AddActivity};
use shared::msgs::lectures::Lecture;
use shared::msgs::teachers::Teacher;
use zoon::{named_color::*, *};

use crate::i18n::t;
use crate::connection::send_msg;
use crate::elements::buttons;
use super::class::limitations::show_lim_view;
use super::{TimetablePages, total_act_hours_for_classes, total_act_hours_for_teacher};
use super::class::selected_class;
use super::{lectures, classes, selected_timetable, teachers::{teachers, selected_teacher}};
use crate::app::timetables::selected_page;

#[static_ref]
pub fn add_act()->&'static Mutable<bool>{
    Mutable::new(true)
}

#[static_ref]
fn selected_lecture()->&'static Mutable<Option<Lecture>>{
    Mutable::new(None)
}
#[static_ref]
fn filtered_lectures()->&'static MutableVec<Lecture>{
    let lecs = lectures().lock_mut().to_vec();
    MutableVec::new_with_values(lecs)
}
#[static_ref]
fn act_hour()->&'static Mutable<String>{
    Mutable::new("".to_string())
}
#[static_ref]
fn filtered_teachers()->&'static MutableVec<Teacher>{
    let teachers = teachers().lock_mut().to_vec();
    MutableVec::new_with_values(teachers)
}
#[static_ref]
fn filtered_classes()->&'static MutableVec<Class>{
    let classes = classes().lock_mut().to_vec();
    MutableVec::new_with_values(classes)
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

fn filter_lectures(value: String){
        filtered_lectures()
        .lock_mut()
        .replace_cloned(
            lectures()
            .lock_mut()
            //.to_vec()
            .iter()
            .filter(|lec| 
                lec.name.to_uppercase().contains(&value.to_uppercase())
            ).map(|c| c.clone()).collect()
        )
}

fn filter_classes(value: String){
    
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
fn filter_teachers(value: String){
    //if value.len() > 0{
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

fn change_hour(value: String){
    act_hour().set(value)   
}

fn add_act_view()->impl Element{
    El::new()
    .child_signal(
        show_lim_view()
        .signal()
        .map_bool(|| add_act_col_view().into_raw(), || add_act_row_view().into_raw()))
}

fn add_act_col_view()->impl Element{
    Column::new()
    .s(Gap::new().y(10))
    .item(activity_teachers())
    .item(activity_classes())
    .item(activity_lecture())
    .item(hour_view())
}

fn add_act_row_view()->impl Element{
    Row::new()
    .s(Gap::new().x(10))
    .s(Width::fill())
    .item(activity_teachers())
    .item(activity_classes())
    .item(activity_lecture())
    .item(hour_view())
}

pub fn home()->impl Element{
    Column::new()
    .s(Gap::new().y(10))
    .item(
        add_act_view()
    )
    .item(alt_buttons())
    .item({
        //let total = Mutable::new(0);
        Row::new()
        .s(Font::new().weight(FontWeight::ExtraBold))
        .item_signal(t!("total-act-hours"))
        .item_signal(
            selected_page().map(|p|{
                match p{
                    TimetablePages::Classes => total_act_hours_for_classes().signal_map_cloned().key_cloned(selected_class().get_cloned().unwrap().id),
                    _ => total_act_hours_for_teacher().signal_map_cloned().key_cloned(selected_teacher().get_cloned().unwrap())
                }
               // _ => 
            })
        )
    })
    
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

fn hour_view()->impl Element{
    let a = Mutable::new(false);
    Column::new().item(
        Label::new().label_signal(t!("act-hour"))
    )
    .s(Borders::all_signal(
        a.signal().map_bool(|| Border::new().width(1).color(BLUE_2).solid(), || Border::new().width(1).color(BLUE_1).solid())
    ))
    .s(RoundedCorners::new().bottom(2).top(2))
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

fn teachers_view()->impl Element{
    Row::new()
    .multiline()
    .s(Gap::new().x(5))
    .s(Width::growable().max(400))
    .items_signal_vec(filtered_teachers().signal_vec_cloned().map(|teacher|{
        Button::new()
        .s(Padding::all(10))
        .s(
            Borders::all_signal(is_teacher_selected(teacher.id)
                .map_bool(|| Border::new().width(2).color(RED_3), || Border::new().width(1).color(BLUE_3)))
        )
        .s(RoundedCorners::all(5))
        .label(&teacher.short_name)
        .on_click(move||{
            let mut tt = act_teachers().lock_mut().to_vec();
            if tt.iter().any(|t| t.id == teacher.id){
                tt.retain(|t| t.id != teacher.id)
            }else{
                tt.push(teacher.clone());
            }
            act_teachers().lock_mut().replace_cloned(tt)
        })
    }))
}

fn is_teacher_selected(id: i32)->impl Signal<Item = bool>{
    act_teachers().signal_vec_cloned().to_signal_map(move |at| at.iter().any(move |a| a.id == id))
}
fn activity_teachers()-> impl Element{
    let a = Mutable::new(false);
    Column::new()
    .s(Align::new().top())
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
    .item( TextInput::new()
        .id("a")
        .s(Borders::all(Border::new().width(1).dotted().color(GRAY_5)))
        .placeholder(Placeholder::with_signal(t!("filter-teachers")))
        .on_change(filter_teachers)
    )
    .item(teachers_view())
}
fn is_class_selected(id: i32)->impl Signal<Item = bool>{
    act_classes().signal_vec_cloned().to_signal_map(move |at| at.iter().any(move |a| a.id == id))
}
fn activity_classes()-> impl Element{
    let a = Mutable::new(false);
    Column::new()
    .s(Align::new().top())
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
    .item( TextInput::new()
        .id("b")
        .s(Borders::all(Border::new().width(1).dotted().color(GRAY_5)))
        .placeholder(Placeholder::with_signal(t!("filter-classes")))
        .on_change(filter_classes)
    )
    .item(classes_view())
}
fn classes_view()->impl Element{
    Row::new()
    .multiline()
    .s(Gap::new().x(5))
    .s(Width::growable().max(400))
    .items_signal_vec(filtered_classes().signal_vec_cloned().map(|class|{
        Button::new()
        .s(Padding::all(10))
        .s(
            Borders::all_signal(is_class_selected(class.id)
                .map_bool(|| Border::new().width(2).color(RED_3), || Border::new().width(1).color(BLUE_3)))
        )
        .s(RoundedCorners::all(5))
        .label(format!("{}{}",class.kademe, class.sube))
        .on_click(move||{
            let mut tt = act_classes().lock_mut().to_vec();
            if tt.iter().any(|c| c.id == class.id){
                tt.retain(|c| c.id != class.id)
            }else{
                tt.push(class.clone());
            }
            act_classes().lock_mut().replace_cloned(tt)
        })
    }))
}

fn is_lecture_selected(id: i32)->impl Signal<Item = bool>{
    selected_lecture().signal_ref(move|lec|{
        if let Some(l) = lec{
            if l.id == id{
                true
            }
            else{
                false    
            }
        }
        else{
            false
        }
    })
}
fn activity_lecture()-> impl Element{
    let a = Mutable::new(false);
    Column::new()
    .s(Borders::all_signal(
        a.signal().map_bool(|| Border::new().width(1).color(BLUE_4).solid(), || Border::new().width(1).color(BLUE_2).solid())
    ))
    .s(RoundedCorners::new().bottom(2).top(2))
    .on_hovered_change(move |hovered| a.set_neq(hovered))
    .s(Align::new().top())
    .item(
        Label::new().label_signal(t!("lecture"))
    )
    .item(
        Label::new()
        .s(Font::new().weight(FontWeight::ExtraLight))
        //.label("label")
        .label_signal(t!("select-lecture"))
    )
    .item( TextInput::new()
        .id("b")
        .s(Borders::all(Border::new().width(1).dotted().color(GRAY_5)))
        .placeholder(Placeholder::with_signal(t!("filter-lectures")))
        .on_change(filter_lectures)
    )
    .item(lectures_view())
}
fn lectures_view()->impl Element{
    Row::new()
    .multiline()
    .s(Gap::new().x(5))
    .s(Width::growable().max(400))
    //.item("item")
    .items_signal_vec(filtered_lectures().signal_vec_cloned().map(|lec|{
        Button::new()
        .s(Padding::all(10))
        .s(
            Borders::all_signal(
                is_lecture_selected(lec.id)
                .map_bool(|| Border::new().width(2).color(RED_3), || Border::new().width(1).color(BLUE_3))
            )
        )
        .s(RoundedCorners::all(5))
        .label(format!("{}",lec.short_name))
        .on_click(move||{
            selected_lecture().set(Some(lec.clone()));
        })
    }))
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