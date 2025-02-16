use reqwest;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};

#[derive(Debug, Deserialize, Serialize)]
struct RecentFilings {
    #[serde(rename = "primaryDocument")]
    primary_document: Vec<String>,
    #[serde(rename = "accessionNumber")]
    accession_number: Vec<String>,
    form: Vec<String>, 
}

/// Fetches the CIK (Central Index Key) for a given stock ticker
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

    let companies = response.as_object()?;

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

/// Fetches only the relevant SEC filing data for a given CIK, filtering by Form 10-K
async fn get_latest_filing_links(cik: &str) -> Option<Vec<String>> {
    sleep(Duration::from_secs(1)).await;
    let url = format!("https://data.sec.gov/submissions/CIK{}.json", cik);
    let user_agent = "FinAnalizeBot/1.0 (nguijoel.bryana@student.ehb.be)";

    let response = reqwest::Client::new()
        .get(&url)
        .header("User-Agent", user_agent)
        .send()
        .await
        .ok()?;

    let text = response.text().await.ok()?;
    let filings: serde_json::Value = serde_json::from_str(&text).ok()?;

    let recent = filings.get("filings")?.get("recent")?;
    let documents: RecentFilings = serde_json::from_value(recent.clone()).ok()?;

    let base_url = "https://www.sec.gov/Archives/edgar/data";
    let mut links = Vec::new();

    // de filter voor Form 10-K filings en de laatste 3 pakken
    for (i, form) in documents.form.iter().enumerate() {
        if form == "10-K" {
            if let Some(doc) = documents.primary_document.get(i) {
                if let Some(acc_num) = documents.accession_number.get(i) {
                    let clean_acc_num = acc_num.replace("-", "");
                    let link = format!("{}/{}/{}/{}", base_url, cik, clean_acc_num, doc);
                    links.push(link);
                }
            }
        }
    }

    Some(links.into_iter().take(3).collect())
}

/// Fetches the SEC filing links for a given ticker
pub async fn get_sec_filing_links(ticker: &str) -> Result<Vec<String>, String> {
    let cik = get_cik_from_ticker(ticker).await.ok_or("Ticker not found")?;
    let links = get_latest_filing_links(&cik).await.ok_or("Error retrieving filings")?;

    if links.is_empty() {
        Err("No filings found".to_string())
    } else {
        Ok(links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_cik_from_ticker() {
        let ticker = "AAPL";
        let cik = get_cik_from_ticker(ticker).await;
        println!("Extracted CIK for {}: {:?}", ticker, cik);
        assert_eq!(cik, Some("0000320193".to_string()));
    }

    #[tokio::test]
    async fn test_get_latest_filing_links() {
        let cik = "0000320193";
        let links = get_latest_filing_links(cik).await;
        println!("Latest Filing Links: {:?}", links);
        assert!(links.is_some() && !links.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_sec_filing_links() {
        let ticker = "AAPL";
        let result = get_sec_filing_links(ticker).await;

        match &result {
            Ok(links) => {
                // Print the first 3 Form 10-K links
                let limited_links: Vec<_> = links.iter().take(3).collect();
                println!("SEC Filing Links for {} (first 3 Form 10-K): {:?}", ticker, limited_links);
                assert!(!links.is_empty());
            }
            Err(err) => {
                println!("Error: {}", err);
                assert!(false, "Test failed due to error");
            }
        }
    }
}







