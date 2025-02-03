use crate::extractors::ContentExtractor;

pub struct TextExtractor;

impl ContentExtractor for TextExtractor {
    fn extract(&self, input: &str) -> Vec<String> {
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
        
        chunks
    }
}