use crate::connection::send_msg;
use shared::{
    msgs::teachers::*,   
    UpMsg,
};
use zoon::{named_color::*, *};

use super::{teacher::{self, limitations::get_t_l}, add_act::teacher_short_name};

#[static_ref]
fn show_teachers()->&'static Mutable<bool>{
    Mutable::new(true)
}

#[static_ref]
pub fn selected_teacher()->&'static Mutable<Option<i32>>{
    Mutable::new(None)
}
pub fn home() -> impl Element {
    Column::new()
    .item(teachers_view())
    .item(
        show_button()
    ).item_signal(selected_teacher().signal().map_some(|id|{
        teacher::home(id)
    }))
}

fn show_button()->impl Element{
    Button::new().label_signal(
        show_teachers().signal().map_bool(||"Gizle", || "Tümünü Göster")
    )
    .on_click(||{
        let s = show_teachers().get();
        show_teachers().set(!s);            
    })
}

fn teachers_view()-> impl Element{
    Row::new()
    .multiline_signal(show_teachers().signal())
    .s(Padding::new().x(50))
    .s(Gap::new().x(10).y(10))
    .items_signal_vec(teachers().signal_vec_cloned().map(|row| {
        let a = Mutable::new(false);
        Column::new()
        .s(Borders::all_signal(a.signal().map_bool(
            || Border::new().width(1).color(BLUE_3).solid(),
            || Border::new().width(1).color(BLUE_1).solid(),
        )))
        .s(RoundedCorners::all(2))
        .s(Width::exact(150))
        .s(Height::exact(50))
        .s(Cursor::new(CursorIcon::Pointer))
        .on_hovered_change(move |b| a.set(b))
        .item(
            Button::new()
            .s(Align::center())
            .s(
                Font::new().weight_signal(selected_teacher().signal().map_some(move |id|{
                    if id == row.id{
                        FontWeight::Bold
                    }
                    else{
                        FontWeight::Light
                    }
                })).color_signal(selected_teacher().signal().map_some(move |id|{
                    if id == row.id{
                        RED_7
                    }
                    else{
                        BLUE_5
                    }
                }))
            )
            .label(format!("{}", teacher_name(row.clone())))
        )
        .on_click(move ||{
            show_teachers().set(false);
            selected_teacher().set(Some(row.id));
            get_t_l();
            super::add_act::change_act_teachers();
        })
        .update_raw_el(|raw_el|{
            raw_el.attr("title", &format!("{} {}", row.first_name, row.last_name))
        })        
    }))
}

#[static_ref]
pub fn teachers() -> &'static MutableVec<Teacher> {
    MutableVec::new_with_values(vec![])
}

pub fn get_teachers() {
    send_msg(UpMsg::GetTeachers)
}

fn teacher_name(teacher: Teacher)-> String{
    if teacher.short_name.len()==0{
        return format!("{} {}", teacher.first_name, teacher.last_name);
    }
    teacher.short_name
}
