use chrono::{DateTime, Utc};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::VecDeque;
use surrealdb::sql::Thing;
use derive_more::From;

use crate::db::DB; // Import DB connection

// ----------- Wallet Struct --------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub balance: Decimal,
    #[serde(serialize_with = "serialize_transactions", deserialize_with = "deserialize_transactions")]
    pub transactions: VecDeque<WalletTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDBUser {
    pub id: String,
    pub balance: String,
    pub transactions: Vec<WalletTransaction>,
}

impl Wallet {
    pub fn new(initial_balance: Decimal) -> Self {
        Self {
            balance: initial_balance,
            transactions: VecDeque::new(),
        }
    }

    pub fn add_credits(&mut self, amount: Decimal) {
        self.balance += amount;
        self.transactions.push_back(WalletTransaction::Credit {
            description: "Credit Addition".to_string(),
            amount,
        });
    }

    pub fn use_tokens(&mut self, tokens: i32, api_type: &str, report_id: &str) -> Decimal {
        let cost_per_token = match api_type {
            "OpenAI" => Decimal::new(25, 1),
            "Anthropic" => Decimal::new(22, 1),
            "Google" => Decimal::new(20, 1),
            "Azure" => Decimal::new(18, 1),
            _ => Decimal::ZERO,
        };

        let total_cost = cost_per_token * Decimal::from_i32(tokens).unwrap();

        if self.balance < total_cost {
            panic!("Insufficient wallet balance!");
        }

        self.balance -= total_cost;

        let report_bill = ReportBill {
            report_id: report_id.to_string(),
            tokens_used: tokens,
            total_cost,
            api_type: api_type.to_string(),
            created_at: Utc::now(),
        };

        self.transactions.push_back(WalletTransaction::Report(report_bill));
        total_cost
    }

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

                    let report_bill_json = serde_json::to_string(bill).unwrap();
                    println!("\n[INFO] Report Bill (JSON format):\n{}", report_bill_json);
                }
                WalletTransaction::Credit { amount, .. } => {
                    bill_output.push_str(&format!("Credit Addition: {:.2} credits added\n", amount));
                }
            }
        }

        bill_output.push_str("---------------------\n");
        bill_output.push_str(&format!("Total remaining credits: {:.2}\n", self.balance));
        bill_output.push_str("---------------------\n");

        bill_output
    }

    // Integrating wallet changes with SurrealDB
    pub async fn save_wallet_to_db(&self, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let db = DB.get().unwrap(); // Ensure DB connection is established
    
        // Serialize wallet data
        let wallet_data = serde_json::json!( {
            "balance": self.balance.to_string(),
            "transactions": self.transactions.iter().map(|t| serde_json::to_value(t).unwrap()).collect::<Vec<_>>()
        });
    
        // Properly format the SurrealDB query
        let query = format!("user:{}", user_id); 
    
        // Explicitly specify the result type
        let result: Result<Vec<serde_json::Value>, surrealdb::Error> = db.update(query).content(wallet_data).await;
    
        // Handle result error properly
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)), // Convert SurrealDB error into Box<dyn std::error::Error>
        }
    }
    
    
    
    // Load the wallet from DB
    pub async fn load_wallet_from_db(user_id: &str) -> Result<Wallet, Box<dyn std::error::Error>> {
        let db = DB.get().unwrap(); // Ensure DB is connected
        
        // Retrieve the user data from the database
        let user_data: Vec<serde_json::Value> = db.select(format!("user WHERE id = '{}'", user_id)).await?;
    
        // Check if user_data is an array or a single object
        if let Some(user_json) = user_data.first() {
            // Deserialize the first element of the array to SurrealDBUser struct
            let surreal_user: SurrealDBUser = serde_json::from_value(user_json.clone())?;
    
            // Retrieve wallet balance from the user data
            let wallet_balance: Decimal = surreal_user.balance.parse().map_err(|e| format!("Failed to parse balance: {}", e))?;
    
            // Deserialize transactions into WalletTransaction enum
            let transactions: VecDeque<WalletTransaction> = surreal_user
                .transactions
                .into_iter()
                .map(|txn| serde_json::from_value(serde_json::to_value(txn).unwrap()).unwrap()) // Convert before deserializing
                .collect();
    
            // Return the Wallet object
            Ok(Wallet {
                balance: wallet_balance,
                transactions,
            })
        } else {
            Err("No user data found.".into())
        }
    }
    
}

impl From<Vec<WalletTransaction>> for SurrealDBUser {
    fn from(transactions: Vec<WalletTransaction>) -> Self {
        let balance = "0"; 
        SurrealDBUser {
            id: "default_id".to_string(),
            balance: balance.to_string(),
            transactions,
        }
    }
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
    pub report_id: String,
    pub tokens_used: i32,
    pub total_cost: Decimal,
    pub api_type: String,
    pub created_at: DateTime<Utc>,
}

// ----------- ApiUsage Struct ----------
#[derive(Debug, Clone)]
struct ApiUsage {
    api_type: String,
    tokens_used: i32,
}

// ----------- Serialization Helpers ----------
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

// ----------- User Struct with Wallet -----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
}

// ----------- UserWithWallet Struct -----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithWallet {
    pub user: User,       // Embed the existing User struct
    pub wallet: Wallet,   // Add the Wallet field
}

impl UserWithWallet {
    pub fn new(user: User, wallet: Wallet) -> Self {
        Self { user, wallet }
    }

    pub fn add_credits(&mut self, amount: Decimal) {
        self.wallet.add_credits(amount);
    }

    pub fn use_tokens(&mut self, tokens: i32, api_type: &str, report_id: &str) -> Decimal {
        self.wallet.use_tokens(tokens, api_type, report_id)
    }
}

// ----------- Tests ----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_transactions() {
        let mut user = UserWithWallet {
            user: User {
                id: "user_123".to_string(),
                email: "user@example.com".to_string(),
                password: "password".to_string(),
            },
            wallet: Wallet::new(Decimal::new(1000, 0)),
        };

        user.add_credits(Decimal::new(500, 0));

        let api_usages = vec![
            ApiUsage {
                api_type: "OpenAI".to_string(),
                tokens_used: 20,
            },
            ApiUsage {
                api_type: "Anthropic".to_string(),
                tokens_used: 30,
            },
        ];

        let report_1 = user.use_tokens(20, "OpenAI", "report_1");
        let report_2 = user.use_tokens(30, "Anthropic", "report_1");
        let single_report = user.use_tokens(15, "Google", "report_2");

        let expected_balance = Decimal::new(1000, 0) + Decimal::new(500, 0)
            - report_1 - report_2 - single_report;

        assert_eq!(user.wallet.balance, expected_balance);

        let wallet_bill = user.wallet.generate_wallet_bill();
        println!("\nWallet Bill:\n{}", wallet_bill);
        assert!(wallet_bill.contains("Token Generation"));
        assert!(wallet_bill.contains("Total remaining credits"));
    }
}













