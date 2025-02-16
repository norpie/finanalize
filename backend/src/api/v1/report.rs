use crate::api::ApiResponse;
use crate::db::SurrealDb;
use crate::models::{Report, ReportCreation, ReportStatusEvent, SurrealDBReport, SurrealDBUser};
use crate::prelude::FinanalizeError;
use crate::prelude::*;
use crate::rabbitmq::PUBLISHER;
use actix_web::http::StatusCode;
use actix_web::web::{self, Data, Json, Path};
use actix_web::{get, post, Responder};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReportCreationLight {
    user_input: String,
}

#[post("/reports")]
pub async fn create_report(
    user: SurrealDBUser,
    db: Data<SurrealDb>,
    report_creation: Json<ReportCreationLight>,
) -> Result<impl Responder> {
    let report_creation = ReportCreation::new(report_creation.user_input.clone());
    let report: SurrealDBReport = db
        .create("report")
        .content(report_creation)
        .await?
        .ok_or(FinanalizeError::InternalServerError)?;
    db.query("RELATE $user -> has -> $report")
        .bind(("user", user.id.clone()))
        .bind(("report", report.id.clone()))
        .await?;
    let created_report: Report = Report::from(report.clone());
    let report_status_event: ReportStatusEvent = ReportStatusEvent::from(created_report.clone());
    PUBLISHER
        .get()
        .unwrap()
        .publish_report_status(report_status_event)
        .await?;
    Ok(ApiResponse::new(created_report))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verdict {
    pub valid: bool,
    pub justification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heading {
    pub heading: String,
    pub paragraphs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct FullReport {
    pub report: Report,
    pub verdict: Option<Verdict>,
    pub title: Option<String>,
    pub headings: Vec<Heading>,
    pub searches: Vec<Query>,
    pub sources: Vec<Source>,
}

#[get("/reports/{report_id}")]
pub async fn get_report(
    user: SurrealDBUser,
    db: Data<SurrealDb>,
    report_id: Path<String>,
) -> Result<impl Responder> {
    let report = db
        .query("SELECT * FROM (SELECT ->has->report as reports FROM $user FETCH reports).reports[0] WHERE id = $report;")
        .bind(("user", user.id))
        .bind(("report", Thing::from(("report", report_id.as_str()))))
        .await?.take::<Option<SurrealDBReport>>(0)?.ok_or(FinanalizeError::NotFound)?;

    let verdict = db
        .query("SELECT * FROM (SELECT ->has_verdict->report_verdict as verdicts FROM $report FETCH verdicts).verdicts[0];")
        .bind(("report", report.id.clone()))
        .await?.take::<Option<Verdict>>(0)?;

    let title = db
        .query("SELECT * FROM (SELECT ->has_title->report_title as titles FROM $report FETCH titles).titles[0]")
        .bind(("report", report.id.clone()))
        .await?.take::<Option<String>>((0, "title"))?;

    let headings = db
        .query("SELECT * FROM (SELECT ->has_paragraph->paragraph as paragraphs FROM $report FETCH paragraphs)[0].paragraphs")
        .bind(("report", report.id.clone()))
        .await?.take::<Vec<Heading>>(0)?;

    let searches = db
        .query("SELECT * FROM (SELECT ->has_search_query->search_query as searches FROM $report FETCH searches)[0].searches")
        .bind(("report", report.id.clone()))
        .await?.take::<Vec<Query>>(0)?;

    let sources = db
        .query("SELECT * FROM (SELECT ->has_search_result->search_result as sources FROM $report FETCH sources)[0].sources")
        .bind(("report", report.id.clone()))
        .await?.take::<Vec<Source>>(0)?;

    Ok(ApiResponse::new(FullReport {
        report: Report::from(report),
        verdict,
        title,
        headings,
        searches,
        sources,
    }))
}

#[post("/reports/{report_id}/retry")]
pub async fn retry(
    report_id: Path<String>,
    user: SurrealDBUser,
    db: Data<SurrealDb>,
) -> Result<impl Responder> {
    let report = db
        .query("SELECT * FROM (SELECT ->has->report as reports FROM $user FETCH reports).reports[0] WHERE id = $report;")
        .bind(("user", user.id))
        .bind(("report", Thing::from(("report", report_id.as_str()))))
        .await?.take::<Option<SurrealDBReport>>(0)?.ok_or(FinanalizeError::NotFound)?;

    PUBLISHER
        .get()
        .unwrap()
        .publish_report_status(ReportStatusEvent::from(Report::from(report.clone())))
        .await?;

    Ok(ApiResponse::new("OK"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserReportPage {
    page: u32,
    #[serde(rename = "perPage")]
    per_page: u32,
}

#[get("/reports")]
pub async fn get_reports(
    user: SurrealDBUser,
    db: Data<SurrealDb>,
    page: web::Query<UserReportPage>,
) -> Result<impl Responder> {
    Ok(ApiResponse::new(db
        .query(
            "SELECT * FROM (SELECT ->has->report as reports FROM $user FETCH reports)[0].reports ORDER BY created_at DESC LIMIT $perPage START $start",
        )
        .bind(("user", user.id.clone()))
        .bind(("perPage", page.per_page))
        .bind(("start", page.page * page.per_page))
        .await?
        .take::<Vec<SurrealDBReport>>(0)?
        .into_iter()
        .map(Report::from)
        .collect::<Vec<Report>>()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportQuery {
    reports: Vec<SurrealDBReport>,
}
