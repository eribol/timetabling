use shared::msgs::{teachers, activities::FullActivity};
use web_sys::window;
use genpdf::*;
use zoon::paragraph;

use crate::app::timetables::{selected_timetable_hour, schedules, activities, teachers::teachers, add_act::{classes_full_name, lecture_name, teachers_full_name}};

pub fn prints(){
    let fon_family = crate::fonts::font_family();
    let mut doc = genpdf::Document::new(fon_family);
    // Change the default settings
    doc.set_title("Demo document");
    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    println!("a3");
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    let mut buf: Vec<u8> = Vec::new();
    print_teachers(&mut doc);
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

fn print_teachers(doc: &mut genpdf::Document){
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10 as i32);

    doc.set_page_decorator(decorator);
    doc.set_line_spacing(1.25);
    let teachers = teachers().lock_mut().to_vec();
    for teacher in teachers{
        let mut title = elements::LinearLayout::vertical();

        let mut title_style = style::Style::new();
        title_style.set_bold();
        title_style.set_font_size(20);

        let title_paragraph = elements::Paragraph::default();
        title.push(title_paragraph.styled_string("Okul Adı", title_style).aligned(Alignment::Center));
        let teacher_name = "A";
        let teacher_name = teacher_name.replace("\u{2068}", "");
        let teacher_name = teacher_name.replace("\u{2069}", "");
        let mut teacher_style = style::Style::new();
        teacher_style.set_italic();
        teacher_style.set_font_size(18);
        title.push(elements::Paragraph::new(&teacher_name).aligned(Alignment::Center));
        doc.push(title);
        //doc.push(elements::Break::new(1));
        add_row(doc, teacher.id);
        doc.push(elements::PageBreak::new());
    }
}

fn add_row(doc:&mut genpdf::Document, t: i32){
    let group_hour: Vec<usize> = selected_timetable_hour().lock_mut().to_vec().into_iter().map(|a| a as usize).collect();
    let len = group_hour.len();
    let mut table = elements::TableLayout::new(vec![8, 8, 8, 8, 8, 8, 8, 8]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    let mut row = table.row();
    row.push_element(create_row_title(format!("{} {}", "Günler", "Saatler")).aligned(Alignment::Center));
    for day in &["p","s","ç","p","c","ct","p"]{
        row.push_element(create_row_title(format!("{}",day)).aligned(Alignment::Center));
    }
    row.push().expect("Invalid table row");
    let sch = schedules().lock_mut().to_vec();
    let acts = activities().lock_mut().to_vec();
    let acts: Vec<FullActivity> = acts.into_iter().filter(|a| a.teachers.iter().any(|t2| t2 == &t)).collect();
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
