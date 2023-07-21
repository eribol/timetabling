use std::collections::{HashMap, HashSet};

use shared::msgs::activities::FullActivity;
use shared::msgs::{timetables::*, activities::Activity};
use zoon::*;
use zoon::named_color::BLUE_3;
use crate::i18n::t;
use crate::elements::{text_inputs, buttons};

use super::{schedules, prints, activities, teachers_limitations, classes_limitations};

pub fn home() -> impl Element {
    Row::new()
    .s(Padding::new().top(50))
    .multiline()
    .s(Align::center())
    .item(
        generate_view()
    ).item(
        right_menu()
    )
}

fn generate_view()->impl Element{
    Column::new()
        .s(Padding::new().left(50))
        .s(Padding::new().right(50))
        .s(
            Shadows::new(vec![
                Shadow::new().blur(2).color(hsluv!(0, 0, 0, 20))
            ])
        )
        //.s(Borders::all(Border::new().width(1).color(BLUE_3)))
        .s(Gap::new().y(20))
        //
        .item(generator())
        .item(buttons())
        .item(information())
}

fn right_menu()-> impl Element{
    Column::new()
    .s(Gap::new().y(5))
    .s(
        Padding::new().left(20)
    )
    .item(
        buttons::default_with_signal(t!("save"))
    ).item(
        buttons::_default("Yazdır").on_click(|| prints::teachers::prints())
    ).item(
        buttons::_default("Yazdır-Sınıflar").on_click(|| prints::classes::prints())
    )
}

fn generator() -> impl Element {
    Column::new()
    .s(Gap::new().y(10))
    .item(hour_view())
}

fn hour_view()->impl Element{
    Column::new()
    .s(Gap::new().y(10))
    .item(
        Label::new()
        .s(Align::center())
        .label("Bir öğretmenin bir sınıfa verebileceği maksimum ders saati")
    )
    .item(
       text_inputs::default().id("hour").placeholder(Placeholder::new(3))
    )
}

fn information()-> impl Element{
    Column::new()
    .item(
        Row::new()
        .item(
            Label::new().label("Toplam ders sayısı: ")
        )
        .item(
            Label::new().label(total_hour().get().to_string())
        )
    )
    .item(
        Row::new().item(
            Label::new().label("Yerleştirilen ders sayısı: ")
        )
        .item(
            Label::new()
            .label_signal(
                schedules().signal_vec_cloned().len()
            )    
        )
    )
}

fn buttons()-> impl Element{
    Column::new()
    .s(Gap::new().x(10).y(5))
    .item(
        buttons::_default("Sil")
        //.label("Dağıt")
        .s(Width::growable())
        .s(Font::new().weight(FontWeight::Medium))
        .on_click(|| set_data())
    )
    .item(
        buttons::_default("Dağıt")
        //.label("Dağıt")
        .s(Width::growable())
        .on_press(|| generate() )
        .s(Font::new().weight(FontWeight::Bold))
    )
}

#[static_ref]
fn total_hour()->&'static Mutable<usize>{
    let mut t: usize = 0;
    super::activities().use_ref(|acts| acts.iter().for_each(|a| t = t + a.hour as usize));
    Mutable::new(t)
}
#[static_ref]
fn placed_hour()->&'static Mutable<usize>{
    Mutable::new(0)
}

#[static_ref]
fn hour()->&'static Mutable<usize>{
    Mutable::new(3)
}
#[static_ref]
fn depth()->&'static Mutable<usize>{
    Mutable::new(4)
}
#[static_ref]
fn depth2()->&'static Mutable<usize>{
    Mutable::new(6)
}
#[static_ref]
fn teachers_acts()->&'static Mutable<HashMap<i32, Vec<FullActivity>>>{
    Mutable::new(HashMap::new())
}
#[static_ref]
fn data()->&'static Mutable<TimetableData>{
    use super::*;
    Mutable::new(create_data())
}

fn set_data(){
    let tat = data().get_cloned().clean_tat;
    let cat = data().get_cloned().clean_cat;
    let dt = data().get_cloned();
    teachers_limitations().set(*tat);
    classes_limitations().set(*cat);
    schedules().lock_mut().replace_cloned(vec![]);
    //total_hour().set(0);
    data().set(create_data());
}
fn create_data()->TimetableData{
    create_acts_data();
    let tat = teachers_limitations().get_cloned();
    let cat = classes_limitations().get_cloned();
    let acts = activities().lock_mut().to_vec().into_iter().map(|a| Activity{
        id: a.id,
        subject: a.subject,
        classes: a.classes,
        teachers: a.teachers,
        hour: a.hour
    }).collect::<Vec<Activity>>();
    let dt = TimetableData{
        tat: Box::new(tat.clone()),
        cat: Box::new(cat.clone()),
        clean_cat: Box::new(cat),
        clean_tat: Box::new(tat),
        acts,
        teachers_acts: teachers_acts().get_cloned(),
        neighbour_acts: create_ng(),
        classes: super::classes::classes().lock_mut().to_vec(),
        teachers: super::teachers::teachers().lock_mut().to_vec(),
        timetables: Box::new(schedules().lock_mut().to_vec())
    };
    dt
}
fn create_acts_data(){
    let activities = activities().lock_mut().to_vec();
    let acts = activities.clone();
    for act in &activities{
        let acts: Vec<FullActivity> = acts.iter().cloned()
            .filter(|a| act.teachers.iter().all(|t| a.teachers.iter().any(|t2| t2 == t))  &&
                act.classes.iter().all(|c| a.classes.iter().any(|c2| c2 == c))
                //&& act.subject == a.subject
            )
            .collect();
        let mut ts_acts = teachers_acts().get_cloned();
        ts_acts.insert(act.id, acts);
        teachers_acts().set(ts_acts);
        /*
        let activities: Vec<Activity> = acts.clone().into_iter()
            .filter(|a| a.id != act.id &&
                (act.classes.iter().any(|c1| a.classes.iter().any(|c2| c1 == c2)) ||
                    act.teachers.iter().any(|t| a.teachers.iter().any(|t2| t2 == t))))
            .collect();
        let mut na:HashMap<i32, Activity> = HashMap::new();
        for a in &activities{
            na.insert(a.id, a.clone());
        }
        model.data.neighbour_acts.insert(act.id, na);
        */
    }
}
fn generate(){
    let params = Params{
        hour: hour().get() as i32,
        depth: depth().get_cloned().clone(),
        depth2: depth2().get_cloned().clone()
    };
    Task::start(async move{
        loop{
            let mut t_data = data().clone().get_cloned();
            let len = t_data.timetables.len();
            if len == total_hour().get(){
                break;
            }
            if t_data.generate(&params){
                schedules().lock_mut().replace_cloned(*t_data.timetables.clone());
                data().set(t_data);
                
            }
            else{
                break
            }
            
            Timer::sleep(100).await;
            
        }
    })
}

fn create_ng()->HashMap<i32,HashMap<i32, Activity>>{
    use super::*;
    let acts = activities().lock_mut().to_vec().into_iter().map(|a| Activity{
        id: a.id,
        subject: a.subject,
        classes: a.classes,
        teachers: a.teachers,
        hour: a.hour
    }).collect::<Vec<Activity>>();
    let mut neighbours: HashMap<i32, HashMap<i32, Activity>> = HashMap::new();
    for a in &acts{
        let ns = acts.clone().into_iter()
        .filter(|a2| 
            a2.id != a.id &&
            (a2
            .classes
            .iter()
            .any(|c| 
                a.classes
                .iter()
                .any(|c2| c2 == c)
            ) ||
            a2
            .teachers
            .iter()
            .any(|t| 
                a.teachers
                .iter()
                .any(|t2| t2 == t)
            ))
        ).collect::<Vec<Activity>>();
        let mut neigh_map: HashMap<i32, Activity> = HashMap::new();
        for n in ns{
            neigh_map.insert(n.id, n);
        }
        neighbours.insert(a.id, neigh_map);
    }
    neighbours
}

/*
fn depth_view()->impl Element{
    Column::new()
    .s(Gap::new().y(10))
    //.item(Label::new().label("Derinlik 1").s(Align::center()))
    .item(
        text_inputs::default()
        .id("depth")
        .s(Align::center())
        .on_change(|s| depth()
            .set(s.parse::<usize>().unwrap())
        ).placeholder(Placeholder::new())
    )
}
fn depth2_view()->impl Element{
    Column::new()
    .s(Gap::new().y(10))
    .item(Label::new().label("Derinlik 2"))
    .item(text_inputs::default().id("depth2").placeholder(Placeholder::new(6)))
}
*/