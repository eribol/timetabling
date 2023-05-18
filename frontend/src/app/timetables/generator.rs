use std::collections::HashMap;

use shared::msgs::{timetables::*, activities::Activity};
use zoon::{named_color::*, *};

use crate::elements::text_inputs;

use super::activities;

pub fn home() -> impl Element {
    Column::new()
        .s(Padding::new().top(10))
        .s(Gap::new().y(20))
        .s(Align::center())
        .item(generator())
        .item(buttons())
        .item(information())
}

fn generator() -> impl Element {
    Column::new()
    .s(Gap::new().y(10))
    .item(hour_view())
    .item(depth_view())
    .item(depth2_view())
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
fn depth_view()->impl Element{
    Column::new()
    .s(Gap::new().y(10))
    .item(Label::new().label("Derinlik 1").s(Align::center()))
    .item(
        text_inputs::default()
        .id("depth")
        .s(Align::center())
        .on_change(|s| depth()
            .set(s.parse::<usize>().unwrap())
        ).placeholder(Placeholder::new(4))
    )
}
fn depth2_view()->impl Element{
    Column::new()
    .s(Gap::new().y(10))
    .item(Label::new().label("Derinlik 2"))
    .item(text_inputs::default().id("depth2").placeholder(Placeholder::new(6)))
}

fn information()-> impl Element{
    Row::new()
    .item(
        Label::new()
        .label_signal(placed_hour().signal().map(|s| s.to_string()))
    )
    .item(
        Label::new().label(total_hour().get().to_string())
    )
}

fn buttons()-> impl Element{
    Row::new()
    .s(Gap::new().x(10))
    .item(
        Button::new()
        .label("Dağıt")
        .on_press(|| generate() )
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
fn create_data()->&'static Mutable<TimetableData>{
    use super::*;
    let tat = teachers_limitations().get_cloned();
    let cat = classes_limitations().get_cloned();
    let acts = activities().lock_mut().to_vec().into_iter().map(|a| Activity{
        id: a.id,
        subject: a.subject,
        classes: a.classes,
        teachers: a.teachers,
        hour: a.hour
    }).collect::<Vec<Activity>>();
    let data = TimetableData{
        tat: Box::new(tat.clone()),
        cat: Box::new(cat.clone()),
        clean_cat: Box::new(cat),
        clean_tat: Box::new(tat),
        acts,
        teachers_acts: HashMap::new(),
        neighbour_acts: create_ng(),
        classes: vec![],//super::classes::classes().lock_mut().to_vec(),
        teachers: super::teachers::teachers().lock_mut().to_vec(),
        timetables: Box::new(vec![])
    };
    Mutable::new(data)
}

fn generate(){
    let params = Params{
        hour: hour().get_cloned().clone() as i32,
        depth: depth().get_cloned().clone(),
        depth2: depth2().get_cloned().clone()
    };
    Task::start(async move{
        loop{
            let mut t_data = create_data().clone().get_cloned();
            let len = t_data.timetables.len();
            placed_hour().set(len);
            if len == total_hour().get(){
                break;
            }
            if t_data.generate(&params){
                create_data().set(t_data);
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

