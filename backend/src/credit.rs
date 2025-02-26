use chrono::{DateTime, Utc};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::VecDeque;
use surrealdb::sql::{Thing, Id};
use uuid::Uuid;

use crate::prelude::FinanalizeError;
use crate::db::DB; // Import DB connection

// ----------- Wallet Struct --------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub id: Thing,
    #[serde(serialize_with = "serialize_transactions", deserialize_with = "deserialize_transactions")]
    pub transactions: VecDeque<WalletTransaction>,
}

// ----------- Wallet Transactions ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WalletTransaction {
    Credit {
        description: String,
        amount: Decimal,
    },
    Report(ReportBill),
}

// ----------- ReportBill Struct ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportBill {
    pub id: Thing,
    pub report_id: String,
    pub tokens_used: i32,
    pub total_cost: Decimal,
    pub api_type: String,
    pub created_at: DateTime<Utc>,
}

impl Wallet {
    pub fn new() -> Self {
        let wallet_id = Thing::from(("wallet".to_string(), Uuid::new_v4().to_string()));
        Self {
            id: wallet_id,
            transactions: VecDeque::new(),
        }
    }

    /// Calculates the balance by iterating over all transactions.
    pub fn calculate_balance(&self) -> Decimal {
        let mut balance = Decimal::ZERO;
        for transaction in &self.transactions {
            match transaction {
                WalletTransaction::Credit { amount, .. } => balance += amount,
                WalletTransaction::Report(bill) => balance -= bill.total_cost,
            }
        }
        balance
    }

    /// Adds a credit transaction.
    pub fn add_credits(&mut self, amount: Decimal) {
        self.transactions.push_back(WalletTransaction::Credit {
            description: "Credit Addition".to_string(),
            amount,
        });
    }

    /// Uses tokens, deducting the total cost from the calculated balance.
    pub fn use_tokens(&mut self, tokens: i32, api_type: &str, report_id: &str) -> Result<Decimal, FinanalizeError> {
        let cost_per_token = match api_type {
            "OpenAI" => Decimal::new(25, 1),
            "Anthropic" => Decimal::new(22, 1),
            "Google" => Decimal::new(20, 1),
            "Azure" => Decimal::new(18, 1),
            _ => Decimal::ZERO,
        };

        let total_cost = cost_per_token * Decimal::from_i32(tokens).unwrap();

        if self.calculate_balance() < total_cost {
            return Err(FinanalizeError::NotFound); // You might want to define a more specific error.
        }

        let report_bill = ReportBill {
            id: Thing::from(("report_bill".to_string(), Uuid::new_v4().to_string())),
            report_id: report_id.to_string(),
            tokens_used: tokens,
            total_cost,
            api_type: api_type.to_string(),
            created_at: Utc::now(),
        };

        self.transactions.push_back(WalletTransaction::Report(report_bill));
        Ok(total_cost)
    }

    /// Saves the wallet to the database.
    pub async fn save_wallet_to_db(&self) -> crate::prelude::Result<()> {
        let db = DB.get().unwrap();

        let wallet_data = serde_json::json!({
            "transactions": self.transactions.iter()
                .map(|t| serde_json::to_value(t).unwrap())
                .collect::<Vec<_>>()
        });

        let wallet_id = format!("wallet:{}", self.id.id);
        let response: Vec<serde_json::Value> = db.update(wallet_id)
            .content(wallet_data)
            .await
            .map_err(FinanalizeError::from)?;

        if response.is_empty() {
            return Err(FinanalizeError::NotFound);
        }

        Ok(())
    }

    /// Loads the wallet from the database.
    pub async fn load_wallet_from_db(wallet_id: &str) -> Result<Wallet, FinanalizeError> {
        let db = DB.get().unwrap();

        let wallet_data: Vec<serde_json::Value> = db
            .select(format!("wallet:{}", wallet_id))
            .await
            .map_err(FinanalizeError::from)?;

        let wallet_json = wallet_data.into_iter().next()
            .ok_or(FinanalizeError::NotFound)?;

        let wallet: Wallet = serde_json::from_value(wallet_json)
            .map_err(FinanalizeError::from)?;

        Ok(wallet)
    }

    /// Relates the wallet to a user.
    pub async fn relate_wallet_to_user(wallet_id: &str, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let db = DB.get().unwrap();

        let query = format!(
            "RELATE user:{}->has_wallet->wallet:{} RETURN AFTER;",
            user_id, wallet_id
        );

        db.query(query).await?;

        Ok(())
    }

    /// Relates a report to a bill.
    pub async fn relate_report_to_bill(report_id: &str, bill_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let db = DB.get().unwrap();

        let query = format!(
            "RELATE report:{}->has_bill->report_bill:{} RETURN AFTER;",
            report_id, bill_id
        );

        db.query(query).await?;

        Ok(())
    }

    /// Generates a wallet bill.
    pub fn generate_wallet_bill(&self) -> String {
        let mut bill_output = String::new();
        bill_output.push_str("---------------------\n");
        bill_output.push_str("Wallet Bill:\n");
        bill_output.push_str("---------------------\n");

        let mut total_spent = Decimal::ZERO;
        for transaction in &self.transactions {
            match transaction {
                WalletTransaction::Report(bill) => {
                    total_spent += bill.total_cost;
                    bill_output.push_str(&format!(
                        "{} Token Generation: {} tokens generated, cost: {:.2} credits, API: {}\n",
                        bill.api_type, bill.tokens_used, bill.total_cost, bill.api_type
                    ));
                }
                WalletTransaction::Credit { amount, .. } => {
                    bill_output.push_str(&format!("Credit Addition: {:.2} credits added\n", amount));
                }
            }
        }
        bill_output.push_str("---------------------\n");
        let remaining = self.calculate_balance();
        bill_output.push_str(&format!("Total remaining credits: {:.2}\n", remaining));
        bill_output.push_str("---------------------\n");

        bill_output
    }
}

fn serialize_transactions<S>(transactions: &VecDeque<WalletTransaction>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let vec: Vec<WalletTransaction> = transactions.iter().cloned().collect();
    vec.serialize(serializer)
}

fn deserialize_transactions<'de, D>(deserializer: D) -> Result<VecDeque<WalletTransaction>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let vec: Vec<WalletTransaction> = Vec::deserialize(deserializer)?;
    Ok(VecDeque::from(vec))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_wallet_transactions() {
        let mut wallet = Wallet::new();
        // Initially, the wallet balance is zero.
        assert_eq!(wallet.calculate_balance(), Decimal::ZERO);

        wallet.add_credits(Decimal::new(500, 0));

        let report_1 = wallet.use_tokens(20, "OpenAI", "report_1").unwrap_or(Decimal::ZERO);
        let report_2 = wallet.use_tokens(30, "Anthropic", "report_1").unwrap_or(Decimal::ZERO);
        let report_3 = wallet.use_tokens(15, "Google", "report_2").unwrap_or(Decimal::ZERO);

        let expected_balance = Decimal::new(500, 0) - report_1 - report_2 - report_3;
        assert_eq!(wallet.calculate_balance(), expected_balance);

        let wallet_bill = wallet.generate_wallet_bill();
        println!("\nWallet Bill:\n{}", wallet_bill);
        assert!(wallet_bill.contains("Token Generation"));
        assert!(wallet_bill.contains("Total remaining credits"));
    }
}

















