use moonlight::*;
use std::collections::{HashMap, HashSet};
use rand::seq::SliceRandom;
use rand::thread_rng;


use super::{activities::{FullActivity, Activity}, classes::{ClassLimitation, ClassUpMsgs, ClassDownMsgs}, teachers::{TeacherLimitation, TeacherUpMsgs, TeacherDownMsgs}};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum TimetableUpMsgs {
    GetClasses(i32),
    GetClassesLimitations(i32),
    GetTeachersLimitations(i32),
    GetActivities(i32),
    Class(ClassUpMsgs),
    Teacher(TeacherUpMsgs),
    //UpdateTeacherLimitations(i32),
    GetSchedules(i32)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum TimetableDownMsgs {
    GetClasses(Vec<super::classes::Class>),
    GetActivities(Vec<FullActivity>),
    GetClassesLimitations(Vec<ClassLimitation>),
    GetClassesLimitationsError,
    Class(ClassDownMsgs),
    Teacher(TeacherDownMsgs),
    GetTeachersLimitations(Vec<TeacherLimitation>),
    GetTeachersLimitationsError,
    GetSchedules(Vec<Schedule>),
    UpdateSchedules(Vec<Schedule>)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Timetable {
    pub id: i32,
    pub name: String,
    pub hour: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Schedule{
    pub day_id: i32,
    pub hour: i32,
    pub activity: i32,
    pub locked: bool,
}


#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(crate = "serde")]
pub struct Params {
    pub hour: i32,
    pub depth: usize,
    pub depth2: usize,
}

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
    pub classes: Vec<super::classes::Class>,
    pub teachers: Vec<crate::Teacher>,
    pub timetables: Box<Vec<Schedule>>,
}

impl Schedule{
    pub fn get_activity(&self, acts: &Vec<Activity>) -> Activity {
        acts.into_iter()
            .find(|a| a.id == self.activity)
            .unwrap()
            .clone()
    }
}

impl TimetableData {
    pub fn generate(&mut self, params: &Params) -> bool {
        let mut acts = self.not_placed_acts();
        if acts.len() == 0 {
            return false;
        }
        acts.sort_by(|a, b| b.hour.cmp(&a.hour));
        let act = &acts[0];
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
                    //*error = "Sınıf ile öğretmenin uyumlu uygun saatleri mevcut değil. Kısıtlamaları kontrol edin.".to_string();
                    return false;
                }
                for a in &conflict_acts[0] {
                    self.delete_activity(a);
                }
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
                    .find(|t| t.hour == (hour - 1) as i32 || t.hour == hour as i32 + act.hour as i32);
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
                hour: timetable as i32,
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
                        //log!(cat_index.hours[t.1.hour as usize]);
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
        //total_act.sort_by(|a,b| a.len().cmp(&b.len()));
        total_act.sort_by(|a, b| {
            a.iter()
                .fold(0, |acc, act| acc + act.hour)
                .cmp(&b.iter().fold(0, |acc, act| acc + act.hour))
        });
        for item in &mut total_act {
            item.sort_by_key(|a| a.id);
            item.dedup();
        }
        //log!("elapsed3 = ", depth);
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
        //conflict_acts.shuffle(&mut thread_rng());
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
