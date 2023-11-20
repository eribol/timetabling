use zoon::*;
use crate::router::Route;

pub fn _root() -> impl Element {
    Row::new()
    .item(_left_nav())
}

fn _left_nav()-> impl Element{
    Row::new()
    .s(Align::new().left())
    .item(
        Link::new().label("Libredu").to(Route::Home)
    ) 
}
