pub fn font_family()-> genpdf::fonts::FontFamily<genpdf::fonts::FontData>{
    let font_bold =
        include_bytes!("../statics/fonts/Ubuntu-Bold.ttf");
    let font_bold_data = genpdf::fonts::FontData::new(
        font_bold.to_vec(),
        None,
    ).expect("font data should be correct");
    let font_bold_italic =
        include_bytes!("../statics/fonts/Ubuntu-BoldItalic.ttf");
    let font_bold_italic_data = genpdf::fonts::FontData::new(
        font_bold_italic.to_vec(),
        None,
    ).expect("font data should be correct");
    let font_regular =
        include_bytes!("../statics/fonts/Ubuntu-Regular.ttf");
    let font_regular_data = genpdf::fonts::FontData::new(
        font_regular.to_vec(),
        None,
    ).expect("font data should be correct");

    let font_regular_italic =
        include_bytes!("../statics/fonts/Ubuntu-Regular.ttf");
    let font_regular_italic_data = genpdf::fonts::FontData::new(
        font_regular_italic.to_vec(),
        None,
    ).expect("font data should be correct");
    let font_family = genpdf::fonts::FontFamily {
        regular: font_regular_data,
        bold: font_bold_data,
        italic: font_regular_italic_data,
        bold_italic: font_bold_italic_data,
    };
    font_family
}