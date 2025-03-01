use super::Figure; // Import the existing Figure struct
use crate::prelude::*;
use async_trait::async_trait;
use scraper::{Html, Selector};
use tokio::task;

use super::Content;
use super::ContentExtract;
use super::FileType;

pub struct FigureExtractor;

#[async_trait]
impl ContentExtract for FigureExtractor {
    async fn extract(&self, file: FileType) -> Result<Vec<Content>> {
        let FileType::Html(input) = file else {
            return Err(FinanalizeError::ParseError(
                "Invalid input type".to_string(),
            ));
        };

        let extracted_figures = task::spawn_blocking(move || {
            let document = Html::parse_document(&input);
            let figure_selector = Selector::parse("figure")?;
            let img_selector = Selector::parse("img")?;
            let caption_selector = Selector::parse("figcaption")?;

            let mut figures = Vec::new();

            for figure_element in document.select(&figure_selector) {
                let Some(img) = figure_element.select(&img_selector).next() else {
                    continue; // Continue to the next iteration if no image is found
                };
                
                let Some(url) = img.value().attr("src") else {
                    continue; // Skip figures without source URLs
                };

                let alt_text = img.value().attr("alt").map(String::from);
                let caption = figure_element
                    .select(&caption_selector)
                    .next()
                    .map(|caption| caption.text().collect::<Vec<_>>().join(" "));
        
                figures.push(Figure {
                    url: url.to_string(),
                    alt_text,
                    caption,
                });
            }

            Ok(figures) as Result<Vec<Figure>>
        })
        .await??;

        Ok(vec![Content::Figures(extracted_figures)])
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_figure_extraction() {
        let test_html = r#"
        <html>
            <body>
                <figure>
                    <img src="image1.jpg" alt="Image 1 Description">
                    <figcaption>Figure 1 Caption</figcaption>
                </figure>
                <figure>
                    <img src="image2.png">
                    <figcaption>Figure 2 Caption</figcaption>
                </figure>
                <img src="standalone.jpg" alt="Standalone Image">
            </body>
        </html>
        "#;

        let file = FileType::Html(test_html.to_string());
        let extractor = FigureExtractor;

        match extractor.extract(file).await {
            Ok(result) => {
                println!("Extraction result: {:?}", result);
                if let Content::Figures(figures) = &result[0] {
                    assert_eq!(figures.len(), 2);
                    assert_eq!(figures[0].url, "image1.jpg");
                    assert_eq!(figures[0].alt_text, Some("Image 1 Description".to_string()));
                    assert_eq!(figures[0].caption, Some("Figure 1 Caption".to_string()));
                    assert_eq!(figures[1].url, "image2.png");
                    assert_eq!(figures[1].alt_text, None);
                    assert_eq!(figures[1].caption, Some("Figure 2 Caption".to_string()));
                    assert!(
                        figures.iter().all(|f| !f.url.is_empty()),
                        "All figures must have a valid URL"
                    );
                } else {
                    panic!("Expected Content::Figures variant");
                }
            }
            Err(err) => {
                panic!("Test failed with error: {:?}", err);
            }
        }
    }
}
