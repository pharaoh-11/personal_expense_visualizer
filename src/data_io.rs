use crate::models::RawTransaction;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

const EXPECTED_COLUMNS: usize = 11; // Based on the header: 交易时间 -> 备注

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
            continue; // Skip this delimiter line
        }

        if !start_parsing_data {
            continue; // Skip metadata lines at the beginning
        }

        if !header_skipped {
            // This is the header line, e.g., "交易时间\t交易类型..."
            // We can optionally validate it here if needed.
            header_skipped = true;
            continue; // Skip the header line itself
        }

        // Now we are parsing actual data lines
        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() >= 8 { // We need at least up to "当前状态"
            // Ensure all parts are trimmed of potential extra quotes or spaces, especially for amounts
            let timestamp_str = parts[0].trim().to_string();
            let transaction_type = parts[1].trim().to_string();
            let counterparty = parts[2].trim().to_string();
            let item_name = parts[3].trim().to_string();
            let direction = parts[4].trim().to_string();
            let amount_str = parts[5].trim().to_string();
            let payment_method = parts[6].trim().to_string();
            let status = parts[7].trim().to_string();
            // Columns 8, 9, 10 (交易单号, 商户单号, 备注) are ignored for now

            // Basic validation: ensure critical fields are not empty if necessary
            if timestamp_str.is_empty() || direction.is_empty() || amount_str.is_empty() {
                // eprintln!("Skipping malformed line (empty critical fields): {}", line);
                continue;
            }
            
            // Further check for "中性交易" in summary, but individual lines might not reflect this directly.
            // The "direction" field ("收/支" or "/") should be primary.
            if direction == "/" && transaction_type.contains("零钱通") { // Example of neutral transaction
                 // For now, we still parse it as Raw, categorization will handle it.
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
            // Line doesn't have enough parts but is not empty, could be a footer or malformed.
            // eprintln!("Skipping malformed line (not enough columns: {}): {}", parts.len(), line);
        }
    }

    Ok(transactions)
}

// TODO: Implement categorize_transaction, process_transactions, aggregate_expenses_by_category
