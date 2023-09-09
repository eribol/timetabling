use std::collections::BTreeSet;
use zoon::*;

pub mod timetables;
pub mod view;
pub mod login;
use shared::msgs::users::User;
pub static LANG_STORAGE_KEY: &str = "tr";

pub fn root() -> impl Element {
    Column::new()
    .s(Padding::new().top(15))    
    .item(view::root())
    .on_viewport_size_change(|width, _height| change_screen_width(width))
}

#[derive(Debug, Clone, Default)]
pub enum Pages {
    Home,
    #[default]
    NotReady
}

// -------------------
// ---- States -------
// -------------------
#[static_ref]
pub fn login_user() -> &'static Mutable<Option<User>> {
    Mutable::new(None)
}

#[static_ref]
pub fn pages() -> &'static Mutable<Pages> {
    Mutable::new(Pages::NotReady)
}

#[static_ref]
pub fn unfinished_mutations() -> &'static Mutable<BTreeSet<CorId>> {
    Mutable::new(BTreeSet::new())
}

#[static_ref]
pub fn screen_width() -> &'static Mutable<u32> {
    //web_sys::Window::inner_width(&self)
    Mutable::new(0)
}

pub fn change_screen_width(w: u32) {
    screen_width().set(w);
}

pub async fn auth_token() -> Option<AuthToken> {
    Some(login_user().lock_ref().as_ref()?.auth_token.clone())
}
///-----------
// Functions
///-----------

pub fn set_page_id(page: Pages) {
    pages().set(page)
}

pub fn load_logged_user() {
    if let Some(Ok(user)) = local_storage().get("user") {
        login_user().set(Some(user));
        self::timetables::get_school();
    }
}

trait ModalElement{
    type Item;
    fn into_iter(self) -> &'static dyn Element<Item = Self::Item, IntoIter = Self::Item>;
}
