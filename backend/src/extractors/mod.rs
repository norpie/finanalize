pub mod text_extractor;

pub trait ContentExtractor{
    fn extract(&self, input: &str) -> Vec<String>;
}

pub trait DataExtractor{
    fn extract(&self, input: &str) -> Vec<String>;
}