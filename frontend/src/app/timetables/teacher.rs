use zoon::{*, named_color::BLUE_3};

use super::class::limitations::{show_lim_view, change_view};
pub mod activities;
pub mod limitations;

pub fn home(id: i32) -> impl zoon::Element {
    zoon::Column::new()
    .s(zoon::Padding::all(10))
    .item(Row::new()
        .item(limitations::limitations_view())
        .item(Column::new()
            .s(Height::exact(500))
            .s(Align::new().top())
            .s(Borders::all(Border::new().width(1).color(BLUE_3)))
            .item(
                Label::new()
                .s(Align::center())
                //.s(Height::exact(500))
                .label_signal(show_lim_view().signal().map_bool(|| "<<<", || ">>>"))
            )
            .on_click(|| change_view())
        )
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
