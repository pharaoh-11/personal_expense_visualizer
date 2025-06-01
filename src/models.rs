use chrono::NaiveDate;
use eframe::egui::Color32;

#[derive(Clone, Debug)]
pub struct ExpenseCategory {
    pub name: String,
    pub amount: f32,
    pub color: Color32,
}

#[derive(Debug, Clone)]
pub struct RawTransaction {
    pub timestamp_str: String,    // 交易时间 (e.g., "2025/5/31 11:02")
    pub transaction_type: String, // 交易类型
    pub counterparty: String,     // 交易对方
    pub item_name: String,        // 商品
    pub direction: String,        // 收/支
    pub amount_str: String,       // 金额(元) (e.g., "￥16.00")
    pub payment_method: String,   // 支付方式
    pub status: String,           // 当前状态
    // We can ignore 交易单号, 商户单号, 备注 for now unless needed
}

#[derive(Debug, Clone)]
pub enum TransactionDirection {
    Income,
    Expense,
    Neutral, // For transactions like transfers that are neither income nor expense for categorization
}

#[derive(Debug, Clone)]
pub struct ProcessedTransaction {
    pub date: NaiveDate,
    pub category: String, // Determined by our logic
    pub amount: f32,
    pub direction: TransactionDirection,
    pub original_item_name: String, // Keep original item name for reference or detailed view
    pub original_counterparty: String, // Keep original counterparty for reference
    pub original_transaction_type: String, // Keep original transaction type
}
