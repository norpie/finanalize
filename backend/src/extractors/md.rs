use crate::prelude::*;
use async_trait::async_trait;
use scraper::{Html, Selector};
use tokio::task;
use super::{Content, ContentExtract};
use super::FileType;

pub struct MarkdownExtractor;

#[async_trait]
impl ContentExtract for MarkdownExtractor {
    async fn extract(&self, file: FileType) -> Result<Vec<Content>> {
        let FileType::MarkDown(buffer) = file else {
            return Err(FinanalizeError::ParseError("Invalid input type".to_string()));
        };

        // Perform the HTML-to-Markdown conversion in a blocking thread
        let markdown = task::spawn_blocking(move || {
            let document = Html::parse_document(&buffer);
            let selector = Selector::parse("body *")
                .map_err(|_| FinanalizeError::ParseError("Selector parse error".to_string()))?;

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
        Ok(vec![Content::MarkDown(markdown)])
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

        let buffer = fs::read_to_string(&test_html_path)
            .await
            .expect("Failed to read test HTML file");

        let extractor = MarkdownExtractor;

        let result = extractor
            .extract(FileType::MarkDown(buffer))
            .await
            .expect("Markdown extraction failed");

        assert_eq!(result.len(), 1);
        if let Content::MarkDown(markdown) = &result[0] {
            assert!(markdown.contains("# Title"));
            assert!(markdown.contains("## Subtitle"));
            assert!(markdown.contains("- Item 1"));
            assert!(markdown.contains("- Item 2"));
            assert!(markdown.contains("This is a paragraph."));
        } else {
            panic!("Expected Content::MarkDown variant");
        }
    }
}
