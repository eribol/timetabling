use genpdf::elements::TableLayout;
use genpdf::{elements, style, Alignment, Document, Size, PaperSize, Element};
use shared::msgs::activities::FullActivity;
use shared::msgs::classes::Class;
use shared::msgs::timetables::Schedule;

use crate::DAYS;
use crate::app::timetables::{selected_timetable_hour, schedules, activities, lectures};
use crate::app::timetables::{school, classes::classes};


pub fn print_class_all(){
    let fon_family = crate::fonts::font_family();
    let mut doc = genpdf::Document::new(fon_family);
    let mut buf: Vec<u8> = Vec::new();
    //print_classes(&mut doc);
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
    doc.set_paper_size(Size::new(297,210));
    doc.set_title(format!("{}", school().get_cloned().unwrap().name));
    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    let school_name = school().get_cloned().unwrap().name;
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);
    let mut title = elements::LinearLayout::vertical();
    let mut title_style = style::Style::new();
    title_style.set_bold();
    title_style.set_font_size(20);
    let title_paragraph = elements::Paragraph::default();
    title.push(title_paragraph.styled_string(&school_name, title_style).aligned(Alignment::Center));
    doc.push(title);
    day_row(doc);
}
use crate::i18n::t_s;
fn day_row(doc: &mut Document){
    let mut table = elements::TableLayout::new(vec![8, 8, 8, 8, 8, 8]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    let mut row = table.row();
    row.push_element(create_row_title(format!("{}", t_s!("days"))).aligned(Alignment::Center));
    for day in &["Pzt","Salı","Çarş","Per","Cuma"]{
        row.push_element(create_row_title(format!("{}",day)).aligned(Alignment::Center));
    }
    row.push().expect("Invalid table row");
    let mut row = table.row();
    row.push_element(create_row_title(format!("{}", t_s!("hours"))).aligned(Alignment::Center));
    for _day in &["Pzt","Salı","Çarş","Per","Cuma"]{
        let mut table2 = elements::TableLayout::new(vec![1, 1, 1, 1, 1, 1, 1]);
        table2.set_cell_decorator(elements::FrameCellDecorator::new(true, false, false));
        let mut row2 = table2.row();
        for i in [1,2,3,4,5,6,7]{
            row2.push_element(create_row_title(i.to_string()));
        }
        row2.push().expect("");
        row.push_element(table2);
    }
    row.push().expect("");
    create_class_row(&mut table);
    doc.push(table);
}

fn create_class_row(table: &mut TableLayout){
    let clss = classes().lock_mut().to_vec();
    for c in clss{
        let mut row = table.row();
        row.push_element(create_row_title(format!("{}{}",c.kademe, c.sube)));
        for d in 0..5{
            let t = create_table_for_row(&c, d);
            row.push_element(t);
        }
        row.push().expect("msg");
    }
}
fn create_table_for_row(c: &Class, d: i32)-> TableLayout{
    let acts = activities().lock_mut().to_vec();
    let c_acts: Vec<&FullActivity> = acts.iter().filter(|a| a.classes.iter().any(|c2| c2 == &c.id)).collect();
    let sch = schedules().lock_mut().to_vec();
    let sch: Vec<&Schedule> = sch.iter().filter(|s| c_acts.iter().any(|a| a.id == s.activity)).collect();
    let l = selected_timetable_hour().lock_mut().to_vec();
    let mut table = genpdf::elements::TableLayout::new(vec![1; l.len()]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, false, false));
    let mut row = table.row();
    for i in 0..l.len(){
        let s = sch.iter().find(|s| s.day_id == d && s.hour as usize == i);
        match s{
            Some(s) => {
                let a = c_acts.iter().find(|a| a.id == s.activity).unwrap();
                let lec = lectures().lock_mut().to_vec();
                let lec = lec.iter().find(|l| l.id == a.subject).unwrap();
                let lec = lec.short_name.chars().take(2).collect::<String>();
                row.push_element(create_row_title(lec));
            }
            None => row.push_element(create_row_title2("".to_string()))
        }
    }
    row.push().expect("msg");    
    table
}
fn create_row_title(title: String) -> elements::Paragraph{
    let mut paragraph = elements::Paragraph::default();
    let mut title_style = style::Style::new();
    title_style.bold();
    title_style.set_font_size(10);
    paragraph = paragraph.styled_string(title, title_style);
    paragraph = paragraph.aligned(Alignment::Center);
    paragraph
}

fn create_row_title2(title: String) -> elements::Paragraph{
    let paragraph = elements::Paragraph::default();
    let mut title_style = style::Style::new();
    //title_style.bold();
    title_style.set_font_size(10);
    paragraph.styled_string(title, title_style)
}