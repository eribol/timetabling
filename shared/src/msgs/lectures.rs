use moonlight::*;


#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum LecturesUpMsg{
    GetLectures,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum LecturesDownMsg{
    GetLectures(Vec<Lecture>),
    GetLectureError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct AddLecture {
    pub name: String,
    //pub kademe: String,
    pub short_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Lecture {
    pub id: i32,
    pub name: String,
    //pub kademe: String,
    pub short_name: String,
}