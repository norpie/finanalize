use std::collections::VecDeque;

#[derive(Debug)]
struct Item {
    description: String,
    cost_per_token: i32,
    tokens_generated: i32,
    api_type: String,
    exchange_rate: f32,
}

#[derive(Debug)]
struct Transaction {
    item: Item,
    total_cost: i32,
}

#[derive(Debug)]
struct Ledger {
    transactions: VecDeque<Transaction>,
}

impl Ledger {
    fn new() -> Self {
        Self {
            transactions: VecDeque::new(),
        }
    }

    fn add_transaction(&mut self, item: Item, total_cost: i32) {
        self.transactions.push_back(Transaction { item, total_cost });
    }

    fn generate_token_bill(&self) -> String {
        let mut bill = String::from("\nToken Generation bill:\n---------------------\n");
        for transaction in &self.transactions {
            if transaction.item.cost_per_token > 0 {
                bill.push_str(&format!(
                    "{}: {} tokens gegenereerd, kosten: {} credits ({} credits/token), Rate: {:.2}\n",
                    transaction.item.description,
                    transaction.item.tokens_generated,
                    transaction.total_cost,
                    transaction.item.cost_per_token,
                    transaction.item.exchange_rate
                ));
            }
        }
        bill.push_str("---------------------\n");
        bill
    }

    fn generate_wallet_bill(&self) -> String {
        let mut total_credits = 0;
        let mut bill = String::from("\nWallet bill:\n---------------------\n");

        for transaction in &self.transactions {
            if transaction.item.cost_per_token == 0 {
                bill.push_str(&format!(
                    "{}: {} credits toegevoegd\n",
                    transaction.item.description, transaction.total_cost
                ));
                total_credits += transaction.total_cost;
            } else {
                bill.push_str(&format!(
                    "{}: {} tokens gegenereerd, kosten: {} credits ({} credits/token), Rate: {:.2}\n",
                    transaction.item.description,
                    transaction.item.tokens_generated,
                    transaction.total_cost,
                    transaction.item.cost_per_token,
                    transaction.item.exchange_rate
                ));
                total_credits -= transaction.total_cost;
            }
        }

        bill.push_str("---------------------\n");
        bill.push_str(&format!("Totaal resterende credits: {}\n", total_credits));
        bill
    }

    fn calculate_total(&self) -> i32 {
        self.transactions.iter().map(|t| t.total_cost).sum()
    }
}

#[derive(Debug)]
struct Wallet {
    credits: i32,
    ledger: Ledger,
}

impl Wallet {
    fn new(credits: i32) -> Self {
        Self {
            credits,
            ledger: Ledger::new(),
        }
    }

    fn add_credits(&mut self, amount: i32) {
        self.credits += amount;
        let item = Item {
            description: "Credit Toevoeging".to_string(),
            cost_per_token: 0,
            tokens_generated: 0,
            api_type: "N/A".to_string(),
            exchange_rate: 0.0,
        };
        self.ledger.add_transaction(item, amount);
        println!("{} credits toegevoegd aan wallet.", amount);
    }

    fn use_tokens(&mut self, description: &str, tokens: i32, api_type: &str, exchange_rate: f32) {
        let total_cost = (tokens as f32 * exchange_rate).round() as i32;
        if self.credits >= total_cost {
            let item = Item {
                description: description.to_string(),
                cost_per_token: total_cost / tokens,
                tokens_generated: tokens,
                api_type: api_type.to_string(),
                exchange_rate,
            };
            self.credits -= total_cost;
            self.ledger.add_transaction(item, total_cost);
            println!("{} tokens gegenereerd voor {} credits.", description, total_cost);
        } else {
            println!("Niet genoeg credits voor token generatie.");
        }
    }

    fn print_token_bill(&self) {
        println!("{}", self.ledger.generate_token_bill());
    }

    fn print_wallet_bill(&self) {
        println!("{}", self.ledger.generate_wallet_bill());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tokens_with_exchange_rate() {
        let mut wallet = Wallet::new(100);
        wallet.use_tokens("OpenAI API", 10, "OpenAI", 2.5);
        assert_eq!(wallet.credits, 75); // 10 tokens * 2.5 = 25 credits
    }

    #[test]
    fn test_add_credits() {
        let mut wallet = Wallet::new(5);
        wallet.add_credits(20);
        assert_eq!(wallet.credits, 25);
    }

    #[test]
    fn test_insufficient_credits_for_tokens() {
        let mut wallet = Wallet::new(3);
        wallet.use_tokens("OpenAI API", 5, "OpenAI", 1.0);
        assert_eq!(wallet.credits, 3); // Geen transactie door te weinig credits
    }

    #[test]
    fn test_token_bill_output() {
        let mut wallet = Wallet::new(100);
        wallet.use_tokens("OpenAI API", 10, "OpenAI", 2.5);
        let bill = wallet.ledger.generate_token_bill();
        assert!(bill.contains("OpenAI API: 10 tokens gegenereerd, kosten: 25 credits (2 credits/token), Rate: 2.50"));
    }

    #[test]
    fn test_wallet_bill_output() {
        let mut wallet = Wallet::new(100);
        wallet.use_tokens("OpenAI API", 10, "OpenAI", 2.5);
        wallet.add_credits(20);
        let bill = wallet.ledger.generate_wallet_bill();
        assert!(bill.contains("OpenAI API: 10 tokens gegenereerd, kosten: 25 credits (2 credits/token), Rate: 2.50"));
        assert!(bill.contains("Credit Toevoeging: 20 credits toegevoegd"));
    }

    #[test]
    fn test_full_example_bills() {
        let mut wallet = Wallet::new(500);
        wallet.use_tokens("OpenAI API", 15, "OpenAI", 2.0);
        wallet.use_tokens("Anthropic API", 25, "Anthropic", 2.2);
        wallet.add_credits(50);
        wallet.use_tokens("Azure API", 40, "Azure", 1.8);
        wallet.add_credits(100);
        wallet.use_tokens("Google API", 30, "Google", 2.0);

        wallet.print_token_bill();
        wallet.print_wallet_bill();
    }
}












