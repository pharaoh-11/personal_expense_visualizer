use eframe::{egui, App, Frame};
use egui::{CentralPanel, Color32, Context, Vec2, Response};
use crate::models::{ExpenseCategory, RawTransaction}; // Added RawTransaction
use crate::pie_chart;
use crate::data_io; // Import the data_io module
use std::path::Path;

pub struct MyApp {
    pub expenses: Vec<ExpenseCategory>, // For the expense pie chart
    pub income_categories: Vec<ExpenseCategory>, // For the income pie chart (to be added)
    pub raw_transactions: Vec<RawTransaction>, // Store loaded raw transactions
    // TODO: Add fields for input UI later
    // date_input_str: String,
    // amount_input: f32,
    // category_input: String,
}

impl Default for MyApp {
    fn default() -> Self {
        // Attempt to load data from the TXT file.
        // NOTE: The path is relative to where the executable is run.
        // For development with `cargo run` from `personal_expense_visualizer` dir,
        // this path should point to `../微信支付账单明细.txt`.
        // For a packaged app, the data file might be in the same dir or an `assets` dir.
        // For simplicity now, let's assume it's one level up from the project dir,
        // which is where the user's file is located relative to the project.
        let data_file_path_str = "../微信支付账单明细.txt"; // Relative to project root when running `cargo run`
        
        let mut loaded_raw_transactions = Vec::new();
        match data_io::load_raw_transactions_from_file(Path::new(data_file_path_str)) {
            Ok(raw_txs) => {
                println!("成功加载 {} 条原始交易记录。", raw_txs.len());
                loaded_raw_transactions = raw_txs;
                // TODO: Process these raw_txs into self.expenses and self.income_categories
            }
            Err(e) => {
                eprintln!("错误：无法从 '{}' 加载交易数据: {}", data_file_path_str, e);
                // Proceed with empty or default data
            }
        }

        // TODO: Replace with actual data loading and processing from data_io.rs
        // For now, expenses are still hardcoded until processing is implemented.
        // Income categories will also be populated from processed data.
        Self {
            expenses: vec![
                ExpenseCategory { name: "餐饮".to_string(), amount: 300.0, color: Color32::from_rgb(255, 99, 71) },
                ExpenseCategory { name: "购物".to_string(), amount: 200.0, color: Color32::from_rgb(60, 179, 113) },
                ExpenseCategory { name: "交通".to_string(), amount: 150.0, color: Color32::from_rgb(70, 130, 180) },
                ExpenseCategory { name: "娱乐".to_string(), amount: 100.0, color: Color32::from_rgb(255, 215, 0) },
                ExpenseCategory { name: "其他".to_string(), amount: 50.0, color: Color32::from_rgb(128, 128, 128) },
            ],
            income_categories: vec![], // Initialize as empty
            raw_transactions: loaded_raw_transactions,
            // date_input_str: String::new(),
            // amount_input: 0.0,
            // category_input: "餐饮".to_string(),
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
            let (rect, response) = ui.allocate_exact_size(Vec2::splat(chart_size), egui::Sense::hover()); // Keep response for hover later
            let center = rect.center();
            let radius = chart_size / 2.0;

            let painter = ui.painter_at(rect); // painter_at takes the rect for clipping

            let total_amount: f32 = self.expenses.iter().map(|e| e.amount).sum();
            if total_amount == 0.0 {
                painter.text(center, egui::Align2::CENTER_CENTER, "无数据", egui::FontId::proportional(20.0), Color32::GRAY);
            } else {
                pie_chart::draw_pie_chart(ui, painter, center, radius, &response, &self.expenses, ctx);
            }


            // Placeholder for data input UI (to be implemented later)
            ui.separator();
            ui.label("数据输入区域 (待实现)");
            // ui.horizontal(|ui| {
            //     ui.label("日期:");
            //     ui.add(egui::TextEdit::singleline(&mut self.date_input_str));
            // });
            // ui.horizontal(|ui| {
            //     ui.label("金额:");
            //     ui.add(egui::DragValue::new(&mut self.amount_input).speed(1.0));
            // });
            // ui.horizontal(|ui| {
            //     ui.label("类别:");
            //     // egui::ComboBox or TextEdit for category
            // });
            // if ui.button("添加记录").clicked() {
            //     // Add record logic
            // }
        });
    }
}
