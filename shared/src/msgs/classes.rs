use moonlight::*;

use super::activities::{Activity, FullActivity};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum ClassUpMsgs {
    UpdateLimitations((i32, Vec<ClassLimitation>)),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum ClassDownMsgs {
    UpdateClassLimitations(Vec<ClassLimitation>),
    UpdateClassLimitationsError(String),
}

pub type UserId = EntityId;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "serde")]
pub struct Class {
    pub id: i32,
    pub kademe: String,
    pub sube: String,
    pub group_id: i32,
}

impl Class{
    pub fn label(&self)->String{
        let label = self.kademe.clone() + &self.sube;
        label
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "serde")]
pub struct ClassLimitation {
    pub class_id: i32,
    pub day: i32,
    pub hours: Vec<bool>,
}