use shared::msgs::activities::FullActivity;

use genpdf::*;

use crate::app::timetables::{
    selected_timetable_hour, schedules, activities, 
    add_act::{ 
        lecture_name, 
        teachers_full_name
    }, 
    school, classes::classes
};

pub fn prints(){
    let fon_family = crate::fonts::font_family();
    let mut doc = genpdf::Document::new(fon_family);
    // Change the default settings
    doc.set_title(format!("{}", school().get_cloned().unwrap().name));
    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    let mut buf: Vec<u8> = Vec::new();
    print_classes(&mut doc);
    doc.render(&mut buf).expect("Render edilemedi");
    let png_jsarray: zoon::JsValue = js_sys::Uint8Array::from(&buf[..]).into();
    // the buffer has to be an array of arrays
    let png_buffer: js_sys::Array = std::iter::IntoIterator::into_iter([png_jsarray]).collect();
    let mut properties = web_sys::BlobPropertyBag::new();
    properties.type_("application/pdf");
    let png_blob =
        web_sys::Blob::new_with_buffer_source_sequence_and_options(&png_buffer, &properties)
            .unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&png_blob).unwrap();
    let window = web_sys::window().unwrap();
    window.open_with_url(&url).expect("Pdf açılamadı");
}

fn print_classes(doc: &mut genpdf::Document){
    let school_name = school().get_cloned().unwrap().name;
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10 as i32);

    doc.set_page_decorator(decorator);
    doc.set_line_spacing(1.25);
    let clss = classes().lock_mut().to_vec();
    for class in clss{
        let mut title = elements::LinearLayout::vertical();

        let mut title_style = style::Style::new();
        title_style.set_bold();
        title_style.set_font_size(20);

        let title_paragraph = elements::Paragraph::default();
        title.push(title_paragraph.styled_string(&school_name, title_style).aligned(Alignment::Center));
        let cls_name = format!("{}{}", class.kademe, class.sube);
        let mut teacher_style = style::Style::new();
        teacher_style.set_italic();
        teacher_style.set_font_size(18);
        title.push(elements::Paragraph::new(&cls_name).aligned(Alignment::Center));
        doc.push(title);
        //doc.push(elements::Break::new(1));
        add_row(doc, class.id);
        doc.push(elements::PageBreak::new());
    }
}
use crate::i18n::t_s;
fn add_row(doc:&mut genpdf::Document, c: i32){
    let group_hour: Vec<usize> = selected_timetable_hour().lock_mut().to_vec().into_iter().map(|a| a as usize).collect();
    let len = group_hour.len();
    let mut table = elements::TableLayout::new(vec![8, 8, 8, 8, 8, 8, 8, 8]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    let mut row = table.row();
    row.push_element(create_row_title(format!("{} {}", t_s!("days"), t_s!("hours"))).aligned(Alignment::Center));
    for day in &["p","s","ç","p","c","ct","p"]{
        row.push_element(create_row_title(format!("{}",day)).aligned(Alignment::Center));
    }
    row.push().expect("Invalid table row");
    let sch = schedules().lock_mut().to_vec();
    let acts = activities().lock_mut().to_vec();
    let acts: Vec<FullActivity> = acts.into_iter().filter(|a| a.classes.iter().any(|c2| c2 == &c)).collect();
    for h in 0..len{
        let mut row = table.row();
        let mut hour_row = elements::LinearLayout::vertical();
        hour_row.push(
            create_row_title((h+1).to_string()+". Ders")
            .aligned(Alignment::Center)
        );
        let mut line_style = style::Style::new();
        line_style.bold();
        line_style.set_font_size(8);
        let par = elements::Paragraph::default().styled_string("00:00-00:00", line_style).aligned(Alignment::Center);
        hour_row.push(par);
        row.push_element(hour_row);
        for day in ["p","s","ç","p","c","ct","p"].iter().enumerate(){
            let sch = sch.iter().find(|s| s.day_id == (day.0+1) as i32 && s.hour == h as i32 && acts.iter().any(|a| a.id == s.activity));
            match sch{
                Some(s) => {
                    let act = acts.iter().find(|a| a.id == s.activity).unwrap();
                    let c_name = teachers_full_name(act.clone());
                    let mut sch_row = elements::LinearLayout::vertical();
                    sch_row.push(
                        create_row_title(c_name)
                        .aligned(Alignment::Center)
                    );
                    let mut line_style = style::Style::new();
                    line_style.bold();
                    line_style.set_font_size(8);
                    line_style.set_line_spacing(0.2);
                    let par = elements::Paragraph::default().styled_string(lecture_name(act.clone()), line_style).aligned(Alignment::Center);
                    sch_row.push(par);
                    row.push_element(sch_row);
                }
                None => row.push_element(create_row_title("".to_string()))
            }
            
        }
        row.push().unwrap();
    }
    doc.push(table);
}

fn create_row_title(title: String) -> elements::Paragraph{
    let paragraph = elements::Paragraph::default();
    let mut title_style = style::Style::new();
    title_style.bold();
    title_style.set_font_size(10);
    paragraph.styled_string(title, title_style)
}
