use crate::prelude::*;
use async_trait::async_trait;
use scraper::{ElementRef, Html, Selector};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::task;
use super::ContentExtract;
use super::Content;

pub struct MarkdownExtractor;

#[async_trait]
impl ContentExtract for MarkdownExtractor {
    async fn extract(&self, file_path: &str) -> Result<Vec<Content>> {
        let mut file = File::open(file_path).await.map_err(FinanalizeError::from)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .await
            .map_err(FinanalizeError::from)?;

        // Perform the HTML-to-Markdown conversion in a blocking thread
        let markdown = task::spawn_blocking(move || {
            let document = Html::parse_document(&buffer);
            let selector = Selector::parse("body *").map_err(|_| FinanalizeError::ParseError("Selector parse error".to_string()))?;

            let mut md = String::new();
            for element in document.select(&selector) {
                let tag_name = element.value().name();
                let text = element.text().collect::<Vec<_>>().join(" ");

                match tag_name {
                    "h1" => md.push_str(&format!("# {}\n\n", text)),
                    "h2" => md.push_str(&format!("## {}\n\n", text)),
                    "h3" => md.push_str(&format!("### {}\n\n", text)),
                    "p" => md.push_str(&format!("{}\n\n", text)),
                    "li" => md.push_str(&format!("- {}\n", text)),
                    _ => md.push_str(&format!("{}\n", text)),
                }
            }

            Ok(md.trim().to_string()) as Result<String>
        })
        .await
        .map_err(|_| FinanalizeError::InternalServerError)??;

        if markdown.is_empty() {
            return Err(FinanalizeError::NotFound);
        }

        // Wrap the generated Markdown in `Content::MarkDown`
        let content = vec![Content::MarkDown(markdown)];

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_html_to_markdown_conversion() {
        let dir = tempdir().unwrap();
        let test_html_path = dir.path().join("test.html");

        let sample_html = r#"
        <html>
            <body>
                <h1>Title</h1>
                <p>This is a paragraph.</p>
                <ul>
                    <li>Item 1</li>
                    <li>Item 2</li>
                </ul>
                <h2>Subtitle</h2>
            </body>
        </html>
        "#;

        fs::write(&test_html_path, sample_html)
            .await
            .expect("Failed to create test HTML file");

        let extractor = MarkdownExtractor;

        let result = extractor.extract(test_html_path.to_str().unwrap()).await.unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            Content::MarkDown(markdown) => {
                assert!(markdown.contains("# Title"));
                assert!(markdown.contains("## Subtitle"));
                assert!(markdown.contains("- Item 1"));
                assert!(markdown.contains("- Item 2"));
                assert!(markdown.contains("This is a paragraph."));
            }
            _ => panic!("Expected Content::MarkDown variant"),
        }

        fs::remove_file(test_html_path)
            .await
            .expect("Failed to delete test HTML file");
    }
}
