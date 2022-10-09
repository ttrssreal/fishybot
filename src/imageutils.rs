use imageproc::drawing;
use std::{env::var};

fn get_text_width(text: &str, font: &rusttype::Font, font_scale: f32) -> u32 {
    let glyphs: Vec<_> = font.layout(text, rusttype::Scale::uniform(font_scale), rusttype::point(0.0, 0.0)).collect();
    let min_x = glyphs.first().map(|g| g.pixel_bounding_box().unwrap().min.x).unwrap();
    let max_x = glyphs.last().map(|g| g.pixel_bounding_box().unwrap().max.x).unwrap();
    (max_x - min_x) as u32
}

pub async fn fill_template(ign: &str, special: bool, fish: f64, junk: f64, treasure: f64) {
    static SPECIAL_COLOUR: image::Rgba<u8> = image::Rgba([255u8, 85u8, 255u8, 255u8]);
    static NORMAL_COLOUR: image::Rgba<u8> = image::Rgba([102u8, 104u8, 249u8, 255u8]);
    static BLACK: image::Rgba<u8> = image::Rgba([0u8, 0u8, 0u8, 255u8]);
    static WIDTH:i32 = 1542;

    let location = var("TEMPLATE_DIR").expect("no template location");
    let master_level =  if fish < 10_000f64 {0}
                                else {
                                    if fish < 25_000f64 {1}
                                else {
                                    if fish < 50_000f64 {2}
                                else {
                                    if fish < 100_000f64 {3}
                                else {4}
                            }}};
    let fish_to_level_up =  if fish < 10_000f64 {10000} 
                                    else {
                                        if fish < 25_000f64 {25000} 
                                    else {
                                        if fish < 50_000f64 {50_000} 
                                    else {
                                        if fish < 100_000f64 {100_000}
                                    else {100_000}
                                }}};
    let next_level = if master_level >= 4 {4} else {master_level + 1};

    let formated_ign = &format!("{}[{}]", ign, master_level);

    let max_len = 791 as f32;
    let len = { let length = (max_len/(fish_to_level_up as f32)*fish as f32) as i32; if length <= 0 {1} else  { if length > (max_len as i32) {max_len as i32} else {length} } };

    let title_size = 180f32;
    let fish_stats_text_size = 150f32;
    let chance_stats_text_size = 70f32;
    let levels_stats_text_size = 150f32;


    let font = rusttype::Font::try_from_bytes(include_bytes!("../reasources/Dubai-Medium.ttf")).unwrap();
    let mut dyn_image = imageproc::utils::load_image_or_panic(format!("{}/fishingtemplate.png", location));

    drawing::draw_text_mut(
        &mut dyn_image, 
        if special {SPECIAL_COLOUR} else {NORMAL_COLOUR}, 
        ((WIDTH/2) as i32)-(get_text_width(formated_ign, &font, title_size)/2) as i32, 14, 
        rusttype::Scale::uniform(title_size),
        &font,
        formated_ign
        
    );

    drawing::draw_text_mut(&mut dyn_image, BLACK, 170, 450, rusttype::Scale::uniform(fish_stats_text_size), &font, &fish.to_string());
    drawing::draw_text_mut(&mut dyn_image, BLACK, 620, 450, rusttype::Scale::uniform(fish_stats_text_size), &font, &junk.to_string());
    drawing::draw_text_mut(&mut dyn_image, BLACK, 1070,450, rusttype::Scale::uniform(fish_stats_text_size), &font, &treasure.to_string());

    drawing::draw_text_mut(&mut dyn_image, BLACK, 494+5,888-4, rusttype::Scale::uniform(chance_stats_text_size), &font, 
    &format!("{:.1}%", treasure/(treasure+fish)*100f64)
    );
    drawing::draw_text_mut(&mut dyn_image, BLACK, 1264+25+5,888-4, rusttype::Scale::uniform(chance_stats_text_size), &font, 
    &format!("{:.1}%", junk/(junk+fish)*100f64)
    );
    
    drawing::draw_text_mut(&mut dyn_image, NORMAL_COLOUR, 180-10,1280-15, rusttype::Scale::uniform(levels_stats_text_size), &font, &format!("{}", master_level));
    drawing::draw_text_mut(&mut dyn_image, NORMAL_COLOUR, 1340-10,1280-15, rusttype::Scale::uniform(levels_stats_text_size), &font, &format!("{}", next_level));
    
    drawing::draw_text_mut(&mut dyn_image, NORMAL_COLOUR, WIDTH/2-get_text_width(ign, &font, 65.0) as i32,1304+76, 
    rusttype::Scale::uniform(65 as f32), &font, 
    &format!("{} / {}", fish, fish_to_level_up)
    );

    drawing::draw_hollow_circle_mut(&mut dyn_image, (524+30,923-5), 45+30, BLACK);
    drawing::draw_hollow_circle_mut(&mut dyn_image, (1324+25,923-5), 45+30, BLACK);

    drawing::draw_filled_circle_mut(&mut dyn_image, (373,1304+28), 27, NORMAL_COLOUR);
    drawing::draw_filled_circle_mut(&mut dyn_image, (376+len,1304+28), 27, NORMAL_COLOUR);
    
    drawing::draw_filled_rect_mut(&mut dyn_image, imageproc::rect::Rect::at(373, 1304).of_size(len as u32, 56), NORMAL_COLOUR);
    
    dyn_image.save_with_format("generated.png", image::ImageFormat::Png).unwrap();
}