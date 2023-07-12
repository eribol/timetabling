use moonlight::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum ActivityUpMsgs {
    AddAct((i32, AddActivity)),
    DelAct((i32, FullActivity))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum ActivityDownMsgs {
    AddedAct(FullActivity),
    AddActError(String),
    DeletedAct(i32),
    DeleteActError(String)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "serde")]
pub struct Activity{
    pub id: i32,
    pub subject: i32,
    pub classes: Vec<i32> ,
    pub teachers: Vec<i32>,
    pub hour: i16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct FullActivity {
    pub id: i32,
    pub subject: i32,
    pub hour: i16,
    pub classes: Vec<i32>,
    pub teachers: Vec<i32>,
    pub blocks: Option<String>,
    pub partner_activity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct AddActivity {
    pub subject: i32,
    pub hour: i16,
    pub classes: Vec<i32>,
    pub teachers: Vec<i32>,
    //pub blocks: Option<String>,
    //pub partner_activity: Option<i32>,
}