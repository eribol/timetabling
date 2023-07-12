use moonlight::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "serde")]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub auth_token: AuthToken,
}