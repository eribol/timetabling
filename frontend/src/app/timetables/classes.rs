use shared::msgs::classes::*;
use shared::msgs::timetables::TimetableUpMsgs;
use zoon::{named_color::*, *};
use crate::app::timetables::selected_timetable;

use super::{class::{selected_class, limitations::create_class_lims}, add_act};

pub fn classes_page_view()-> impl Element{
    Column::new()
    .item(classes_view())
    .item_signal(selected_class().signal_cloned().map_some(|cl| {
        super::class::home(cl.id)
    }))
}
fn classes_view() -> impl Element {
    Row::new()
    .s(Align::center())
    .s(Gap::new().x(2))
    .s(Cursor::new(CursorIcon::Pointer))
    .multiline()
    .items_signal_vec(classes().signal_vec_cloned().map(|row| {
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
                        selected_class().signal_cloned()
                        .map_option(move|class|{
                            if class.id == row.id{
                                FontWeight::Bold
                            }
                            else{
                                FontWeight::Light
                            }
                        }, ||FontWeight::Light)   
                    )
                    .color_signal(
                        selected_class().signal_cloned().map_option(move |class|{
                            if class.id == row.id{
                                RED_7
                            }
                            else{
                                BLUE_5
                            }
                        }, || BLUE_5)
                    )
                )
                .label(format!("{}{}", row.kademe, row.sube))
            )
            .on_click(move || {
                let class = selected_class().get_cloned();
                match class{
                    Some(c) =>{
                        if c.id != row.id{
                            let clss = classes().lock_mut().to_vec();
                            let cls = clss.iter().find(|c| c.id == row.id).unwrap();
                            selected_class().set(Some(cls.clone()));
                            add_act::change_act_classes();
                            create_class_lims();
                        }
                        else{
                            selected_class().set(None)
                        }
                    }
                    None => {
                        let clss = classes().lock_mut().to_vec();
                        let cls = clss.iter().find(|c| c.id == row.id).unwrap();
                        selected_class().set(Some(cls.clone()));
                        add_act::change_act_classes();
                        create_class_lims();
                    }
                }
                
            })
        })
    )
}

#[static_ref]
pub fn classes() -> &'static MutableVec<Class> {
    MutableVec::new_with_values(vec![])
}
pub fn get_classes() {
    use crate::connection::*;
    use shared::*;
    let group_id = selected_timetable().get();
    let msg = UpMsg::Timetable(TimetableUpMsgs::GetClasses(group_id));
    send_msg(msg)
}