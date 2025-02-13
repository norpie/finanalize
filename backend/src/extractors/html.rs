use crate::prelude::*;
use async_trait::async_trait;
use scraper::{Html, Selector, ElementRef};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::task;

use super::text::TextExtractor;
use super::ContentExtract;

pub struct HTMLExtractor;

#[async_trait]
impl ContentExtract for HTMLExtractor {
    async fn extract(&self, file_path: &str) -> Result<Vec<String>> {
        let mut file = File::open(file_path).await.map_err(FinanalizeError::from)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .await
            .map_err(FinanalizeError::from)?;

        // Move parsing into a blocking thread to avoid `Send` issues
        let extracted_text = task::spawn_blocking(move || {
            let document = Html::parse_document(&buffer);

            // Select all `div` and `article` elements inside `<body>`
            let selector = Selector::parse("body > div, body > article").map_err(|err| {
                FinanalizeError::ParseError(format!("{:?}", err))
            })?;

            let mut text = String::new();

            for element in document.select(&selector) {
                // Check if the element is NOT inside ignored tags
                if !is_inside_ignored_section(element) {
                    text.push_str(&element.text().collect::<Vec<_>>().join(" "));
                    text.push('\n');
                }
            }

            if text.is_empty() {
                return Err(FinanalizeError::NotFound);
            }

            Ok(text)
        })
        .await
        .map_err(|_| FinanalizeError::InternalServerError)??; // Handle potential task panics

        TextExtractor {}.extract(&extracted_text).await
    }
}

/// Function to check if an element is inside a header, nav, aside, or footer
fn is_inside_ignored_section(element: ElementRef) -> bool {
    let mut parent = element.parent();
    
    while let Some(node) = parent {
        if let Some(el) = ElementRef::wrap(node) {
            match el.value().name() {
                "nav" | "aside" | "header" | "footer" => return true,
                _ => {}
            }
        }
        parent = node.parent();
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_html_extraction() {
        let test_html = r#"
        <html>
            <body>
                <header><h1>Header Title</h1></header>
                <nav>Navigation Links</nav>
                <aside>Sidebar Content</aside>
                <div>Main Content</div>
                <article>Another Important Section</article>
                <footer>Footer Information</footer>
            </body>
        </html>
        "#;
        
        let file_path = "test.html";
        tokio::fs::write(file_path, test_html)
            .await
            .expect("Kon test.html niet maken");

        let extractor = HTMLExtractor;
        let result = extractor.extract(file_path).await.unwrap();

        println!("Extracted text: {:?}", result);

        assert!(result.iter().any(|text| text.contains("Main Content")));
        assert!(result.iter().any(|text| text.contains("Another Important Section")));
        
        // Ensure unwanted elements are not included
        assert!(!result.iter().any(|text| text.contains("Header Title")));
        assert!(!result.iter().any(|text| text.contains("Navigation Links")));
        assert!(!result.iter().any(|text| text.contains("Sidebar Content")));
        assert!(!result.iter().any(|text| text.contains("Footer Information")));

        tokio::fs::remove_file(file_path)
            .await
            .expect("Kon test.html niet verwijderen");
    }
}