use shared::msgs::classes::Class;
use zoon::*;
pub mod activities;
pub mod limitations;

pub fn home(id: i32) -> impl zoon::Element {
    zoon::Column::new()
    .s(Padding::new().top(20))
    .item(Row::new()
        .item(limitations::schedule_table())
        .item(activities::activities_view(id))
        .s(Gap::new().x(10))
    )
}

#[static_ref]
pub fn cls_id()-> &'static Mutable<i32>{
    Mutable::new(-1)
}

#[static_ref]
pub fn selected_class()-> &'static Mutable<Option<Class>>{
    Mutable::new(None)
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
