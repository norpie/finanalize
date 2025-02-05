use crate::prelude::*;
use chromiumoxide::{Browser, BrowserConfig, Handler};
use std::time::{Duration, Instant};
use futures_util::StreamExt;
use tokio::spawn;
use tokio::time::{sleep, timeout};

pub struct BrowserWrapper {
    browser: Browser,
    handler: Handler,
}
pub async fn setup_browser() -> Result<BrowserWrapper>{
    let browser_config = BrowserConfig::builder().with_head().build().unwrap();
    let (browser, handler) = Browser::launch(browser_config).await?;
    let wrapper = BrowserWrapper {browser, handler};
    Ok(wrapper)
}
pub async fn scrape_page(
    wrapper: BrowserWrapper,
    url: String,
) -> Result<String> {
    let browser = wrapper.browser;
    let mut handler = wrapper.handler;
    let handle = spawn(async move {
        loop {
                if let Ok(Some(_event)) = timeout(Duration::from_secs(2), handler.next()).await {
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
        let wrapper = setup_browser().await.unwrap();
        let url = "https://example.com".to_string();
        let result: String = scrape_page(wrapper, url)
            .await
            .unwrap()
            .to_string();
        println!("{}", result);
        assert!(result.contains("Example Domain"));
    }
}
