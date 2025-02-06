use crate::prelude::*;
use chromiumoxide::{Browser, BrowserConfig, Handler};
use futures_util::StreamExt;
use once_cell::sync::{Lazy, OnceCell};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::spawn;
use tokio::sync::{Mutex};
use tokio::time::{sleep, timeout};

#[derive(Debug, Clone)]
pub struct BrowserWrapper {
    browser: Arc<Browser>,
    handler: Arc<Mutex<Handler>>,
}

static INSTANCE: OnceCell<BrowserWrapper> = OnceCell::new();

pub async fn setup_browser() -> Result<()> {
    let browser_config = BrowserConfig::builder().no_sandbox().build().unwrap();
    let (browser, handler) = Browser::launch(browser_config).await?;
    let wrapper = BrowserWrapper {
        browser: Arc::new(browser),
        handler: Arc::new(Mutex::new(handler)),
    };
    INSTANCE.set(wrapper).unwrap();
    Ok(())
}

pub async fn scrape_page(url: String) -> Result<String> {
    let wrapper = INSTANCE.get().unwrap();
    let browser = wrapper.browser.clone();
    let mut handler = wrapper.handler.clone();
    let handle = spawn(async move {
        let mut locked_handler = handler.lock().await;
        loop {
            if let Ok(Some(_event)) = timeout(Duration::from_secs(2), locked_handler.next()).await {
                continue;
            } else {
                dbg!("No event for 2 seconds, breaking...");
                break;
            }
        }
    });
    let page = browser.new_page(url).await?;
    page.enable_stealth_mode().await?;
    page.wait_for_navigation().await?;
    let timeout = Duration::from_secs(2);
    let deadline = Instant::now() + timeout;

    while Instant::now() < deadline {
        sleep(Duration::from_millis(100)).await;
        let result = page.evaluate("document.readyState").await;
        if let Ok(res) = result {
            if let Some(value) = res.value() {
                if value == "complete" {
                    break;
                }
            }
        }
    }
    let content = page.content().await?;
    handle.await.ok();
    Ok(content)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_scrape_page() {
        setup_browser().await.unwrap();
        sleep(Duration::from_secs(2)).await;
        let url = "https://example.com".to_string();
        let result: String = scrape_page(url).await.unwrap().to_string();
        println!("{}", result);
        assert!(result.contains("Example Domain"));
    }
}
