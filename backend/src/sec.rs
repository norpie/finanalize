use reqwest;
use serde::{Deserialize, Serialize};
use actix_web::{get, web, HttpResponse, Responder};
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize, Serialize)]
struct FilingData {
    cik: String,
    ticker: Option<String>,
    filings: Filings,
}

#[derive(Debug, Deserialize, Serialize)]
struct Filings {
    recent: RecentFilings,
}

#[derive(Debug, Deserialize, Serialize)]
struct RecentFilings {
    form: Vec<String>,
    #[serde(rename = "filingDate")] // Matches JSON field in raw document
    filing_date: Vec<String>,
    #[serde(rename = "primaryDocument")] // Matches JSON field
    primary_document: Vec<String>,
    #[serde(rename = "accessionNumber")] // Matches JSON field
    accession_number: Vec<String>,
}

async fn get_cik_from_ticker(ticker: &str) -> Option<String> {
    let url = "https://www.sec.gov/files/company_tickers.json";
    let user_agent = "FinAnalizeBot/1.0 (nguijoel.bryana@student.ehb.be)";

    let response = reqwest::Client::new()
        .get(url)
        .header("User-Agent", user_agent)
        .send()
        .await
        .ok()?
        .json::<serde_json::Value>()
        .await
        .ok()?;

    dbg!(&response); // Debug output for JSON response

    let companies = response.as_object()?; // JSON is an object (hashmap geen array)

    for (_key, company) in companies.iter() {
        if let (Some(cik_str), Some(company_ticker)) =
            (company.get("cik_str")?.as_u64(), company.get("ticker")?.as_str())
        {
            if company_ticker.eq_ignore_ascii_case(ticker) {
                return Some(format!("{:0>10}", cik_str));
            }
        }
    }

    None
}

async fn get_latest_filings(cik: &str) -> Option<FilingData> {
    sleep(Duration::from_secs(1)).await;   
    let url = format!("https://data.sec.gov/submissions/CIK{}.json", cik);
    let user_agent = "FinAnalizeBot/1.0 (nguijoel.bryana@student.ehb.be)";

    let response = reqwest::Client::new()
        .get(&url)
        .header("User-Agent", user_agent)
        .send()
        .await
        .ok()?;

    let text = response.text().await.ok()?; // Get raw response as text


    let filings: FilingData = serde_json::from_str(&text).ok()?; // Try to deserialize
    Some(filings)
}

#[get("/api/v1/sec/{ticker}")]
pub async fn fetch_sec_filings(path: web::Path<String>) -> impl Responder {
    let ticker = path.into_inner();
    
    match get_cik_from_ticker(&ticker).await {
        Some(cik) => {
            match get_latest_filings(&cik).await {
                Some(data) => HttpResponse::Ok().json(data),
                None => HttpResponse::InternalServerError().body("Error retrieving filings"),
            }
        }
        None => HttpResponse::NotFound().body("Ticker not found"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_cik_from_ticker() -> Result<(), Box<dyn std::error::Error>> {
        let json_str = r#"
        {
            "0": {"cik_str": 320193, "ticker": "AAPL", "title": "Apple Inc."},
            "1": {"cik_str": 1045810, "ticker": "NVDA", "title": "NVIDIA CORP"},
            "2": {"cik_str": 789019, "ticker": "MSFT", "title": "MICROSOFT CORP"}
        }
        "#;
    
        let response: serde_json::Value = serde_json::from_str(json_str)?;
        let companies = response.as_object().ok_or("JSON is not an object")?;
    
        let mut cik = None;
        for (_key, company) in companies.iter() {
            if let (Some(cik_str), Some(company_ticker)) =
                (company.get("cik_str").and_then(|v| v.as_u64()), company.get("ticker").and_then(|v| v.as_str()))
            {
                if company_ticker.eq_ignore_ascii_case("AAPL") {
                    cik = Some(format!("{:0>10}", cik_str));
                    break;
                }
            }
        }
        
        // Print output for debugging
        println!("Extracted CIK: {:?}", cik);

        assert_eq!(cik, Some("0000320193".to_string()));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_latest_filings() {
        let cik = "0000320193"; // CIK for Apple
        let filings = get_latest_filings(cik).await;
        
        // Print output to see the actual filings
        println!("Latest Filings: {:?}", filings);

        assert!(filings.is_some());
    }
}



