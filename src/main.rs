use eframe::{egui, App, CreationContext, Frame}; // Added CreationContext
use egui::{CentralPanel, Context, Color32, Vec2, Pos2, Stroke, epaint::PathShape, FontDefinitions, FontFamily}; // Added FontDefinitions, FontFamily
use std::f32::consts::PI;

struct ExpenseCategory {
    name: String,
    amount: f32,
    color: Color32,
}

struct MyApp {
    expenses: Vec<ExpenseCategory>,
}

impl Default for MyApp {
    fn default() -> Self {
        // Sample data for now
        Self {
            expenses: vec![
                ExpenseCategory { name: "餐饮".to_string(), amount: 300.0, color: Color32::from_rgb(255, 99, 71) }, // Tomato
                ExpenseCategory { name: "购物".to_string(), amount: 200.0, color: Color32::from_rgb(60, 179, 113) }, // MediumSeaGreen
                ExpenseCategory { name: "交通".to_string(), amount: 150.0, color: Color32::from_rgb(70, 130, 180) }, // SteelBlue
                ExpenseCategory { name: "娱乐".to_string(), amount: 100.0, color: Color32::from_rgb(255, 215, 0) }, // Gold
                ExpenseCategory { name: "其他".to_string(), amount: 50.0, color: Color32::from_rgb(128, 128, 128) }, // Gray
            ],
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("个人日常开销可视化");

            // Allocate space for the pie chart
            let available_size = ui.available_size();
            let chart_size = available_size.x.min(available_size.y) * 0.8;
            let (rect, _response) = ui.allocate_exact_size(Vec2::splat(chart_size), egui::Sense::hover());
            let center = rect.center();
            let radius = chart_size / 2.0;

            let painter = ui.painter_at(rect);

            let total_amount: f32 = self.expenses.iter().map(|e| e.amount).sum();
            if total_amount == 0.0 {
                painter.text(center, egui::Align2::CENTER_CENTER, "无数据", egui::FontId::proportional(20.0), Color32::GRAY);
                return;
            }

            let mut start_angle = -PI / 2.0; // Start from the top

            for expense in &self.expenses {
                let proportion = expense.amount / total_amount;
                let angle_delta = proportion * 2.0 * PI;
                let end_angle = start_angle + angle_delta;

                // Draw the pie slice (arc)
                let mut points = vec![center];
                let num_segments = (angle_delta * radius / 5.0).ceil().max(10.0) as usize; // Adaptive number of segments
                for i in 0..=num_segments {
                    let angle = start_angle + (angle_delta * i as f32 / num_segments as f32);
                    points.push(Pos2::new(center.x + radius * angle.cos(), center.y + radius * angle.sin()));
                }
                points.push(center); // Close the shape

                let shape = PathShape::convex_polygon(points, expense.color, Stroke::NONE);
                painter.add(shape);

                // TODO: Add labels and hover information

                start_angle = end_angle;
            }

            // Placeholder for data input UI (to be implemented later)
            ui.separator();
            ui.label("数据输入区域 (待实现)");
            // Example:
            // ui.horizontal(|ui| {
            //     ui.label("日期:");
            //     // ui.add(egui::TextEdit::singleline(&mut date_input_str));
            // });
            // ui.horizontal(|ui| {
            //     ui.label("金额:");
            //     // ui.add(egui::DragValue::new(&mut amount_input).speed(1.0));
            // });
            // ui.horizontal(|ui| {
            //     ui.label("类别:");
            //     // egui::ComboBox::from_label("")
            //     //     .selected_text(format!("{:?}", selected_category_input))
            //     //     .show_ui(ui, |ui| {
            //     //         // Populate with categories
            //     //     });
            // });
            // if ui.button("添加记录").clicked() {
            //     // Add record logic
            // }
        });
    }
}

use std::fs; // Added for file system operations

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "个人开销可视化",
        options,
        Box::new(|cc: &CreationContext| {
            let mut fonts = FontDefinitions::default();
            let font_path = "C:\\Windows\\Fonts\\msjh.ttc"; // Using msjh.ttc as per user

            match fs::read(font_path) {
                Ok(font_bytes) => {
                    let font_name = "msjh_ttc_custom".to_owned(); 
                    fonts.font_data.insert(
                        font_name.clone(),
                        // Try loading the .ttc file directly with from_owned.
                        // This might load the first font in the collection or have undefined behavior
                        // if from_owned is not designed for .ttc files directly in this egui version.
                        egui::FontData::from_owned(font_bytes),
                    );

                    fonts
                        .families
                        .entry(FontFamily::Proportional)
                        .or_default()
                        .insert(0, font_name.clone());

                    fonts
                        .families
                        .entry(FontFamily::Monospace)
                        .or_default()
                        .push(font_name);
                    
                    cc.egui_ctx.set_fonts(fonts);
                }
                Err(e) => {
                    // If font loading fails, panic. This will terminate the application.
                    panic!("错误：无法加载字体 '{}': {}. 程序将终止。", font_path, e);
                }
            }
            Box::new(MyApp::default()) // Return Box<MyApp> directly
        }),
    )
}
