use zoon::{named_color::*,*};
use shared::msgs::activities::FullActivity;
use crate::app::timetables::{activities, schedules};

use crate::i18n::t_s;
use super::super::timetables::add_act::*;

pub fn activities_view(id: i32)->impl Element{
    Column::new()
        //.s(Align::new().right())
        .s(Align::new().top())
        .s(Width::fill())
        .item(    
            home()
        )
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
                        acts.teachers.iter()
                        .any(|t| t == &id) && !schedules().lock_ref().iter().any(|s| s.activity == acts.id)
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
            classes_full_name(a_c.get_cloned().clone())
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
        )
    )
}