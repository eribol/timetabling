use std::collections::HashMap;

use shared::msgs::classes::*;
use shared::msgs::timetables::TimetableUpMsgs;
use zoon::{named_color::*, *};
use crate::app::timetables::{ selected_timetable};

use super::{TimetablePages, change_page, class::{selected_class, limitations::create_class_lims, cls_id}, add_act};

fn is_selected(id: i32)->impl Signal<Item = bool>{
    if id == cls_id().get(){
        let a = Mutable::new_and_signal(true);
        return a.1
    }
    Mutable::new_and_signal(false).1
}
pub fn classes_view() -> impl Element {
    Column::new()
        .s(Gap::both(10))
        .s(Width::fill())
        .s(Align::center())
        .item_signal(classes()
            .signal_cloned().map(|col| {
                Row::new()
                //.s(Width::fill())
                .s(Align::center())
                .s(Gap::new().x(2))
                .s(Cursor::new(CursorIcon::Pointer))
                .multiline()
                .items(col.into_iter().map(|row| {
                    let a = Mutable::new(false);
                    Column::new()
                        .s(Borders::all_signal(a.signal().map_bool(
                            || Border::new().width(1).color(BLUE_3).solid(),
                            || Border::new().width(1).color(BLUE_1).solid(),
                        )))
                        .s(RoundedCorners::all(2))
                        .s(Width::exact(75))
                        .s(Height::exact(50))
                        .on_hovered_change(move |b| a.set(b))
                        .item(
                            Label::new()
                            .s(Cursor::new(CursorIcon::Pointer))
                            .s(Align::new().center_x().center_y())
                            .s(
                                Font::new()
                                .weight_signal(
                                    cls_id().signal_ref(move |id|{
                                        if id == &row.1.id{
                                            FontWeight::Bold
                                        }
                                        else{
                                            FontWeight::Light
                                        }
                                    })
                                    //.map_bool(|| , || FontWeight::Medium)
                                )
                                .color_signal(
                                    cls_id().signal_ref(move |id|{
                                        if id == &row.1.id{
                                            RED_7
                                        }
                                        else{
                                            BLUE_5
                                        }
                                    })
                                )
                            )
                            .label(format!("{}{}", row.1.kademe, row.1.sube))
                    )
                    .on_click(move || {
                        cls_id().set(row.1.id);
                        create_class_lims();
                        let clss = classes().lock_mut();
                        let cls = clss.get(&row.1.id).unwrap();
                        selected_class().set(Some(cls.clone()));
                        add_act::change_act_classes();
                    })
            }))
        }))
        .item_signal(selected_class().signal_cloned().map_some(|cl| {
            super::class::home(cl.id)
        }))
}

#[static_ref]
pub fn classes() -> &'static Mutable<HashMap<i32, Class>> {
    Mutable::new(HashMap::new())
}
pub fn get_classes() {
    use crate::connection::*;
    use shared::*;
    let group_id = selected_timetable().get();
    let msg = UpMsg::Timetable(TimetableUpMsgs::GetClasses(group_id));
    send_msg(msg)
}