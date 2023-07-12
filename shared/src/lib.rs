use moonlight::*;
use msgs::{
    classes::*,
    teachers::*, 
    lectures::*, 
    timetables::*,
    schools::*,
    users::*, activities::{ActivityUpMsgs, ActivityDownMsgs},
};
pub mod msgs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
#[repr(u8)]
pub enum UpMsg {
    Login { email: String, password: String },
    Logout,
    GetSchool,
    GetTeachers,
    GetLectures,
    GetTimetables,
    Timetable(TimetableUpMsgs),
    Activity(ActivityUpMsgs),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
#[repr(u8)]
pub enum DownMsg {
    // ------ Auth ------
    LoggedUser(User),
    AuthError(String),
    LoggedOut,
    LoggedOutError(String),
    GetSchool { id: i32, name: String },
    GetSchoolError(String),
    GetTimetables(Vec<Timetable>),
    GetTeachers(Vec<Teacher>),
    GetLectures(Vec<Lecture>),
    GetClassesLimitationsError,
    GetClassesLimitations(Vec<ClassLimitation>),
    Timetable(TimetableDownMsgs),
    Activity(ActivityDownMsgs),
    Empty
}
