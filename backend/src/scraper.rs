use crate::prelude::*;
use fantoccini::{Client, ClientBuilder};
use serde::Serialize;
use serde_json::json;
use tokio::sync::OnceCell;

pub static BROWSER: OnceCell<Client> = OnceCell::const_new();

pub async fn get_or_init_browser() -> Result<Client> {
    if BROWSER.get().is_none() {
        BROWSER
            .set(
                ClientBuilder::native()
                    .capabilities(
                        json!({
                            "moz:firefoxOptions": {
                                "args": ["--headless"]
                            }
                        })
                        .as_object()
                        .unwrap()
                        .clone(),
                    )
                    .connect("http://localhost:4444")
                    .await?,
            )
            .map_err(|_| FinanalizeError::NotFound)?;
        println!("Browser initialized");
    }
    println!("Returning browser");
    Ok(BROWSER.get().unwrap().clone())
}

#[derive(Serialize)]
struct Capabilities {
    #[serde(rename = "moz:firefoxOptions")]
    firefox_options: FFOptions,
}

#[derive(Serialize)]
struct FFOptions {
    args: Vec<String>,
}

pub async fn scrape_page(url: String) -> Result<String> {
    let c = get_or_init_browser().await?;
    c.goto(&url).await?;
    let source = c.source().await?;
    Ok(source)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test_scrape_page() {
        let url = "https://github.com".to_string();
        let result: String = scrape_page(url).await.unwrap().to_string();
        get_or_init_browser().await.unwrap().close().await.unwrap();
        assert!(result.contains("GitHub"));
    }
}
