use zoon::*;


mod app;
mod i18n;
mod router;
mod connection;
mod elements;
mod header;
pub mod fonts;

pub static DAYS: [&str; 7] = [
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
    "sunday",
];

fn main() {
    app::load_logged_user();
    let w = window().inner_width().unwrap().as_string();
    if let Some(width) = w {
        app::change_screen_width(width.parse::<u32>().unwrap());
    }
    start_app("app", app::root);
    connection::connection();
    router::router();
}
