use eframe::{CreationContext, egui::{FontDefinitions, FontFamily}};
use std::fs;

mod app;
mod models;
mod pie_chart;
mod data_io;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "个人开销可视化",
        options,
        Box::new(|cc: &CreationContext| {
            // Font setup
            let mut fonts = FontDefinitions::default();
            let font_path = "C:\\Windows\\Fonts\\msjh.ttc"; // Or user-provided path/embedded font
            // let ttc_index = 0; // ttc_index is not used with FontData::from_owned for the whole file

            match fs::read(font_path) {
                Ok(font_bytes) => {
                    let font_name = "msjh_ttc_custom".to_owned();
                    // Attempt to load from .ttc, hoping from_owned handles it or loads the first font.
                    // This part might need adjustment if from_owned isn't suitable for .ttc directly
                    // or if a specific font from the collection is needed and this version of egui
                    // doesn't have from_owned_font_data_or_collection_index.
                    // For now, we proceed with from_owned as it's available.
                    fonts.font_data.insert(
                        font_name.clone(),
                        egui::FontData::from_owned(font_bytes),
                    );

                    fonts.families.entry(FontFamily::Proportional).or_default().insert(0, font_name.clone());
                    fonts.families.entry(FontFamily::Monospace).or_default().push(font_name);
                    
                    cc.egui_ctx.set_fonts(fonts);
                }
                Err(e) => {
                    panic!("错误：无法加载字体 '{}': {}. 程序将终止。", font_path, e);
                }
            }
            
            // Create and return the app instance from the app module
            Box::new(app::MyApp::default())
        }),
    )
}
