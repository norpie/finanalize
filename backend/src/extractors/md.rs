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
            if buffer.trim().starts_with("<") {
                // Input looks like HTML, process it as HTML
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
            } else {
                // Input is plain text; return it as-is
                Ok(buffer)
            }
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
            assert!(markdown.contains("# Title"), "Markdown should contain '# Title'");
            assert!(markdown.contains("## Subtitle"), "Markdown should contain '## Subtitle'");
            assert!(markdown.contains("- Item 1"), "Markdown should contain '- Item 1'");
            assert!(markdown.contains("- Item 2"), "Markdown should contain '- Item 2'");
            assert!(markdown.contains("This is a paragraph."), "Markdown should contain the paragraph text");
        } else {
            panic!("Expected Content::MarkDown variant");
        }
    }

    #[tokio::test]
    async fn test_plain_text_conversion() {
        let sample_text = "This is plain text input.";

        let extractor = MarkdownExtractor;

        let result = extractor
            .extract(FileType::MarkDown(sample_text.to_string()))
            .await
            .expect("Markdown extraction failed for plain text");

        assert_eq!(result.len(), 1);
        if let Content::MarkDown(markdown) = &result[0] {
            assert_eq!(markdown, sample_text, "Plain text should be returned as-is");
        } else {
            panic!("Expected Content::MarkDown variant");
        }
    }

    #[tokio::test]
    async fn test_empty_input() {
        let sample_text = "";

        let extractor = MarkdownExtractor;

        let result = extractor.extract(FileType::MarkDown(sample_text.to_string())).await;

        assert!(result.is_err(), "Empty input should return an error");
        if let Err(FinanalizeError::NotFound) = result {
            // Expected error
        } else {
            panic!("Expected NotFound error for empty input");
        }
    }
}
