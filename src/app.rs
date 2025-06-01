use chrono::NaiveDate; // Added NaiveDate
use eframe::{egui, App, Frame};
use egui::{CentralPanel, Color32, Context, Vec2}; // Removed unused Response, ScrollArea, TextStyle
use crate::models::ExpenseCategory; // Removed unused RawTransaction
use crate::pie_chart;
use crate::data_io;
use std::path::{Path, PathBuf};

pub struct MyApp {
    expenses: Vec<ExpenseCategory>,
    income_categories: Vec<ExpenseCategory>,
    // raw_transactions: Vec<RawTransaction>, // We might not need to store raw if processed immediately
    current_data_file: Option<PathBuf>,
    // For manual input
    input_date_str: String,
    input_amount_str: String,
    input_category_str: String,
    input_is_expense: bool,
    input_item_str: String,
}

impl MyApp {
    fn load_and_process_data(&mut self, file_path: &Path) {
        match data_io::load_raw_transactions_from_file(file_path) {
            Ok(raw_txs) => {
                println!("成功从 '{}' 加载 {} 条原始交易记录。", file_path.display(), raw_txs.len());
                let (expenses, incomes) = data_io::process_and_aggregate_transactions(&raw_txs);
                self.expenses = expenses;
                self.income_categories = incomes;
                self.current_data_file = Some(file_path.to_path_buf());
            }
            Err(e) => {
                eprintln!("错误：无法从 '{}' 加载或处理交易数据: {}", file_path.display(), e);
                // Optionally clear existing data or show an error message in UI
                self.expenses.clear();
                self.income_categories.clear();
            }
        }
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let mut app = Self {
            expenses: vec![],
            income_categories: vec![],
            current_data_file: None,
            input_date_str: ::chrono::Local::now().format("%Y/%m/%d").to_string(), // Prefixed with ::chrono
            input_amount_str: String::new(),
            input_category_str: String::new(),
            input_is_expense: true,
            input_item_str: String::new(),
        };
        // Attempt to load data from the default TXT file.
        let default_data_file_path_str = "../微信支付账单明细.txt";
        app.load_and_process_data(Path::new(default_data_file_path_str));
        app
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("个人财务可视化");
            ui.add_space(10.0);

            if ui.button("导入微信导出的收入/支出TXT").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Text files", &["txt"])
                    .pick_file()
                {
                    self.load_and_process_data(&path);
                }
            }
            if let Some(path) = &self.current_data_file {
                ui.label(format!("当前数据文件: {}", path.display()));
            }


            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.heading("支出概览"); // Changed h2 to heading
                    let available_width = ui.available_width();
                    let chart_size = available_width.min(ui.available_height() * 0.4).max(100.0);
                    let (rect, response) = ui.allocate_exact_size(Vec2::splat(chart_size), egui::Sense::hover());
                    let center = rect.center();
                    let radius = chart_size / 2.0 * 0.9; // Slightly smaller radius for padding

                    if self.expenses.is_empty() {
                        ui.painter_at(rect).text(center, egui::Align2::CENTER_CENTER, "无支出数据", egui::FontId::proportional(16.0), Color32::GRAY);
                    } else {
                        pie_chart::draw_pie_chart(ui, ui.painter_at(rect), center, radius, &response, &self.expenses, ctx);
                    }
                });

                ui.separator();

                ui.vertical(|ui| {
                    ui.heading("收入概览"); // Changed h2 to heading
                    let available_width = ui.available_width();
                    let chart_size = available_width.min(ui.available_height() * 0.4).max(100.0);
                    let (rect, response) = ui.allocate_exact_size(Vec2::splat(chart_size), egui::Sense::hover());
                    let center = rect.center();
                    let radius = chart_size / 2.0 * 0.9;

                    if self.income_categories.is_empty() {
                        ui.painter_at(rect).text(center, egui::Align2::CENTER_CENTER, "无收入数据", egui::FontId::proportional(16.0), Color32::GRAY);
                    } else {
                        pie_chart::draw_pie_chart(ui, ui.painter_at(rect), center, radius, &response, &self.income_categories, ctx);
                    }
                });
            });
            
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.heading("手动添加记录"); // Changed h2 to heading
            egui::Grid::new("input_grid").num_columns(2).spacing([40.0, 4.0]).show(ui, |ui| {
                ui.label("日期 (YYYY/MM/DD):");
                ui.text_edit_singleline(&mut self.input_date_str);
                ui.end_row();

                ui.label("项目/商品:");
                ui.text_edit_singleline(&mut self.input_item_str);
                ui.end_row();
                
                ui.label("金额:");
                ui.text_edit_singleline(&mut self.input_amount_str);
                ui.end_row();

                ui.label("类别:");
                ui.text_edit_singleline(&mut self.input_category_str);
                ui.end_row();

                ui.label("类型:");
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.input_is_expense, true, "支出");
                    ui.radio_value(&mut self.input_is_expense, false, "收入");
                });
                ui.end_row();

                if ui.button("添加记录").clicked() {
                    // Basic validation and parsing
                    if let (Ok(date), Ok(amount)) = (
                        NaiveDate::parse_from_str(&self.input_date_str, "%Y/%m/%d"),
                        self.input_amount_str.parse::<f32>()
                    ) {
                        if !self.input_category_str.trim().is_empty() && amount > 0.0 {
                            let category_name = self.input_category_str.trim().to_string();
                            let color = data_io::get_color_for_category(&category_name, 
                                if self.input_is_expense { self.expenses.len() } else { self.income_categories.len() });

                            let new_entry = ExpenseCategory {
                                name: category_name,
                                amount,
                                color,
                            };

                            if self.input_is_expense {
                                // Check if category exists, if so, add to it, otherwise push new
                                if let Some(existing_cat) = self.expenses.iter_mut().find(|cat| cat.name == new_entry.name) {
                                    existing_cat.amount += new_entry.amount;
                                } else {
                                    self.expenses.push(new_entry);
                                }
                            } else {
                                if let Some(existing_cat) = self.income_categories.iter_mut().find(|cat| cat.name == new_entry.name) {
                                    existing_cat.amount += new_entry.amount;
                                } else {
                                    self.income_categories.push(new_entry);
                                }
                            }
                            
                            // Clear input fields
                            self.input_date_str = ::chrono::Local::now().format("%Y/%m/%d").to_string(); // Prefixed with ::chrono
                            self.input_item_str.clear();
                            self.input_amount_str.clear();
                            self.input_category_str.clear();
                        } else {
                            // TODO: Show error message in UI
                            eprintln!("输入无效：类别不能为空且金额必须大于0。");
                        }
                    } else {
                        // TODO: Show error message in UI for parsing
                        eprintln!("输入无效：日期或金额格式不正确。");
                    }
                }
            });
        });
    }
}
