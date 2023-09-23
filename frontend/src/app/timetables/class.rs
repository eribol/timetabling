use shared::msgs::classes::Class;
use zoon::{*, named_color::BLUE_3};

use self::limitations::{loaded_lims, show_lim_view};
pub mod activities;
pub mod limitations;

pub fn home(id: i32) -> impl zoon::Element {
    zoon::Column::new()
    .s(Padding::new().top(20))
    .s(Height::exact(1080))
    .item(Row::new()
        .s(Padding::new().left(20).right(20))
        .item_signal(loaded_lims().signal().map_true(||
            limitations::limitations_view()
        ))
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
            .on_click(|| limitations::change_view())
        )
        .item(activities::activities_view(id))
        .s(Gap::new().x(10))
    )
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
