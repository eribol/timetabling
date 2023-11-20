use shared::msgs::activities::FullActivity;
use genpdf::*;

use crate::{app::timetables::{selected_timetable_hour, schedules, activities, teachers::teachers, add_act::{classes_full_name, lecture_name}, school}, i18n};

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
    let name = school().get_cloned().unwrap().name;
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
        title.push(title_paragraph.styled_string(&name, title_style).aligned(Alignment::Center));
        let teacher_name = format!("{} {}", teacher.first_name, teacher.last_name);
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
    row.push_element(create_row_title(format!("{} {}", i18n::t_s!("days"), i18n::t_s!("hours"))).aligned(Alignment::Center));
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
        let par = elements::Paragraph::default().styled_string("", line_style).aligned(Alignment::Center);
        hour_row.push(par);
        row.push_element(hour_row);
        for day in ["p","s","ç","p","c","ct","p"].iter().enumerate(){
            let sch = sch.iter().find(|s| s.day_id == (day.0+1) as i32 && s.hour == h as i32 && acts.iter().any(|a| a.id == s.activity));
            match sch{
                Some(s) => {
                    let act = acts.iter().find(|a| a.id == s.activity).unwrap();
                    let c_name = classes_full_name(act.clone());
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

fn _hour_linear(h: usize)->elements::Paragraph{
   create_row_title(h.to_string())
}

fn create_row_title(title: String) -> elements::Paragraph{
    let paragraph = elements::Paragraph::default();
    let mut title_style = style::Style::new();
    title_style.set_font_size(12);
    paragraph.styled_string(title, title_style)
}

/*
pub fn print_teachers(
    school: &School,
    timetables: &Vec<(String, String, Vec<Schedule>)>,
    hour: i32,
    schedules: &Vec<Schedule>,
    acts: &Vec<Activity>,
)
{
    let browser_lang = window().unwrap().navigator().language();
    //let mut b_lang = DEFAULT_LANG;
    //match browser_lang{
    //    Some(ref l) => {
    //        if l == &"tr-TR".to_string() {b_lang = Lang::TrTR}
    //    },
    //    _ => {}
    //}
    

    use crate::fonts::font_family;
    let mut doc = genpdf::Document::new(font_family());
    doc.set_title(format!("{}", &school.name));
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10 as i32);

    doc.set_page_decorator(decorator);
    doc.set_line_spacing(1.25);

    for timetable in timetables{
        use fluent::fluent_args;
        let mut title = elements::LinearLayout::vertical();

        let mut title_style = style::Style::new();
        title_style.set_bold();
        title_style.set_font_size(20);

        let title_paragraph = elements::Paragraph::default();
        title.push(title_paragraph.styled_string(&school.name, title_style).aligned(Alignment::Center));
        let teacher = format!("{} {}", &timetable.0, timetable.1);
        let teacher_name = "A";
        let teacher_name = teacher_name.replace("\u{2068}", "");
        let teacher_name = teacher_name.replace("\u{2069}", "");
        let mut teacher_style = style::Style::new();
        teacher_style.set_italic();
        teacher_style.set_font_size(18);
        title.push(elements::Paragraph::new(&teacher_name).aligned(Alignment::Center));
        doc.push(title);
        doc.push(elements::Break::new(1));
        add_teacher_rows(&mut doc, timetable, hour, acts, "A".to_string());
    }
    let mut buf: Vec<u8> = Vec::new();
    doc.render(&mut buf).expect("Render edilemedi");
    let png_jsarray: JsValue = js_sys::Uint8Array::from(&buf[..]).into();
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


fn add_teacher_rows(
    doc: &mut genpdf::Document,
    timetables: &(String, String, Vec<Schedule>),
    hour: i32,
    acts: &Vec<Activity>,
    school_ctx: String,
)
{
    use fluent::fluent_args;
    let mut table = elements::TableLayout::new(vec![8, 8, 8, 8, 8, 8, 8, 8]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    let mut row = table.row();
    row.push_element(create_row_title(format!("{} {}", t!["days"], t!["hours"])).aligned(Alignment::Center));
    for day in &days{
        row.push_element(create_row_title(format!("{}",t![t!("day", fluent_args!["dayId" => &day.id])])).aligned(Alignment::Center));
    }
    row.push().expect("Invalid table row");
    for h in 0..hour{
        let schedules_hour = &schedules[h as usize];
        let mut row = table.row();
        let mut hour_linear = elements::LinearLayout::vertical();
        hour_linear.push(create_row_title(format!("{}. {}", h+1, t!["hour"])).aligned(Alignment::Center));
        let paragraph = elements::Paragraph::default();
        let mut new_line = style::Style::new();
        new_line.set_font_size(8);
        hour_linear.push(paragraph.styled_string(format!("{}-{}", &schedules_hour.start_time.format("%H:%M"), &schedules_hour.end_time.format("%H:%M")), new_line).aligned(Alignment::Center));
        row.push_element(hour_linear);
        for d in &days{
            let timetable = timetables.2.iter().find(|t| t.hour == h as i16 && t.day_id == d.id);
            match timetable{
                Some(t) => {
                    let mut linear = elements::LinearLayout::vertical();
                    let mut classes = t.get_activity(acts).get_full_activity(school_ctx, url).classes.iter().map(|c| format!("{}/{}-", c.kademe, c.sube)).collect::<String>();
                    classes.remove(classes.len()-1);
                    linear.push(create_class_line(&classes).aligned(Alignment::Center));
                    let subject = t.get_activity(acts).get_full_activity(school_ctx, url).subject;
                    linear.push(create_class_line(&subject.name).aligned(Alignment::Center));
                    row.push_element(linear);
                }
                None => {
                    row.push_element(elements::Paragraph::new("").aligned(Alignment::Center));
                }
            }

        }
        row.push().expect("Invalid table row");
    }
    doc.push(table);
    doc.push(elements::PageBreak::new());
}

pub fn print_classes(
    school: &SchoolDetail,
    timetables: &Vec<(String, String, Vec<Timetable>)>,
    hour: i32,
    schedules: &Vec<Schedule>,
    acts: &Vec<Activity>,
    school_ctx: &SchoolContext,
    url: &Url
) {
    let browser_lang = window().navigator().language();
    let mut b_lang = DEFAULT_LANG;
    match browser_lang{
        Some(ref l) => {
            if l == &"tr-TR".to_string() {b_lang = Lang::TrTR}
        },
        _ => {}
    }
    let lang= I18n::new(b_lang);
    create_t![lang];
    let font_bold =
        include_bytes!("../statics/fonts/Ubuntu-Bold.ttf");
    let font_bold_data = fonts::FontData::new(
        font_bold.to_vec(),
        None,
    ).expect("font data should be correct");

    let font_bold_italic =
        include_bytes!("../statics/fonts/Ubuntu-BoldItalic.ttf");
    let font_bold_italic_data = fonts::FontData::new(
        font_bold_italic.to_vec(),
        None,
    ).expect("font data should be correct");

    let font_regular =
        include_bytes!("../statics/fonts/Ubuntu-Regular.ttf");
    let font_regular_data = fonts::FontData::new(
        font_regular.to_vec(),
        None,
    ).expect("font data should be correct");

    let font_regular_italic =
        include_bytes!("../statics/fonts/Ubuntu-Regular.ttf");
    let font_regular_italic_data = fonts::FontData::new(
        font_regular_italic.to_vec(),
        None,
    ).expect("font data should be correct");
    let font_family = fonts::FontFamily {
        regular: font_regular_data,
        bold: font_bold_data,
        italic: font_regular_italic_data,
        bold_italic: font_bold_italic_data,
    };
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title(format!("{}", &school.name));
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10 as i32);

    doc.set_page_decorator(decorator);
    doc.set_line_spacing(1.25);

    for timetable in timetables{
        use fluent::fluent_args;
        let mut title = elements::LinearLayout::vertical();

        let title_paragraph = elements::Paragraph::default();
        title.push(title_paragraph.styled_string(&school.name, create_title()).aligned(Alignment::Center));
        let class = format!("{} {}", &timetable.0, timetable.1);
        let class_name = t!("weekly-timetable-schedule-for-class", fluent_args!["className" => class.clone()]);
        let class_name = class_name.replace("\u{2068}", "");
        let class_name = class_name.replace("\u{2069}", "");
        title.push(elements::Paragraph::new(&class_name).aligned(Alignment::Center));
        doc.push(title);
        doc.push(elements::Break::new(1));
        add_class_rows(&mut doc, timetable, hour, &lang, schedules, acts, school_ctx, url);
    }
    let mut buf: Vec<u8> = Vec::new();
    doc.render(&mut buf).expect("Render edilemedi");
    let png_jsarray: JsValue = js_sys::Uint8Array::from(&buf[..]).into();
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

fn add_class_rows(
    doc: &mut genpdf::Document,
    timetables: &(String, String, Vec<Timetable>),
    hour: i32,
    lang: &I18n,
    schedules: &Vec<Schedule>,
    acts: &Vec<Activity>,
    school_ctx: &SchoolContext,
    url: &Url
) {
    create_t![lang];
    use fluent::fluent_args;
    let mut table = elements::TableLayout::new(vec![8; 8]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    let mut row = table.row();
    row.push_element(create_row_title(format!("{} {}", t!["days"], t!["hours"])).aligned(Alignment::Center));
    let days = create_days();
    for day in &days{
        row.push_element(create_row_title(t![t!("day", fluent_args!["dayId" => &day.id])]).aligned(Alignment::Center));
    }
    row.push().expect("Invalid table row");
    for h in 0..hour{
        let hour_schedules = &schedules[h as usize];
        let mut row = table.row();
        let mut hour_linear = elements::LinearLayout::vertical();
        hour_linear.push(create_row_title(format!("{}. {}", h+1, t!["hour"])).aligned(Alignment::Center));
        let paragraph = elements::Paragraph::default();
        let mut new_line = style::Style::new();
        new_line.set_font_size(8);
        hour_linear.push(paragraph.styled_string(format!("{}-{}", &hour_schedules.start_time.format("%H:%M"), &hour_schedules.end_time.format("%H:%M")), new_line).aligned(Alignment::Center));
        row.push_element(hour_linear);
        for d in &days{
            let timetable = timetables.2.iter().find(|t| t.hour == h as i16 && t.day_id == d.id);
            match timetable{
                Some(tt) => {
                    let mut linear = elements::LinearLayout::vertical();
                    let mut teachers = tt.get_activity(acts).get_full_activity(school_ctx, url).teachers.iter()
                        .map(
                            |t|
                                format!("{}-", &t.short_name)
                                ).collect::<String>();
                    teachers.remove(teachers.len()-1);
                    let subject = &tt.get_activity(acts).get_full_activity(school_ctx, url).subject;
                    linear.push(create_subject_line(&subject.short_name).aligned(Alignment::Center));
                    linear.push(create_teacher_line(teachers));
                    row.push_element(linear);
                }
                None => {
                    row.push_element(elements::Paragraph::new("").aligned(Alignment::Center));
                }
            }

        }
        row.push().expect("Invalid table row");
    }
    doc.push(table);
    doc.push(elements::PageBreak::new());
}

fn create_title() -> style::Style{
    let mut title_style = style::Style::new();
    title_style.set_bold();
    title_style.set_font_size(20);
    title_style
}

fn create_row_title(title: String) -> elements::Paragraph{
    let paragraph = elements::Paragraph::default();
    let mut title_style = style::Style::new();
    title_style.set_font_size(12);
    paragraph.styled_string(title, title_style)
}

fn create_subject_line(title: &String) -> elements::Paragraph{
    let paragraph = elements::Paragraph::default();
    let mut title_style = style::Style::new();
    title_style.set_bold();
    title_style.set_font_size(7);
    paragraph.styled_string(title, title_style)
}

fn create_class_line(title: &String) -> elements::Paragraph{
    let paragraph = elements::Paragraph::default();
    let mut title_style = style::Style::new();
    title_style.set_italic();
    title_style.set_font_size(8);
    paragraph.styled_string(title, title_style)
}

fn create_teacher_line(name: String) -> elements::Paragraph{
    let paragraph = elements::Paragraph::default();
    let mut title_style = style::Style::new();
    title_style.set_italic();
    title_style.set_font_size(6);
    paragraph.styled_string(name, title_style).aligned(Alignment::Center)
}
*/