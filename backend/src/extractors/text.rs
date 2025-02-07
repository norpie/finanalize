use crate::prelude::*;
use async_trait::async_trait;

use super::ContentExtract;

pub struct TextExtractor;

#[async_trait]
impl ContentExtract for TextExtractor {
    async fn extract(&self, input: &str) -> Result<Vec<String>> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        for word in input.split_whitespace() {
            if current_chunk.len() + word.len() >= 512 {
                chunks.push(current_chunk.clone());
                current_chunk.clear();
            }
            if !current_chunk.is_empty() {
                current_chunk.push(' ');
            }
            current_chunk.push_str(word);
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_extract() {
        let text = "Once upon a time, in a vast kingdom surrounded by towering mountains and endless forests, there lived a wise old storyteller named Eldrin. \
        He traveled from village to village, sharing tales of ancient heroes, mystical creatures, and forgotten lands. Children gathered around him, eyes wide with wonder, \
        as he spoke of dragons soaring through the skies, knights embarking on perilous quests, and hidden treasures buried deep beneath the earth. \
        His voice carried the weight of history, weaving a tapestry of imagination that spanned generations. Eldrin believed that stories held power—the power to inspire, \
        to teach, and to unite. Even kings sought his wisdom, hoping to learn from the past through his words. But one day, as he reached the final village on his journey, \
        Eldrin encountered a young girl who claimed she had a story he had never heard before. Intrigued, he sat and listened as she spoke of a world beyond the stars, \
        where dreams shaped reality and hope never faded. Eldrin smiled, knowing that every story leads to another.";

        let extractor = TextExtractor;
        let chunks = extractor.extract(text).await.unwrap();

        // Check that each chunk is at most 512 characters
        for chunk in &chunks {
            assert!(chunk.len() <= 512, "Chunk exceeds 512 characters!");
        }
    }
}
