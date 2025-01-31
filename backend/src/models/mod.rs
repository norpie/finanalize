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
    pub user_id: Thing,
    pub user_input: String,
    pub status: ReportStatus,
    pub created_at: String,
    pub updated_at: String,
    pub has_verdict: bool,
}

impl From<SurrealDBReport> for Report {
    fn from(report: SurrealDBReport) -> Self {
        Report {
            id: report.id.id.to_string(),
            user_id: report.user_id.id.to_string(),
            user_input: report.user_input,
            status: report.status,
            created_at: report.created_at,
            updated_at: report.updated_at,
            has_verdict: report.has_verdict,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportStatus{
    Pending,
    Valid,
    Invalid,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportStatusEvent{
    pub report_id: String,
    pub status: ReportStatus,
}

impl From<Report> for ReportStatusEvent { // ReportStatusEvent::from(report)
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
    pub user_id: String,
    pub user_input: String,
    pub status: ReportStatus,
    pub created_at: String,
    pub updated_at: String,
    pub has_verdict: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCreation {
    pub user_id: String,
    pub user_input: String,
}