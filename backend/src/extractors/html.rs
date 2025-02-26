use super::{Content, ContentExtract, FileType};
use crate::extractors::figure::FigureExtractor; // Import FigureExtractor
use crate::extractors::md::MarkdownExtractor;
use crate::prelude::*;
use async_trait::async_trait;
use log::debug;
use scraper::{ElementRef, Html, Selector};
use tokio::task;

pub struct HTMLExtractor;

#[async_trait]
impl ContentExtract for HTMLExtractor {
    async fn extract(&self, file: FileType) -> Result<Vec<Content>> {
        debug!("Extracting content from HTML file");
        // Ensure the input is of the correct type
        let FileType::Html(input) = file else {
            return Err(FinanalizeError::ParseError(
                "Invalid input type".to_string(),
            ));
        };
        debug!("Input type is HTML");
        // Step 1: Extract figures using FigureExtractor
        let figure_extractor = FigureExtractor;
        let figure_content = figure_extractor
            .extract(FileType::Html(input.clone()))
            .await
            .unwrap_or_else(|_| vec![]); // Safely handle errors by returning an empty vector

        // Step 2: Extract text for Markdown conversion
        let extracted_texts = task::spawn_blocking(move || {
            debug!("Parsing HTML content");
            let document = Html::parse_document(&input);

            // Select all `div` and `article` elements
            let selector = Selector::parse("div, article")
                .map_err(|err| FinanalizeError::ParseError(format!("{:?}", err)))?;

            let mut texts = vec![];

            for element in document.select(&selector) {
                // Check if the element is NOT inside ignored tags
                if !is_inside_ignored_section(element) {
                    let text = element.text().collect::<Vec<_>>().join(" ");
                    debug!("Extracted text: {}", text); // Debug log
                    if !text.is_empty() {
                        texts.push(text);
                    }
                } else {
                    debug!("Ignored element: {:?}", element.value().name()); // Debug log
                }
            }

            if texts.is_empty() {
                return Err(FinanalizeError::NotFound);
            }

            Ok(texts)
        })
        .await
        .map_err(|_| FinanalizeError::InternalServerError)??;
        debug!("Extracted texts: {:?}", extracted_texts);
        let markdown_extractor = MarkdownExtractor;

        // Convert extracted HTML content to Markdown using MarkdownExtractor
        let markdown_content = markdown_extractor
            .extract(FileType::MarkDown(extracted_texts.join("\n")))
            .await
            .unwrap_or_else(|_| vec![]); // Safely handle errors by returning an empty vector

        // Combine figure content and markdown content
        let mut result = figure_content;
        result.extend(markdown_content);

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
                <figure>
                    <img src="image1.jpg" alt="Image 1">
                    <figcaption>Caption for Image 1</figcaption>
                </figure>
                <footer>Footer Information</footer>
            </body>
        </html>
        "#;

        let file = FileType::Html(test_html.to_string());
        let extractor = HTMLExtractor;

        match extractor.extract(file).await {
            Ok(result) => {
                println!("Extraction result: {:?}", result);

                // Ensure the result contains both figures and Markdown
                assert!(result.iter().any(|content| matches!(content, Content::Figures(_))));
                assert!(result.iter().any(|content| matches!(content, Content::MarkDown(_))));

                // Check the figure content
                if let Some(Content::Figures(figures)) = result.iter().find(|content| matches!(content, Content::Figures(_))) {
                    assert_eq!(figures.len(), 1);
                    let figure = &figures[0];
                    assert_eq!(figure.url, "image1.jpg");
                    assert_eq!(figure.alt_text.as_deref(), Some("Image 1"));
                    assert_eq!(figure.caption.as_deref(), Some("Caption for Image 1"));
                }

                // Check the Markdown content
                if let Some(Content::MarkDown(markdown)) = result.iter().find(|content| matches!(content, Content::MarkDown(_))) {
                    assert!(markdown.contains("Main Content"));
                    assert!(markdown.contains("Another Important Section"));

                    // Ensure ignored sections are not included
                    assert!(!markdown.contains("Header Title"));
                    assert!(!markdown.contains("Navigation Links"));
                    assert!(!markdown.contains("Sidebar Content"));
                    assert!(!markdown.contains("Footer Information"));
                }
            }
            Err(err) => {
                panic!("Test failed with error: {:?}", err);
            }
        }
    }
}
