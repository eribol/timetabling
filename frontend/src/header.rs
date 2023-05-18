use zoon::*;
use crate::{i18n::{t, lang, self}, app, router::Route};

pub fn root() -> impl Element {
    Row::new()
    .item(left_nav())
}

fn left_nav()-> impl Element{
    Row::new()
    .s(Align::new().left())
    .item(
        Link::new().label("Libredu").to(Route::Home)
    ) 
}
