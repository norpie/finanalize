use chrono::{DateTime, Utc};
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::VecDeque;

// ----------- User Struct with Wallet ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub wallet: Wallet,
}

// ----------- Wallet Struct --------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub balance: Decimal,
    pub transactions: VecDeque<WalletTransaction>,
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
            "OpenAI" => Decimal::new(25, 1), // Cost per token in OpenAI (2.50)
            "Anthropic" => Decimal::new(22, 1), // Cost per token in Anthropic (2.20)
            "Google" => Decimal::new(20, 1), // Cost per token in Google (2.00)
            "Azure" => Decimal::new(18, 1),  // Cost per token in Azure (1.80)
            _ => Decimal::ZERO,
        };

        let total_cost = cost_per_token * Decimal::from_i32(tokens).unwrap();

        // Check if the user has enough balance
        if self.balance < total_cost {
            panic!("Insufficient wallet balance!"); // This should cause the test to fail
        }

        // Deduct the total cost from the wallet balance
        self.balance -= total_cost;

        // Add the transaction to the wallet
        let report_bill = ReportBill {
            report_id: report_id.to_string(),
            tokens_used: tokens,
            total_cost,
            api_type: api_type.to_string(),
            created_at: Utc::now(),
        };

        self.transactions
            .push_back(WalletTransaction::Report(report_bill));

        total_cost
    }

    pub fn generate_wallet_bill(&self) -> String {
        let mut bill_output = String::new();

        bill_output.push_str("---------------------\n");
        bill_output.push_str("Wallet Bill:\n");
        bill_output.push_str("---------------------\n");

        let mut total_spent = Decimal::ZERO;
        let mut report_bills = Vec::new();

        for transaction in &self.transactions {
            match transaction {
                WalletTransaction::Report(bill) => {
                    total_spent += bill.total_cost;
                    report_bills.push(bill.clone());

                    bill_output.push_str(&format!(
                        "{} Token Generation: {} tokens generated, cost: {:.2} credits, API: {}\n",
                        bill.api_type, bill.tokens_used, bill.total_cost, bill.api_type
                    ));

                    // Print the Report Bill as JSON
                    let report_bill_json = serde_json::to_string(bill).unwrap();
                    println!("\n[INFO] Report Bill (JSON format):\n{}", report_bill_json);
                }
                WalletTransaction::Credit { amount, .. } => {
                    bill_output
                        .push_str(&format!("Credit Addition: {:.2} credits added\n", amount));
                }
            }
        }

        bill_output.push_str("---------------------\n");
        bill_output.push_str(&format!("Total remaining credits: {:.2}\n", self.balance));
        bill_output.push_str("---------------------\n");

        bill_output
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

// ----------- Report Struct ----------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: String,
    pub user_input: String,
    pub created_at: DateTime<Utc>,
    pub api_usage: Vec<ApiUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiUsage {
    pub api_type: String,
    pub tokens_used: i32,
}

// ----------- User Functions ----------
impl User {
    pub fn generate_report_with_multiple_apis(
        &mut self,
        user_input: String,
        api_usages: Vec<ApiUsage>,
    ) -> Report {
        let report_id = format!("report_{}", Utc::now().timestamp());
        let report = Report {
            id: report_id.clone(),
            user_input,
            created_at: Utc::now(),
            api_usage: api_usages.clone(),
        };

        for api_usage in &api_usages {
            self.wallet
                .use_tokens(api_usage.tokens_used, &api_usage.api_type, &report.id);
        }

        report
    }

    pub fn get_wallet_bill(&self) -> String {
        self.wallet.generate_wallet_bill()
    }
}

// ----------- Database interaction mock (simulate a database fetch) ----------
fn fetch_user_from_database(user_id: &str) -> Option<User> {
    if user_id == "user_123" {
        Some(User {
            id: "user_123".to_string(),
            email: "user@example.com".to_string(),
            password: "password".to_string(),
            wallet: Wallet::new(Decimal::new(1400, 0)), // initial balance 200 credits
        })
    } else {
        None
    }
}

// ----------- Tests ----------
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_generate_report_with_multiple_apis_and_credits() {
        let mut user = fetch_user_from_database("user_123").unwrap();

        // Voeg credits toe aan de wallet
        user.wallet.add_credits(Decimal::new(500, 0)); // Voeg 500 credits toe

        // Tokens gebruiken voor meerdere API's
        let api_usages = vec![
            ApiUsage {
                api_type: "OpenAI".to_string(),
                tokens_used: 20,
            },
            ApiUsage {
                api_type: "Anthropic".to_string(),
                tokens_used: 30,
            },
            ApiUsage {
                api_type: "Google".to_string(),
                tokens_used: 15,
            },
        ];

        // Genereer het rapport met meerdere API's
        let report_1 = user.generate_report_with_multiple_apis(
            "Test Report with Multiple APIs".to_string(),
            api_usages,
        );

        // Verwachte balans na het rapport met meerdere API's
        let expected_balance = Decimal::new(1400, 0) + Decimal::new(500, 0) // Beginbalans + toegevoegde credits
                - (Decimal::new(25, 1) * Decimal::from_i32(20).unwrap())  // Kosten voor OpenAI
                - (Decimal::new(22, 1) * Decimal::from_i32(30).unwrap())  // Kosten voor Anthropic
                - (Decimal::new(20, 1) * Decimal::from_i32(15).unwrap()); // Kosten voor Google

        // Vergelijk de werkelijke en verwachte balans
        assert_eq!(user.wallet.balance, expected_balance);

        // Genereer wallet bill na het rapport met meerdere API's
        let wallet_bill = user.wallet.generate_wallet_bill();
        println!("\nWallet Bill after report 1:\n{}", wallet_bill);

        assert!(wallet_bill.contains("Token Generation"));
        assert!(wallet_bill.contains("Total remaining credits"));

        // Nu een rapport genereren met slechts 1 API (bijv. OpenAI)
        let api_usages_single = vec![ApiUsage {
            api_type: "OpenAI".to_string(),
            tokens_used: 10,
        }];

        let report_2 = user.generate_report_with_multiple_apis(
            "Test Report with Single API".to_string(),
            api_usages_single,
        );

        // Verwachte balans na dit rapport
        let expected_balance_after_single_api =
            expected_balance - (Decimal::new(25, 1) * Decimal::from_i32(10).unwrap()); // Kosten voor OpenAI

        // Vergelijk de werkelijke en verwachte balans
        assert_eq!(user.wallet.balance, expected_balance_after_single_api);

        // Genereer wallet bill na het rapport met 1 API
        let wallet_bill_after_single = user.wallet.generate_wallet_bill();
        println!("\nWallet Bill after report 2 (single API):\n{}", wallet_bill_after_single);

        assert!(wallet_bill_after_single.contains("Token Generation"));
        assert!(wallet_bill_after_single.contains("Total remaining credits"));
    }
}


