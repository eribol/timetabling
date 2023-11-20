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
    GetSchedules(i32),
    DelSchedules(Vec<i32>),
    UpdateSchedules(Vec<Schedule>)
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
    UpdateSchedules(Vec<Schedule>),
    DelSchedules
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
    pub hour: i16,
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
