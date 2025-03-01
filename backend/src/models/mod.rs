use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::workflow::{
    job::{classify_sources::models::ClassifySourcesOutput, validation::models::ValidationOutput},
    JobType,
};

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
    pub searches: Option<Vec<String>>,
    pub search_results: Option<Vec<String>>,
    pub raw_sources: Option<Vec<String>>,
    pub sources: Option<Vec<ClassifySourcesOutput>>,
    pub report: Option<String>,
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
            searches: report.searches,
            search_results: report.search_results,
            raw_sources: report.raw_sources,
            sources: report.sources,
            report: report.report,
        }
    }
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
    pub searches: Option<Vec<String>>,
    pub search_results: Option<Vec<String>>,
    pub raw_sources: Option<Vec<String>>,
    pub sources: Option<Vec<ClassifySourcesOutput>>,
    pub report: Option<String>,
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::workflow::{
        job::{
            classify_sources::models::ClassifySourcesOutput, validation::models::ValidationOutput,
        },
        JobType,
    };

    use super::FullReport;

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
                searches: None,
                search_results: None,
                raw_sources: None,
                sources: None,
                report: None,
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
            self.searches = Some(searches);
            self
        }

        pub fn with_search_results(mut self, search_results: Vec<String>) -> Self {
            self.search_results = Some(search_results);
            self
        }

        pub fn with_raw_sources(mut self, sources: Vec<String>) -> Self {
            self.raw_sources = Some(sources);
            self
        }

        pub fn with_sources(mut self, sources: Vec<ClassifySourcesOutput>) -> Self {
            self.sources = Some(sources);
            self
        }
    }
}
