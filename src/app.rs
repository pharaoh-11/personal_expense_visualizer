use chrono::NaiveDate;
use eframe::{egui, App, Frame};
use egui::{CentralPanel, Color32, Context, Vec2, ScrollArea, Grid, RichText}; // Removed unused TextFormat
use crate::models::{ExpenseCategory, ProcessedTransaction, TransactionDirection};
use crate::pie_chart;
use crate::data_io;
use std::path::{Path, PathBuf};

// Removed #[derive(Default)] to resolve conflict
pub struct MyApp {
    expenses: Vec<ExpenseCategory>,
    income_categories: Vec<ExpenseCategory>,
    all_processed_transactions: Vec<ProcessedTransaction>,
    current_data_file: Option<PathBuf>,
    hovered_expense_idx: Option<usize>, // Index of hovered segment in expenses pie
    hovered_income_idx: Option<usize>,  // Index of hovered segment in income pie
    
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
                let (processed, expenses, incomes) = data_io::process_transactions_for_display(&raw_txs);
                self.all_processed_transactions = processed;
                self.expenses = expenses;
                self.income_categories = incomes;
                self.current_data_file = Some(file_path.to_path_buf());
            }
            Err(e) => {
                eprintln!("错误：无法从 '{}' 加载或处理交易数据: {}", file_path.display(), e);
                self.all_processed_transactions.clear();
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
            all_processed_transactions: vec![],
            current_data_file: None,
            hovered_expense_idx: None,
            hovered_income_idx: None,
            input_date_str: ::chrono::Local::now().format("%Y/%m/%d").to_string(),
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
        // State for hovered segment (index, type)
        // We'll manage this more directly in pie_chart drawing for now.
        // let mut hovered_segment_info: Option<(usize, &'static str)> = None;

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

            // Pie charts section
            let pie_chart_section_height = ui.available_height() * 0.4;
            ui.allocate_ui(Vec2::new(ui.available_width(), pie_chart_section_height.max(150.0)), |ui| {
                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("支出概览");
                        let chart_size = ui.available_width().min(ui.available_height()).max(100.0);
                        let (rect, response) = ui.allocate_exact_size(Vec2::splat(chart_size), egui::Sense::hover().union(egui::Sense::click()));
                        let center = rect.center();
                        let radius = chart_size / 2.0 * 0.9;

                        if self.expenses.is_empty() {
                            ui.painter_at(rect).text(center, egui::Align2::CENTER_CENTER, "无支出数据", egui::FontId::proportional(16.0), Color32::GRAY);
                        } else {
                            // Pass current hover state and get back new hover state
                            let new_hover_idx = pie_chart::draw_pie_chart(
                                ui, ui.painter_at(rect), center, radius, &response, 
                                &self.expenses, ctx, "expense_chart", self.hovered_expense_idx,
                            );
                            if response.hovered() { // Only update if mouse is over the chart area
                                self.hovered_expense_idx = new_hover_idx;
                            } else {
                                self.hovered_expense_idx = None; // Clear hover if mouse leaves chart area
                            }
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.heading("收入概览");
                        let chart_size = ui.available_width().min(ui.available_height()).max(100.0);
                        let (rect, response) = ui.allocate_exact_size(Vec2::splat(chart_size), egui::Sense::hover().union(egui::Sense::click()));
                        let center = rect.center();
                        let radius = chart_size / 2.0 * 0.9;

                        if self.income_categories.is_empty() {
                            ui.painter_at(rect).text(center, egui::Align2::CENTER_CENTER, "无收入数据", egui::FontId::proportional(16.0), Color32::GRAY);
                        } else {
                            let new_hover_idx = pie_chart::draw_pie_chart(
                                ui, ui.painter_at(rect), center, radius, &response, 
                                &self.income_categories, ctx, "income_chart", self.hovered_income_idx,
                            );
                            if response.hovered() {
                                self.hovered_income_idx = new_hover_idx;
                            } else {
                                self.hovered_income_idx = None;
                            }
                        }
                    });
                });
            });
            
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.heading("交易记录明细");
            // Allocate remaining height for table and input, ensuring some minimums
            let remaining_height = ui.available_height();
            let table_scroll_height = (remaining_height * 0.5).max(100.0); // Min 100px for table
            
            ScrollArea::vertical().max_height(table_scroll_height).show(ui, |ui| {
                Grid::new("transactions_table")
                    .num_columns(5)
                    .spacing([10.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        // Header
                        ui.strong("日期");
                        ui.strong("类别");
                        ui.strong("项目/对方");
                        ui.strong("金额");
                        ui.strong("收/支");
                        ui.end_row();

                        for tx in &self.all_processed_transactions {
                            ui.label(tx.date.format("%Y-%m-%d").to_string());
                            ui.label(&tx.category);
                            let item_display = if tx.original_item_name != "/" && !tx.original_item_name.is_empty() {
                                tx.original_item_name.chars().take(30).collect::<String>() // Limit length
                            } else {
                                tx.original_counterparty.chars().take(30).collect::<String>()
                            };
                            ui.label(item_display);
                            ui.label(format!("{:.2}", tx.amount));
                            match tx.direction {
                                TransactionDirection::Income => ui.label(RichText::new("收入").color(Color32::GREEN)),
                                TransactionDirection::Expense => ui.label(RichText::new("支出").color(Color32::RED)),
                                TransactionDirection::Neutral => ui.label("中性"),
                            };
                            ui.end_row();
                        }
                    });
            });


            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            
            ui.heading("手动添加记录");
            Grid::new("input_grid").num_columns(2).spacing([40.0, 4.0]).show(ui, |ui| {
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
                    if let (Ok(_date), Ok(amount)) = ( // Changed date to _date as it's not directly used to create ExpenseCategory here
                        NaiveDate::parse_from_str(&self.input_date_str, "%Y/%m/%d"),
                        self.input_amount_str.parse::<f32>()
                    ) {
                        if !self.input_category_str.trim().is_empty() && amount > 0.0 {
                            let category_name = self.input_category_str.trim().to_string();
                            let color = data_io::get_color_for_category(&category_name, 
                                if self.input_is_expense { self.expenses.len() } else { self.income_categories.len() });

                            let category_name_clone = category_name.clone();
                            let new_expense_category_entry = ExpenseCategory {
                                name: category_name, // category_name is moved here for ExpenseCategory
                                amount,
                                color,
                            };

                            let direction = if self.input_is_expense {
                                TransactionDirection::Expense
                            } else {
                                TransactionDirection::Income
                            };

                            // Add to the detailed transaction list for the table
                            self.all_processed_transactions.push(ProcessedTransaction {
                                date: _date, // Use the parsed date
                                category: category_name_clone.clone(), // Use cloned category name
                                amount,
                                direction,
                                original_item_name: self.input_item_str.clone(),
                                original_counterparty: String::new(), // Manual entry might not have a counterparty
                                original_transaction_type: "手动输入".to_string(),
                            });
                            
                            // Sort all_processed_transactions by date (optional, but good for display)
                            self.all_processed_transactions.sort_by_key(|t| t.date);


                            if self.input_is_expense {
                                if let Some(existing_cat) = self.expenses.iter_mut().find(|cat| cat.name == new_expense_category_entry.name) {
                                    existing_cat.amount += new_expense_category_entry.amount;
                                } else {
                                    self.expenses.push(new_expense_category_entry);
                                }
                                // Sort expenses for consistent pie chart display
                                self.expenses.sort_by(|a,b| b.amount.partial_cmp(&a.amount).unwrap_or(std::cmp::Ordering::Equal));
                            } else {
                                if let Some(existing_cat) = self.income_categories.iter_mut().find(|cat| cat.name == new_expense_category_entry.name) {
                                    existing_cat.amount += new_expense_category_entry.amount;
                                } else {
                                    self.income_categories.push(new_expense_category_entry);
                                }
                                // Sort income categories for consistent pie chart display
                                self.income_categories.sort_by(|a,b| b.amount.partial_cmp(&a.amount).unwrap_or(std::cmp::Ordering::Equal));
                            }
                            
                            // Clear input fields
                            self.input_date_str = ::chrono::Local::now().format("%Y/%m/%d").to_string();
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
