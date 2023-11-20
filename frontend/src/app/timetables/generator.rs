use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::thread_rng;
use shared::msgs::activities::FullActivity;
use shared::msgs::classes::{ClassLimitation, Class};
use shared::msgs::teachers::{TeacherLimitation, Teacher};
use shared::msgs::{timetables::*, activities::Activity};
use zoon::*;
use zoon::named_color::{BLUE_3, BLUE_1};
use crate::app::timetables::classes::classes;
use crate::connection::send_msg;
use crate::i18n::t;
use crate::elements::{text_inputs, buttons};


use super::teachers::teachers;
use super::{schedules, activities, teachers_limitations, classes_limitations, prints};

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
        buttons::default_with_signal(t!("save")).on_click(|| save_schedules())
    ).item(
        buttons::_default("Yazdır").on_click(|| prints::teachers::prints())
    ).item(
        buttons::_default("Yazdır-Sınıflar").on_click(|| prints::classes::prints())
    )
    .item(
        buttons::_default("Çarşaf Sınıflar").on_click(|| prints::all_classes::print_class_all())
    )
    .item(
        buttons::_default("Çarşaf Öğretmenler").on_click(|| prints::all_teachers::print_teacher_all())
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
            Label::new().label_signal(
                total_hour().signal()
            )
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
    let (a, _b) = Mutable::new_and_signal_cloned(false);
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
        Button::new()
        .s(Borders::all_signal(a.signal().map_bool(
            || Border::new().width(1).color(BLUE_3).solid(),
            || Border::new().width(1).color(BLUE_1).solid(),
        )))
        .s(Height::exact(50))
        .s(RoundedCorners::all(2))
        .label_signal(
            is_generate()
            .signal()
            .map_bool(|| 
                Label::new()
                .s(Align::center())
                .label_signal(t!("generate")), 
                || Label::new().s(Align::center()).label_signal(t!("stop"))))
        .s(Width::growable())
        .on_press(||{
            is_generate().set(!is_generate().get());
            generate();  
        })
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
fn save_schedules(){
    let schedules = schedules().lock_mut().to_vec();
    let acts = activities().lock_mut().to_vec();
    let acts = acts.iter().filter(|a| !schedules.iter().any(|s| s.activity == a.id)).map(|a| a.id).collect::<Vec<i32>>();
    let del_msg = TimetableUpMsgs::DelSchedules(acts);
    let update_msg = TimetableUpMsgs::UpdateSchedules(schedules);
    //send_msg(shared::UpMsg::Timetable(del_msg));
    send_msg(shared::UpMsg::Timetable(update_msg));
}
#[static_ref]
fn is_generate()->&'static Mutable<bool>{
    Mutable::new(true)
}

fn set_data(){
    let tat = data().get_cloned().clean_tat;
    let cat = data().get_cloned().clean_cat;
    teachers_limitations().set(*tat);
    classes_limitations().set(*cat);
    schedules().lock_mut().replace_cloned(vec![]);
    //total_hour().set(0);
    data().set(create_data());
}
fn create_data()->TimetableData{
    use zoon::println;
    println!("data");
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
    is_data_ready().set(true);
    dt
}
#[static_ref]
fn is_data_ready()->&'static Mutable<bool>{
    Mutable::new(false)
}
pub fn fix_schedules(){
    if !is_data_ready().get(){
        data();
        //is_generate().set(!is_generate().get());
        return ();
    }
    use zoon::println;
    println!("fix");
    let clss = classes().lock_mut().to_vec();
    let c_acts = activities().lock_mut().to_vec();
    let schdls = schedules().lock_mut().to_vec();
    for c in &clss{
        let c_acts: Vec<&FullActivity> = c_acts.iter().filter(|a| a.classes.iter().any(|c2| c2== &c.id)).collect();
        let c_lim = classes_limitations().lock_mut();
        let c_lim = c_lim.get(&c.id).unwrap();
        for c_l in c_lim{
            for h in c_l.hours.iter().enumerate(){
                if !h.1{
                    let c_sch = schdls.clone().into_iter()
                    .enumerate()
                    .find(|sc| sc.1.day_id == c_l.day && sc.1.hour as usize == h.0 && c_acts.iter().any(|c_a| c_a.id == sc.1.activity));
                    if let Some(cs) = c_sch{
                        let mut dt = data().get_cloned();
                        let acts = dt.acts.clone();
                        let act = acts.iter().find(|a| a.id == cs.1.activity).unwrap();
                        dt.delete_activity(act);
                        //*dt.timetables = schdls.clone();
                        data().set(dt);
                    }
                }
            }
        }
    }
    let tchrs = teachers().lock_mut().to_vec();
    let t_acts = activities().lock_mut().to_vec();
    for t in &tchrs{
        let t_acts: Vec<&FullActivity> = t_acts.iter().filter(|a| a.teachers.iter().any(|c2| c2== &t.id)).collect();
        let t_lim = teachers_limitations().lock_mut();
        let t_lim = t_lim.get(&t.id).unwrap();
        for t_l in t_lim{
            for h in t_l.hours.iter().enumerate(){
                if !h.1{
                    let t_sch = schdls.clone().into_iter()
                    .enumerate()
                    .find(|sc| sc.1.day_id == t_l.day && sc.1.hour as usize == h.0 && t_acts.iter().any(|c_a| c_a.id == sc.1.activity));
                    if let Some(ts) = t_sch{
                        let mut dt = data().get_cloned();
                        let acts = dt.acts.clone();
                        let mut tat = dt.tat.clone().get(&t.id).unwrap().clone();
                        tat[t_l.day as usize].hours[h.0] = false;
                        let act = acts.iter().find(|a| a.id == ts.1.activity).unwrap();
                        dt.delete_activity(act);
                        dt.tat.insert(t.id, tat.clone());
                        data().set(dt);
                    }
                }
                else{
                    let mut dt = data().get_cloned();
                    let mut tat = dt.tat.clone().get(&t.id).unwrap().clone();
                    tat[t_l.day as usize].hours[h.0] = true;
                    dt.tat.insert(t.id, tat.clone());
                    data().set(dt);
                }
            }
        }    
    }
    let s = data().get_cloned().timetables;
    schedules().lock_mut().replace_cloned(*s);
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
    }
}
fn generate(){
    let params = Params{
        hour: hour().get() as i32,
        depth: 8,
        depth2: 6
    };
    fix_schedules();
    if !is_generate().get(){
        Task::start(async move{
            Timer::sleep(1000).await;
            loop{
                let mut t_data = data().clone().get_cloned();
                let len = t_data.timetables.len();
                if len == total_hour().get(){
                    is_generate().set(true);
                    break;
                }
                if t_data.generate(&params) && !is_generate().get(){
                    schedules().lock_mut().replace_cloned(*t_data.timetables.clone());
                    data().set(t_data);
                }
                else{
                    break
                }
                Timer::sleep(20).await;
            }
        })
    }
    
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
#[derive(Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct TimetableData {
    pub tat: Box<HashMap<i32, Vec<TeacherLimitation>>>,
    pub cat: Box<HashMap<i32, Vec<ClassLimitation>>>,
    pub clean_tat: Box<HashMap<i32, Vec<TeacherLimitation>>>,
    pub clean_cat: Box<HashMap<i32, Vec<ClassLimitation>>>,
    pub acts: Vec<Activity>,
    pub teachers_acts: HashMap<i32, Vec<FullActivity>>,
    pub neighbour_acts: HashMap<i32, HashMap<i32, Activity>>,
    pub classes: Vec<Class>,
    pub teachers: Vec<Teacher>,
    pub timetables: Box<Vec<Schedule>>,
}

impl TimetableData {
    pub fn generate(&mut self, params: &Params) -> bool {
        //use zoon::println;
        self.acts.shuffle(&mut thread_rng());
        //self.acts.sort_by(|a, b| b.hour.cmp(&a.hour));
        let acts = self.not_placed_acts();
        if acts.len() == 0 {
            return false;
        }
        //
        let act = &acts[0];
        //println!("{act:?}");
        let available = self.find_timeslot(act, params);
        match available {
            Some(slots) => {
                self.put_act(slots.0, slots.1, act);
                return true;
            }
            None => {
                let timetables_backup = self.timetables.clone();
                let tat_backup = self.tat.clone();
                let cat_backup = self.cat.clone();
                if self.recursive_put(act, 0, &act, params) {
                    return true;
                }
                self.timetables = timetables_backup;
                self.tat = tat_backup;
                self.cat = cat_backup;
                let conflict_acts = self.find_conflict_activity(act, &act, params);
                if conflict_acts.is_empty() {
                    return false;
                }
                //println!("tat:{:?} cat: {:?}", self.tat.get(&act.teachers[0]), self.cat.get(&act.classes[0]));
                for a in &conflict_acts[0] {
                    self.delete_activity(a);
                }
                //sprintln!("Sonrası, tat:{:?} cat: {:?}", self.tat.get(&act.teachers[0]), self.cat.get(&act.classes[0]));
                if let Some(slots) = self.find_timeslot(act, params) {
                   
                    self.put_act(slots.0, slots.1, act);
                }
                for a in &conflict_acts[0] {
                    if let Some(slots) = self.find_timeslot(a, params) {
                        self.put_act(slots.0, slots.1, a);
                    }
                }
                return true;
            }
        }
    }

    fn not_placed_acts(&self) -> Vec<Activity> {
        self.acts
            .clone()
            .into_iter()
            .filter(|a| {
                !a.teachers.is_empty() && !self.timetables.iter().any(|t| a.id == t.activity)
            })
            .collect()
    }
    pub fn find_timeslot(&self, act: &Activity, params: &Params) -> Option<(i32, usize)> {
        let mut days = vec![1, 2, 3, 4, 5, 6, 7];
        if self.tat.len() == 0 {
            return None;
        }
        days.shuffle(&mut thread_rng());
        for day in days {
            for hour in 0..self.tat.get(&act.teachers[0]).unwrap()[0].hours.len() {
                if self.teachers_available(act, hour, day)
                    && self.classes_available(act, hour, day)
                    && self.same_day_available(act, hour, day, params)
                {
                    return Some((day, hour));
                }
            }
        }
        None
    }
    fn same_day_available(&self, act: &Activity, hour: usize, day: i32, params: &Params) -> bool {
        if let Some(teacher_acts) = self.teachers_acts.get(&act.id) {
            let same_day_acts: Vec<Schedule> = self
                .timetables
                .iter()
                .cloned()
                .filter(|t| t.day_id == day && teacher_acts.iter().any(|ta| ta.id == t.activity))
                .collect();
            if same_day_acts.len() == 0 {
                return true;
            }
            else if (act.hour  + same_day_acts.len() as i16) > params.hour as i16 {
                return false;
            }
            else {
                let hours = same_day_acts
                    .iter()
                    .cloned()
                    .find(|t| hour > 0 && t.hour == (hour - 1) as i16 || t.hour == hour as i16 + act.hour);
                if hours.is_some() {
                    return true;
                }
                return false;
            }
        }
        true
    }
    fn classes_available(&self, act: &Activity, hour: usize, day: i32) -> bool {
        let mut classes_availables = vec![];
        for class in &act.classes {
            let class = self.cat.get(class);
            if let Some(c) = class {
                for c2 in c {
                    if c2.day == day {
                        classes_availables.push(c2);
                    }
                }
            }
        }
        (hour..hour + act.hour as usize).all(|h| classes_availables.iter().all(|ca| ca.hours[h]))
    }
    fn teachers_available(&self, act: &Activity, hour: usize, day: i32) -> bool {
        let mut teachers_availables = vec![];
        for teacher in &act.teachers {
            let teacher = self.tat.get(teacher);
            if let Some(t) = teacher {
                for t2 in t {
                    if t2.day == day{
                        teachers_availables.push(t2);
                    }
                }
            }
        }
        hour + act.hour as usize <= teachers_availables[0].hours.len()
            && (hour..hour + act.hour as usize)
                .all(|h| teachers_availables.iter().all(|ta| ta.hours[h]))
    }
    fn put_act(&mut self, day: i32, hour: usize, act: &Activity) {
        for timetable in hour..hour + act.hour as usize {
            let tt = Schedule {
                day_id: day,
                hour: timetable as i16,
                activity: act.id,
                locked: false,
            };
            for teacher in &act.teachers {
                if let Some(tat) = self.tat.get_mut(teacher) {
                    if let Some(tat_index) = tat.iter_mut().find(|t2| t2.day as i32== tt.day_id) {
                        tat_index.hours[tt.hour as usize] = false;
                    }
                }
            }
            for class in &act.classes {
                if let Some(cat) = self.cat.get_mut(class) {
                    if let Some(cat_index) = cat.iter_mut().find(|c2| c2.day == tt.day_id) {
                        //log!(cat_index.hours[t.1.hour as usize]);
                        cat_index.hours[tt.hour as usize] = false;
                    }
                }
            }
            self.timetables.push(tt);
        }
    }
    fn delete_activity(&mut self, act: &Activity) {
        let tt: Vec<(usize, Schedule)> = self
            .timetables
            .iter()
            .cloned()
            .enumerate()
            .filter(|t| t.1.activity == act.id)
            .collect();

        for t in &tt {
            for teacher in &act.teachers {
                if let Some(tat) = self.tat.get_mut(teacher) {
                    if let Some(tat_index) = tat.iter_mut().find(|t2| t2.day as i32== t.1.day_id) {
                        tat_index.hours[t.1.hour as usize] = true;
                    }
                }
            }
            for class in &act.classes {
                if let Some(cat) = self.cat.get_mut(class) {
                    if let Some(cat_index) = cat.iter_mut().find(|c2| c2.day == t.1.day_id) {
                        
                        cat_index.hours[t.1.hour as usize] = true;
                    }
                    
                }
            }
        }
        self.timetables.retain(|t| t.activity != act.id);
    }
    fn find_conflict_activity(
        &self,
        act: &Activity,
        ignores: &Activity,
        params: &Params,
    ) -> Vec<Vec<Activity>> {
        //let now = instant::Instant::now();
        let mut total_act: Vec<Vec<Activity>> = Vec::new();
        let activities = self.neighbour_acts.get(&act.id).unwrap();
        let mut teacher_availables = vec![];
        for teacher in &act.teachers {
            let t_a = self.clean_tat.get(teacher).unwrap();
            for ta in t_a {
                teacher_availables.push(ta);
            }
        }
        for teacher_available in &teacher_availables {
            for h in 0..teacher_available.hours.len() {
                if h + act.hour as usize <= teacher_available.hours.len() {
                    let available = (h..h + act.hour as usize).all(|h| teacher_available.hours[h]);
                    if available {
                        let mut less_conflict: Vec<Activity> = Vec::new();
                        for i in h..h + act.hour as usize {
                            let conflict_slot: Vec<Schedule> = self
                                .timetables
                                .to_owned()
                                .into_iter()
                                .filter(|t| {
                                    t.day_id == teacher_available.day as i32
                                        && t.hour as usize == i
                                        && ignores.id != t.activity
                                        && activities.get(&t.activity).is_some()
                                })
                                .collect();
                            for c in &conflict_slot {
                                let activity = activities.get(&c.activity);
                                if let Some(a) = activity {
                                    let b = a.clone();
                                    less_conflict.push(b.to_owned());
                                }
                            }
                        }
                        if less_conflict.len() > 0 {
                            //if !less_conflict.iter().any(|i| i.id == ignores.id) {
                            total_act.push(less_conflict);
                            //}
                        }
                    }
                }
            }
        }
        //log!("elapsed2 = ", now.elapsed().as_millis());
        total_act.shuffle(&mut thread_rng());
        
        for item in &mut total_act {
            item.sort_by_key(|a| a.id);
            item.dedup();
        }
        if total_act.len() >= params.depth {
            return total_act[..params.depth].to_vec();
        }
        total_act
    }
    pub(crate) fn recursive_put(
        &mut self,
        act: &Activity,
        depth: usize,
        ignores: &Activity,
        params: &Params,
    ) -> bool {
        let mut conflict_acts = self.find_conflict_activity(act, ignores, params);
        //let start = Instant::now();
        let mut okey2 = false;
        conflict_acts.shuffle(&mut thread_rng());
        let tat2 = self.tat.clone();
        let cat2 = self.cat.clone();
        let timetables2 = self.timetables.clone();
        for c_act in &mut conflict_acts {
            for a in &*c_act {
                self.delete_activity(a);
            }
            //let mut c_act2: Vec<Activity> = Vec::new();
            c_act.shuffle(&mut thread_rng());
            c_act.sort_by(|a, b| a.hour.cmp(&b.hour));
            c_act.push(act.clone());
            //ignore_list.append(&mut c_act.clone());
            let mut okey = true;
            for a in c_act.iter().rev() {
                let available = self.find_timeslot(a, params);
                match available {
                    Some(slots) => {
                        self.put_act(slots.0, slots.1, a);
                    }
                    None => {
                        if depth < params.depth2 {
                            let rec_result = self.recursive_put(a, depth + 1, act, params);
                            if !rec_result {
                                okey = false;
                                break;
                            }
                        } else {
                            okey = false;
                            break;
                        }
                    }
                }
            }
            if okey {
                okey2 = true;
                //ignore_list.retain(|a3| a3.id != act.id);
                break;
            } else {
                self.tat = tat2.to_owned();
                self.cat = cat2.to_owned();
                self.timetables = timetables2.to_owned();
            }
        }
        okey2
    }
}