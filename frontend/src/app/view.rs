use super::{*, login::login_page};
use super::timetables::pages_view;
use crate::app::timetables::timetable_tabs;

pub fn root() -> impl Element {
    Column::new()
    .item_signal(pages().signal_cloned().map(|page|{
        match page{
            Pages::Home => El::new().child(home()),
            Pages::NotReady => El::new().child(login_page()),
        }
    }))
}

fn home() -> impl Element {
    Column::new()
    .item(timetable_tabs())
    .item(pages_view())
}
