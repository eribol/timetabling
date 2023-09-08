use zoon::{named_color::*,*};
use shared::msgs::activities::FullActivity;
use crate::app::timetables::add_act::{home, teachers_full_name, add_act, change_add_act, ActCol, lecture_name, teacher_short_name, del_act};
use crate::app::timetables::{activities, schedules};
use crate::i18n::t_s;

pub fn activities_view(id: i32)->impl Element{
    Column::new()
    .s(Align::new().top())
    .s(Width::fill())
    .item_signal(
        add_act().signal().map_true(||
            home()
    ))
    .item(
        Button::new()
        .label_signal(add_act().signal().map_bool(|| t_s!("hide"), || t_s!("show")))
        .on_click(change_add_act)
    )
    .s(Padding::new().left(20))
    .item(
            Row::new()
            .multiline()
            .s(Gap::new().x(2).y(2))
            .items_signal_vec(
                activities()
                .signal_vec_cloned()
                .filter_signal_cloned(move |acts| 
                    Mutable::new(
                        acts.classes.iter()
                        .any(|c| c == &id) && !schedules().lock_ref().iter().any(|s| s.activity == acts.id)
                    )
                    .signal()
                )
                .map(move |act|{
                    let a = ActCol{
                        act: Mutable::new(act.clone()),
                        del: Mutable::new(false)
                    };
                    act_view(a)      
                })
            )
        )
        
}


#[static_ref]
pub fn move_select()->&'static Mutable<Option<FullActivity>>{
    Mutable::new(None)
}

fn act_view(act: ActCol)->impl Element{
    let a_c = act.act.clone();
    let a_c2 = act.act.clone();
    let act_id = a_c.get_cloned().id;
    let del = act.del;
    Column::new()
    .s(Padding::new().top(10))
    .s(Width::growable())
    .s(Height::growable())
    .s(RoundedCorners::all(5))
    .s(
        Borders::all(Border::new().width(1).color(BLUE_3).solid())
    )
    .item(
        Button::new()
        .label(
            lecture_name(a_c.get_cloned().clone())
        )
    ).item(
        Button::new()
        .label(
            teacher_short_name(a_c.get_cloned().clone())
        )
    ).item(
        Button::new()
        .label(
            a_c.get_cloned().hour
        )
    ).update_raw_el(|raw| 
        raw.attr("title", &teachers_full_name(a_c.get_cloned().clone()))
    ).item(
        Row::new()
        .item(
            Label::new()
            .s(Font::new().color(RED_6))
            .label_signal(
                del.signal().map_bool(|| "Emin misin?", || "Sil")
            )
            .s(Cursor::new(CursorIcon::Pointer))
            .s(Align::center())
            .on_click(move|| 
                if del.get(){
                    del_act(a_c.get_cloned().clone())
                }
                else{
                    del.set(true)
                }
            )
        ).item(
                Label::new()
                .s(Font::new().color(RED_6))
                .s(Cursor::new(CursorIcon::Pointer))
                .s(Align::center())
                .on_click(move || 
                    if move_select().get_cloned().is_some() && 
                    move_select().get_cloned().unwrap().id == act_id{
                        move_select().set(None);                        
                    }
                    else{
                        move_select().set(Some(a_c2.get_cloned().clone()));
                    }
                )
                .label_signal(
                    move_select().signal_cloned().map(move |mv|{
                        match mv{
                            Some(m) => {
                                if m.id == act_id{
                                    "Vazgeç"
                                }
                                else{""}
                            },
                            None => "Taşı"
                        }
                    })
                )
        )
    ).update_raw_el(|raw_el| {
            raw_el
                .attr("draggable", "true")
                .event_handler_with_options(
                    EventOptions::new().preventable(),
                    |event: events::DragEnter| {
                        event.stop_propagation();
                        event.prevent_default();
                        drop_zone_active().set_neq(true);
                        print!("enter")
                    },
                )
                .event_handler_with_options(
                    EventOptions::new().preventable(),
                    |event: events::DragOver| {
                        event.stop_propagation();
                        event.prevent_default();
                        event.data_transfer().unwrap_throw().set_drop_effect("id");
                        print!("over");
                    },
                )
                .event_handler_with_options(
                    EventOptions::new().preventable(),
                    |event: events::DragLeave| {
                        event.stop_propagation();
                        event.prevent_default();
                        //drop_zone_active().set_neq(false);
                        print!("leave");
                    },
                )
                .event_handler_with_options(
                    EventOptions::new().preventable(),
                    |event: events::Drop| {
                        event.stop_propagation();
                        event.prevent_default();
                        drop_zone_active().set_neq(false);
                        let _file_list = event.data_transfer().unwrap_throw().files().unwrap_throw();
                        print!("drop")
                    },
                )
        })
}

#[static_ref]
pub fn is_placed(act: i32)-> &'static Mutable<bool>{
    let schs = schedules().lock_mut().to_vec();
    let p = schs.iter().any(|s| s.activity == act);
    Mutable::new(p)
}

#[static_ref]
fn drop_zone_active() -> &'static Mutable<bool> {
    Mutable::new(false)
}