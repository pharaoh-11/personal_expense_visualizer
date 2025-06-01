use crate::models::{RawTransaction, ExpenseCategory, TransactionDirection}; // Removed ProcessedTransaction
use chrono::NaiveDate;
use eframe::egui::Color32;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

// const EXPECTED_COLUMNS: usize = 11; // Already defined, can be removed if not used elsewhere

pub fn load_raw_transactions_from_file(
    file_path: &Path,
) -> io::Result<Vec<RawTransaction>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut transactions = Vec::new();
    let mut start_parsing_data = false;
    let mut header_skipped = false;

    for line_result in reader.lines() {
        let line = line_result?;

        if line.contains("----------------------微信支付账单明细列表--------------------") {
            start_parsing_data = true;
            continue;
        }

        if !start_parsing_data {
            continue;
        }

        if !header_skipped {
            header_skipped = true;
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() >= 8 {
            let timestamp_str = parts[0].trim().to_string();
            let transaction_type = parts[1].trim().to_string();
            let counterparty = parts[2].trim().to_string();
            let item_name = parts[3].trim().to_string();
            let direction = parts[4].trim().to_string();
            let amount_str = parts[5].trim().to_string();
            let payment_method = parts[6].trim().to_string();
            let status = parts[7].trim().to_string();

            if timestamp_str.is_empty() || direction.is_empty() || amount_str.is_empty() {
                continue;
            }

            transactions.push(RawTransaction {
                timestamp_str,
                transaction_type,
                counterparty,
                item_name,
                direction,
                amount_str,
                payment_method,
                status,
            });
        } else if !line.trim().is_empty() {
            // eprintln!("Skipping malformed line (not enough columns: {}): {}", parts.len(), line);
        }
    }
    Ok(transactions)
}

fn parse_amount(amount_str: &str) -> Option<f32> {
    amount_str
        .replace(['￥', ','], "") // Remove currency symbol and commas
        .trim()
        .parse::<f32>()
        .ok()
}

fn parse_date(date_str: &str) -> Option<NaiveDate> {
    // Expects "YYYY/MM/DD HH:MM" format, we only need the date part.
    NaiveDate::parse_from_str(date_str.split_whitespace().next()?, "%Y/%m/%d").ok()
}

fn categorize_transaction(
    transaction_type: &str,
    counterparty: &str,
    item_name: &str,
    direction: &TransactionDirection,
) -> String {
    match direction {
        TransactionDirection::Expense => {
            if transaction_type.contains("商户消费") {
                if counterparty.contains("美团") || item_name.contains("美团") || counterparty.contains("饿了么") {
                    return "餐饮".to_string();
                }
                if ["肯德基", "麦当劳", "食其家", "塔斯汀", "沪上阿姨", "瑞幸", "luckin", "华莱士", "星巴克"].iter().any(|kw| counterparty.contains(kw) || item_name.contains(kw)) {
                    return "餐饮".to_string();
                }
                if item_name.contains("超市") || counterparty.contains("超市") || counterparty.contains("罗森") || item_name.contains("便利店") {
                    return "购物-日用".to_string();
                }
                if counterparty.contains("拼多多") || counterparty.contains("京东") || item_name.contains("淘宝") || item_name.contains("天猫") || item_name.contains("抖音电商") {
                    return "购物-网购".to_string();
                }
                if transaction_type.contains("交通") || item_name.contains("出行") || counterparty.contains("滴滴") || counterparty.contains("高德打车") || counterparty.contains("中铁网络") || item_name.contains("公交") || item_name.contains("地铁") {
                    return "交通".to_string();
                }
                if counterparty.contains("Valve") || item_name.contains("Steam") || item_name.contains("游戏") || counterparty.contains("KTV") {
                    return "娱乐".to_string();
                }
                if item_name.contains("话费") || counterparty.contains("联通") || counterparty.contains("电信") || counterparty.contains("移动") {
                    return "通讯缴费".to_string();
                }
                 if item_name.contains("药") || counterparty.contains("药房") || counterparty.contains("医院") {
                    return "医疗健康".to_string();
                }
            } else if transaction_type.contains("微信红包") || transaction_type.contains("转账") && (item_name.contains("红包") || counterparty.contains("发出")) {
                return "社交红包".to_string();
            } else if transaction_type.contains("扫二维码付款") {
                 if item_name.contains("菜") || item_name.contains("饭") || item_name.contains("餐") || counterparty.contains("餐饮") {
                    return "餐饮".to_string();
                 }
            }
            "其他支出".to_string()
        }
        TransactionDirection::Income => {
            if transaction_type.contains("微信红包") || item_name.contains("红包") {
                return "红包收入".to_string();
            }
            if transaction_type.contains("转账") {
                 if counterparty.contains("爸爸") || counterparty.contains("妈妈") || item_name.contains("工资") || item_name.contains("薪") {
                    return "家庭/工资".to_string();
                 }
                return "转账收入".to_string();
            }
            if transaction_type.contains("退款") {
                return "退款".to_string();
            }
            "其他收入".to_string()
        }
        TransactionDirection::Neutral => "中性交易".to_string(),
    }
}

// Predefined colors for categories
pub fn get_color_for_category(category_name: &str, index: usize) -> Color32 { // Made public
    let colors = [
        Color32::from_rgb(255, 99, 71),  // Tomato
        Color32::from_rgb(60, 179, 113), // MediumSeaGreen
        Color32::from_rgb(70, 130, 180), // SteelBlue
        Color32::from_rgb(255, 215, 0),  // Gold
        Color32::from_rgb(255, 165, 0), // Orange
        Color32::from_rgb(128, 0, 128),  // Purple
        Color32::from_rgb(0, 128, 128),  // Teal
        Color32::from_rgb(210, 105, 30), // Chocolate
        Color32::from_rgb(255, 20, 147), // DeepPink
        Color32::from_rgb(0, 191, 255),  // DeepSkyBlue
        Color32::from_rgb(124, 252, 0),  // LawnGreen
        Color32::from_rgb(138, 43, 226), // BlueViolet
    ];
    // Simple hash for more varied color selection if categories exceed predefined colors
    if category_name.len() < colors.len() {
        colors[category_name.len() % colors.len()]
    } else {
        colors[index % colors.len()]
    }
}


pub fn process_and_aggregate_transactions(
    raw_transactions: &[RawTransaction],
) -> (Vec<ExpenseCategory>, Vec<ExpenseCategory>) {
    let mut expense_map: HashMap<String, f32> = HashMap::new();
    let mut income_map: HashMap<String, f32> = HashMap::new();

    for raw_tx in raw_transactions {
        let _date = match parse_date(&raw_tx.timestamp_str) { // Changed date to _date
            Some(d) => d,
            None => {
                // eprintln!("Skipping transaction due to unparseable date: {}", raw_tx.timestamp_str);
                continue;
            }
        };
        let amount = match parse_amount(&raw_tx.amount_str) {
            Some(a) => a,
            None => {
                // eprintln!("Skipping transaction due to unparseable amount: {}", raw_tx.amount_str);
                continue;
            }
        };

        let direction = match raw_tx.direction.as_str() {
            "支出" => TransactionDirection::Expense,
            "收入" => TransactionDirection::Income,
            _ => { // Includes "/" and other potential values like "不计收支" if they exist
                // Let's treat "/" as neutral for now, or if it's a specific type like "零钱通"
                if raw_tx.transaction_type.contains("零钱通") || raw_tx.transaction_type.contains("理财通") || raw_tx.transaction_type.contains("信用卡还款") {
                    TransactionDirection::Neutral
                } else {
                    // If direction is "/" but not a known neutral type, it's ambiguous.
                    // For now, we might skip it or log it.
                    // eprintln!("Skipping transaction with ambiguous direction '/': {:?}", raw_tx);
                    continue; 
                }
            }
        };
        
        if matches!(direction, TransactionDirection::Neutral) {
            continue; // Skip neutral transactions for pie charts
        }

        let category = categorize_transaction(
            &raw_tx.transaction_type,
            &raw_tx.counterparty,
            &raw_tx.item_name,
            &direction,
        );

        match direction {
            TransactionDirection::Expense => {
                *expense_map.entry(category).or_insert(0.0) += amount;
            }
            TransactionDirection::Income => {
                *income_map.entry(category).or_insert(0.0) += amount;
            }
            TransactionDirection::Neutral => {} // Already handled
        }
    }

    let mut expense_categories = Vec::new();
    for (i, (name, amount)) in expense_map.into_iter().enumerate() {
        let color = get_color_for_category(&name, i); // Get color before name is moved
        expense_categories.push(ExpenseCategory {
            name, // name is moved here
            amount,
            color,
        });
    }

    let mut income_categories = Vec::new();
    for (i, (name, amount)) in income_map.into_iter().enumerate() {
        let color = get_color_for_category(&name, i + expense_categories.len()); // Get color before name is moved
        income_categories.push(ExpenseCategory {
            name, // name is moved here
            amount,
            color,
        });
    }

    (expense_categories, income_categories)
}
