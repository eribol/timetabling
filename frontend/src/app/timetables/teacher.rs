use zoon::*;
pub mod activities;
pub mod limitations;

pub fn home(id: i32) -> impl zoon::Element {
    zoon::Column::new()
        //.s(zoon::Align::center())
        .s(zoon::Padding::all(10))
        .item(Row::new()
            .item(limitations::schedule_table())
            .item(activities::activities_view(id))
            .s(Gap::new().x(10))
        )
}


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Modal {
    z_index: String,
    background_color: String,
    visibility: String,
    opacity: String,
    transition: String,
}

impl Default for Modal {
    fn default() -> Self {
        Self {
            z_index: "100".to_string(),
            background_color: "#000".to_string(),
            visibility: "hidden".to_string(),
            transition: "".to_string(),
            opacity: "0".to_string(),
        }
    }
}
