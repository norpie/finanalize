use crate::prelude::*;
use async_trait::async_trait;

use super::{JobType, WorkflowState};

pub mod answer_questions;
pub mod chunk_content;
pub mod classify_data;
pub mod classify_sources;
pub mod content_formatter;
pub mod extract_content;
pub mod extract_data;
// pub mod generate_graphs; // png
pub mod generate_preview;
pub mod generate_report;
// pub mod generate_visualizations; // what graphs from available data
// pub mod graph_identifier; // where to put the graphs
pub mod index_chunks;
pub mod scrape_pages;
// pub mod search_before_questions;
pub mod search_queries;
pub mod search_terms;
pub mod section_names;
pub mod sectionize_questions;
pub mod sub_section_questions;
pub mod sub_sections;
pub mod title;
pub mod validation;

#[async_trait]
pub trait Job: Send + Sync + 'static {
    async fn run(&self, state: WorkflowState) -> Result<WorkflowState>;
}

impl JobType {
    pub fn next(&self) -> Option<JobType> {
        match self {
            // Start
            JobType::Pending => Some(JobType::Validation),
            // Doing
            JobType::Validation => Some(JobType::GenerateTitle),
            JobType::GenerateTitle => Some(JobType::GenerateSectionNames),
            JobType::GenerateSectionNames => Some(JobType::GenerateSubSectionNames),
            JobType::GenerateSubSectionNames => Some(JobType::GenerateSubSectionQuestions),
            JobType::GenerateSubSectionQuestions => Some(JobType::GenerateSearchQueries),
            JobType::GenerateSearchQueries => Some(JobType::SearchQueries),
            JobType::SearchQueries => Some(JobType::ScrapeTopResults),
            JobType::ScrapeTopResults => Some(JobType::ExtractContent),
            JobType::ExtractContent => Some(JobType::FormatContent),
            // JobType::ExtractContent => Some(JobType::ExtractData),
            // JobType::ExtractData => Some(JobType::FormatContent),
            JobType::FormatContent => Some(JobType::ClassifyContent),
            JobType::ClassifyContent => Some(JobType::ChunkContent),
            // JobType::ClassifyContent => Some(JobType::ClassifyData),
            // JobType::ClassifyData => Some(JobType::GenerateVisualizations),
            // JobType::GenerateVisualizations => Some(JobType::GenerateGraphs),
            // JobType::GenerateGraphs => Some(JobType::ChunkContent),
            JobType::ChunkContent => Some(JobType::IndexChunks),
            JobType::IndexChunks => Some(JobType::AnswerQuestions),
            JobType::AnswerQuestions => Some(JobType::SectionizeQuestions),
            JobType::SectionizeQuestions => Some(JobType::RenderLaTeXPdf),
            // JobType::SectionizeQuestions => Some(JobType::RenderGraphs),
            // JobType::RenderGraphs => Some(JobType::RenderLaTeXPdf),
            JobType::RenderLaTeXPdf => Some(JobType::GeneratePreviewDocument),
            JobType::GeneratePreviewDocument => Some(JobType::Done),
            // Done
            JobType::Invalid => None,
            JobType::Failed => None,
            JobType::Done => None,
        }
    }

    pub fn job(&self) -> Option<Box<dyn Job>> {
        match self {
            JobType::Pending => None,
            JobType::Validation => Some(Box::new(validation::ValidationJob)),
            JobType::GenerateTitle => Some(Box::new(title::TitleJob)),
            JobType::GenerateSectionNames => Some(Box::new(section_names::SectionNamesJob)),
            JobType::GenerateSubSectionNames => Some(Box::new(sub_sections::SubSectionsJob)),
            JobType::GenerateSubSectionQuestions => {
                Some(Box::new(sub_section_questions::SubSectionQuestionsJob))
            }
            JobType::GenerateSearchQueries => {
                Some(Box::new(search_queries::GenerateSearchQueriesJob))
            }
            JobType::SearchQueries => Some(Box::new(search_terms::SearchJob)),
            JobType::ScrapeTopResults => Some(Box::new(scrape_pages::ScrapePagesJob)),
            JobType::ExtractContent => Some(Box::new(extract_content::ExtractContentJob)),
            // JobType::ExtractData => Some(Box::new(extract_data::ExtractDataJob)),
            JobType::FormatContent => Some(Box::new(content_formatter::FormatContentJob)),
            JobType::ClassifyContent => Some(Box::new(classify_sources::ClassifySourcesJob)),
            // JobType::ClassifyData => Some(Box::new(classify_data::ClassifyDataJob)),
            // JobType::GenerateVisualizations => {
            // Some(Box::new(generate_visualizations::GenerateVisualizationsJob))
            // }
            // JobType::GenerateGraphs => Some(Box::new(generate_graphs::GenerateGraphsJob)),
            JobType::ChunkContent => Some(Box::new(chunk_content::ChunkContentJob)),
            JobType::IndexChunks => Some(Box::new(index_chunks::IndexChunksJob)),
            JobType::AnswerQuestions => Some(Box::new(answer_questions::AnswerQuestionsJob)),
            JobType::SectionizeQuestions => {
                Some(Box::new(sectionize_questions::SectionizeQuestionsJob))
            }
            // JobType::RenderGraphs => Some(Box::new(graph_identifier::GraphIdentifierJob)),
            JobType::RenderLaTeXPdf => Some(Box::new(generate_report::GenerateReportJob)),
            JobType::GeneratePreviewDocument => {
                Some(Box::new(generate_preview::GeneratePreviewDocumentJob))
            }
            _ => None,
        }
    }
}
