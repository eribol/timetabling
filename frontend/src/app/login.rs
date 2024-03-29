use super::login_user;
use crate::i18n;
use shared::UpMsg;
use std::borrow::Cow;
use zoon::named_color::{BLUE_5, RED_5};
use zoon::{eprintln, *};

#[static_ref]
fn email() -> &'static Mutable<String> {
    Mutable::new(String::from(""))
}

#[static_ref]
fn password() -> &'static Mutable<String> {
    Mutable::new(String::from(""))
}

#[static_ref]
pub fn login_error() -> &'static Mutable<Option<Cow<'static, str>>> {
    Mutable::new(None)
}

pub fn set_login_error(error: String) {
    login_error().set(Some(Cow::from(error)))
}

fn set_email(e: String) {
    email().set(e)
}

fn set_password(p: String) {
    password().set(p)
}
pub fn set_and_store_logged_user(user: shared::msgs::users::User) {
    if let Err(error) = local_storage().insert("user", &user) {
        return set_login_error(error.to_string());
    }
    password().take();
    login_user().set(Some(user));
    crate::router::router()
        .go(crate::router::previous_route().unwrap_or(crate::router::Route::Home));
}

pub fn login_page() -> impl Element {
    Column::new()
        .s(Align::new().center_y().center_x())
        .s(Padding::all(150))
        .s(Gap::new().y(15))
        .item(
            Label::new()
                .s(Align::center())
                .label_signal(i18n::t!("login"))
                .s(Font::new().weight(FontWeight::SemiBold)),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .s(Height::exact(30))
                .id("email")
                .input_type(InputType::text())
                .placeholder(Placeholder::with_signal(i18n::t!("email")))
                .on_change(set_email),
        )
        .item(
            TextInput::new()
                .s(Align::center())
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .s(Height::exact(30))
                .id("password")
                .input_type(InputType::password())
                .placeholder(Placeholder::with_signal(i18n::t!("password")))
                .on_change(set_password),
        )
        .item(
            Button::new()
                .s(Height::exact(35))
                .s(RoundedCorners::all(10))
                .s(Borders::all(Border::new().solid().color(BLUE_5)))
                .label(El::new().s(Align::center()).child_signal(i18n::t!("login")))
                .on_click(|| login()),
        ).item_signal(
            login_error().signal_cloned().map_some(|e| 
                Label::new()
                .s(Font::new().weight(FontWeight::Number(10)).color(RED_5))
                .label(e)
            )
        )
}

fn login() {
    use crate::connection::*;
    use shared::*;
    login_error().take();
    Task::start(async {
        let msg = UpMsg::Login {
            email: email().get_cloned(),
            password: password().get_cloned(),
        };
        match connection().send_up_msg(msg).await {
            Err(error) => {
                let error = error.to_string();
                eprintln!("login request failed: {}", error);
                set_login_error(error);
            }
            Ok(_msg) => (),
        }
    });
}

pub fn _get_school() {
    use crate::connection::*;
    Task::start(async {
        let msg = UpMsg::GetSchool;
        match connection().send_up_msg(msg).await {
            Err(error) => {
                eprintln!("login request failed: {}", error);
            }
            Ok(_) => (),
        }
    });
}
