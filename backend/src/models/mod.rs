use crate::extractors::Data;
use crate::workflow::job::answer_questions::models::QuestionAnswer;
use crate::workflow::job::classify_sources::models::ClassifiedSource;
use crate::workflow::job::generate_graphs::models::{GraphFileOutput, TableOutput};
use crate::workflow::job::generate_visualizations::models::Visualization;
use crate::workflow::job::graph_identifier::models::GraphIdentifierOutput;
use crate::workflow::{
    job::{
        chunk_content::models::Chunk, index_chunks::models::EmbeddedChunk,
        validation::models::ValidationOutput,
    },
    JobType,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendUser {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDBUser {
    pub id: Thing,
    pub email: String,
    pub password: String,
}

impl From<SurrealDBUser> for User {
    fn from(user: SurrealDBUser) -> Self {
        User {
            id: user.id.id.to_string(),
            email: user.email,
            password: user.password,
        }
    }
}

impl From<SurrealDBUser> for FrontendUser {
    fn from(user: SurrealDBUser) -> Self {
        FrontendUser {
            id: user.id.id.to_string(),
            email: user.email,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealDBReport {
    pub id: Thing,
    pub user_input: String,
    pub status: JobType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<SurrealDBReport> for Report {
    fn from(report: SurrealDBReport) -> Self {
        Report {
            id: report.id.id.to_string(),
            user_input: report.user_input,
            status: report.status,
            created_at: report.created_at.to_utc(),
            updated_at: report.updated_at.to_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportStatusEvent {
    pub report_id: String,
    pub status: JobType,
}

impl From<Report> for ReportStatusEvent {
    fn from(report: Report) -> Self {
        ReportStatusEvent {
            report_id: report.id,
            status: report.status,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: String,
    pub user_input: String,
    pub status: JobType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCreation {
    pub user_input: String,
    pub status: JobType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ReportCreation {
    pub fn new(user_input: String) -> Self {
        let now = Utc::now();
        ReportCreation {
            user_input,
            status: JobType::Pending,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullSDBReport {
    pub id: Thing,
    pub user_input: String,
    pub status: JobType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub validation: Option<ValidationOutput>,
    pub title: Option<String>,
    pub sections: Option<Vec<String>>,
    pub sub_sections: Option<Vec<Vec<String>>>,
    pub sub_section_questions: Option<Vec<Vec<Vec<String>>>>,
    pub search_queries: Option<Vec<String>>,
    pub search_urls: Option<Vec<String>>,
    pub html_sources: Option<Vec<PreClassificationSource>>,
    pub raw_sources: Option<Vec<PreClassificationSource>>,
    pub csv_sources: Option<Vec<String>>,
    pub data_sources: Option<Vec<Data>>,
    pub formatted_sources: Option<Vec<PreClassificationSource>>,
    pub sources: Option<Vec<ClassifiedSource>>,
    pub chunks: Option<Vec<Chunk>>,
    pub chunk_embeddings: Option<Vec<EmbeddedChunk>>,
    pub question_answer_pairs: Option<Vec<Vec<Vec<QuestionAnswer>>>>,
    pub sub_section_contents: Option<Vec<Vec<String>>>,
    pub report: Option<String>,
    pub extracted_data: Option<Vec<Data>>,
    pub visuals: Option<Vec<Visualization>>,
    pub charts: Option<Vec<GraphFileOutput>>,
    pub tables: Option<Vec<TableOutput>>,
    pub report_text: Option<String>,
    pub chart_positions: Option<Vec<GraphIdentifierOutput>>,
    pub table_positions: Option<Vec<GraphIdentifierOutput>>,
}

impl From<FullSDBReport> for FullReport {
    fn from(report: FullSDBReport) -> Self {
        FullReport {
            id: report.id.id.to_string(),
            user_input: report.user_input,
            status: report.status,
            created_at: report.created_at.to_utc(),
            updated_at: report.updated_at.to_utc(),
            validation: report.validation,
            title: report.title,
            sections: report.sections,
            sub_sections: report.sub_sections,
            sub_section_questions: report.sub_section_questions,
            search_queries: report.search_queries,
            search_urls: report.search_urls,
            html_sources: report.html_sources,
            md_sources: report.raw_sources,
            csv_sources: report.csv_sources,
            data_sources: report.data_sources,
            formatted_sources: report.formatted_sources,
            sources: report.sources,
            chunks: report.chunks,
            chunk_embeddings: report.chunk_embeddings,
            question_answer_pairs: report.question_answer_pairs,
            sub_section_contents: report.sub_section_contents,
            report: report.report,
            extracted_data: report.extracted_data,
            visuals: report.visuals,
            charts: report.charts,
            tables: report.tables,
            report_text: report.report_text,
            chart_positions: report.chart_positions,
            table_positions: report.table_positions,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreClassificationSource {
    pub url: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullReport {
    pub id: String,
    pub user_input: String,
    pub status: JobType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub validation: Option<ValidationOutput>,
    pub title: Option<String>,
    pub sections: Option<Vec<String>>,
    pub sub_sections: Option<Vec<Vec<String>>>,
    pub sub_section_questions: Option<Vec<Vec<Vec<String>>>>,
    pub search_queries: Option<Vec<String>>,
    pub search_urls: Option<Vec<String>>,
    pub html_sources: Option<Vec<PreClassificationSource>>,
    pub md_sources: Option<Vec<PreClassificationSource>>,
    pub csv_sources: Option<Vec<String>>,
    pub data_sources: Option<Vec<Data>>,
    pub formatted_sources: Option<Vec<PreClassificationSource>>,
    pub sources: Option<Vec<ClassifiedSource>>,
    pub chunks: Option<Vec<Chunk>>,
    pub chunk_embeddings: Option<Vec<EmbeddedChunk>>,
    pub question_answer_pairs: Option<Vec<Vec<Vec<QuestionAnswer>>>>,
    pub sub_section_contents: Option<Vec<Vec<String>>>,
    pub report: Option<String>,
    pub extracted_data: Option<Vec<Data>>,
    pub visuals: Option<Vec<Visualization>>,
    pub charts: Option<Vec<GraphFileOutput>>,
    pub tables: Option<Vec<TableOutput>>,
    pub report_text: Option<String>,
    pub chart_positions: Option<Vec<GraphIdentifierOutput>>,
    pub table_positions: Option<Vec<GraphIdentifierOutput>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendReport {
    pub user_input: String,
    pub status: JobType,
    pub error: Option<String>,
    pub valid: Option<bool>,
    pub title: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{FullReport, PreClassificationSource};
    use crate::workflow::job::classify_sources::models::ClassifiedSource;
    use crate::workflow::job::generate_graphs::models::{GraphFileOutput, TableOutput};
    use crate::workflow::{
        job::{chunk_content::models::Chunk, validation::models::ValidationOutput},
        JobType,
    };
    use chrono::Utc;

    impl FullReport {
        pub fn new(id: String, user_input: String) -> Self {
            Self {
                id,
                status: JobType::Pending,
                user_input,
                created_at: Utc::now(),
                updated_at: Utc::now(),

                validation: None,
                title: None,
                sections: None,
                sub_sections: None,
                sub_section_questions: None,
                search_queries: None,
                search_urls: None,

                html_sources: None,
                md_sources: None,
                csv_sources: None,
                data_sources: None,
                formatted_sources: None,
                sources: None,
                chunks: None,
                chunk_embeddings: None,

                question_answer_pairs: None,
                sub_section_contents: None,

                report: None,
                extracted_data: None,
                visuals: None,
                charts: None,
                tables: None,
                report_text: None,
                chart_positions: None,
                table_positions: None,
            }
        }

        pub fn with_validation(mut self, validation: ValidationOutput) -> Self {
            self.validation = Some(validation);
            self
        }

        pub fn with_title(mut self, title: String) -> Self {
            self.title = Some(title);
            self
        }

        pub fn with_sections(mut self, sections: Vec<String>) -> Self {
            self.sections = Some(sections);
            self
        }

        pub fn with_sub_sections(mut self, sub_sections: Vec<Vec<String>>) -> Self {
            self.sub_sections = Some(sub_sections);
            self
        }

        pub fn with_sub_section_questions(
            mut self,
            sub_section_questions: Vec<Vec<Vec<String>>>,
        ) -> Self {
            self.sub_section_questions = Some(sub_section_questions);
            self
        }

        pub fn with_searches(mut self, searches: Vec<String>) -> Self {
            self.search_queries = Some(searches);
            self
        }

        pub fn with_search_results(mut self, search_results: Vec<String>) -> Self {
            self.search_urls = Some(search_results);
            self
        }

        pub fn with_html_sources(mut self, html_sources: Vec<PreClassificationSource>) -> Self {
            self.html_sources = Some(html_sources);
            self
        }

        pub fn with_raw_sources(mut self, sources: Vec<PreClassificationSource>) -> Self {
            self.md_sources = Some(sources);
            self
        }

        pub fn with_sources(mut self, sources: Vec<ClassifiedSource>) -> Self {
            self.sources = Some(sources);
            self
        }

        pub fn with_chunks(mut self, chunks: Vec<Chunk>) -> Self {
            self.chunks = Some(chunks);
            self
        }

        pub fn with_extracted_data(mut self, extracted_data: Vec<crate::extractors::Data>) -> Self {
            self.extracted_data = Some(extracted_data);
            self
        }

        pub fn with_visuals(
            mut self,
            visuals: Vec<crate::workflow::job::generate_visualizations::models::Visualization>,
        ) -> Self {
            self.visuals = Some(visuals);
            self
        }

        pub fn with_report_text(mut self, report_text: String) -> Self {
            self.report_text = Some(report_text);
            self
        }

        pub fn with_charts(mut self, charts: Vec<GraphFileOutput>) -> Self {
            self.charts = Some(charts);
            self
        }

        pub fn with_tables(mut self, tables: Vec<TableOutput>) -> Self {
            self.tables = Some(tables);
            self
        }
    }
}
