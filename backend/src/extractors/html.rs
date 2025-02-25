use super::{Content, ContentExtract, FileType};
use crate::extractors::md::MarkdownExtractor;
use crate::prelude::*;
use async_trait::async_trait;
use scraper::{ElementRef, Html, Selector};
use tokio::task;

pub struct HTMLExtractor;

#[async_trait]
impl ContentExtract for HTMLExtractor {
    async fn extract(&self, file: FileType) -> Result<Vec<Content>> {
        // Ensure the input is of the correct type
        let FileType::Html(input) = file else {
            return Err(FinanalizeError::ParseError(
                "Invalid input type".to_string(),
            ));
        };

        // Move parsing into a blocking thread to avoid `Send` issues
        let extracted_texts = task::spawn_blocking(move || {
            let document = Html::parse_document(&input);

            // Select all `div` and `article` elements
            let selector = Selector::parse("div, article")
                .map_err(|err| FinanalizeError::ParseError(format!("{:?}", err)))?;

            let mut texts = vec![];

            for element in document.select(&selector) {
                // Check if the element is NOT inside ignored tags
                if !is_inside_ignored_section(element) {
                    let text = element.text().collect::<Vec<_>>().join(" ");
                    println!("Extracted text: {}", text); // Debug log
                    if !text.is_empty() {
                        texts.push(text);
                    }
                } else {
                    println!("Ignored element: {:?}", element.value().name()); // Debug log
                }
            }

            if texts.is_empty() {
                return Err(FinanalizeError::NotFound);
            }

            Ok(texts)
        })
        .await
        .map_err(|_| FinanalizeError::InternalServerError)??;

        let markdown_extractor = MarkdownExtractor;

        // Convert extracted HTML content to Markdown using MarkdownExtractor
        let result = markdown_extractor
            .extract(FileType::MarkDown(extracted_texts.join("\n")))
            .await?;

        Ok(result)
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

        let file = FileType::Html(test_html.to_string());
        let extractor = HTMLExtractor;

        match extractor.extract(file).await {
            Ok(result) => {
                println!("Extraction result: {:?}", result);
                // Ensure there is one Markdown entry
                assert_eq!(result.len(), 1);

                if let Content::MarkDown(markdown) = &result[0] {
                    // Check the Markdown output
                    assert!(
                        markdown.contains("Main Content"),
                        "Expected 'Main Content' in Markdown output"
                    );
                    assert!(
                        markdown.contains("Another Important Section"),
                        "Expected 'Another Important Section' in Markdown output"
                    );

                    // Ensure ignored sections are not included
                    assert!(
                        !markdown.contains("Header Title"),
                        "Ignored section 'Header Title' found in Markdown output"
                    );
                    assert!(
                        !markdown.contains("Navigation Links"),
                        "Ignored section 'Navigation Links' found in Markdown output"
                    );
                    assert!(
                        !markdown.contains("Sidebar Content"),
                        "Ignored section 'Sidebar Content' found in Markdown output"
                    );
                    assert!(
                        !markdown.contains("Footer Information"),
                        "Ignored section 'Footer Information' found in Markdown output"
                    );
                } else {
                    panic!("Expected Content::MarkDown variant");
                }
            }
            Err(err) => {
                panic!("Test failed with error: {:?}", err);
            }
        }
    }
}
