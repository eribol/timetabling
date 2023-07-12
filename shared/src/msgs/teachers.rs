use moonlight::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum TeacherUpMsgs {
    UpdateLimitations((i32, Vec<TeacherLimitation>)),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum TeacherDownMsgs {
    UpdateLimitations(Vec<TeacherLimitation>),
    UpdateLimitationsError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct Teacher {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub short_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct TeacherLimitation {
    pub user_id: i32,
    pub school_id: i32,
    pub group_id: i32,
    pub day: i32,
    pub hours: Vec<bool>,
}

impl Teacher{
    pub fn label(&self)-> String{
        if self.short_name.len() == 0 as usize{
            self.label_full()
        }
        else{
            self.short_name.clone()
        }
    }
    pub fn label_full(&self)-> String{
        self.first_name.clone() + &self.last_name
    }
}