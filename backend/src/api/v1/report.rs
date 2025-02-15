use crate::api::ApiResponse;
use crate::db::SurrealDb;
use crate::models::{Report, ReportCreation, ReportStatusEvent, SurrealDBReport, SurrealDBUser};
use crate::prelude::FinanalizeError;
use crate::rabbitmq::{RabbitMQPublisher, PUBLISHER};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Path};
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
) -> Result<impl Responder, FinanalizeError> {
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
    PUBLISHER.get().unwrap().publish_report_status(report_status_event).await?;
    Ok(ApiResponse::new(created_report))
}

#[get("/reports/{report_id}")]
pub async fn get_report(
    user: SurrealDBUser,
    db: Data<SurrealDb>,
    report_id: Path<String>,
) -> Result<impl Responder, FinanalizeError> {
    let report_thing: Thing = ("report", report_id.as_str()).into();
    let mut response = db
        .query("SELECT * FROM (SELECT ->has->report as reports FROM $user FETCH reports).reports[0] WHERE id = $report;")
        .bind(("user", user.id))
        .bind(("report", report_thing))
        .await?;
    let Some(query) = response.take::<Option<SurrealDBReport>>(0)? else {
        return Ok(ApiResponse::error(
            StatusCode::NOT_FOUND,
            "Report not found".into(),
        ));
    };
    let report = query.clone();
    Ok(ApiResponse::new(Report::from(report)))
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
    page: Json<UserReportPage>,
) -> Result<impl Responder, FinanalizeError> {
    let mut response = db
        .query(
            "SELECT ->has->report as reports FROM $user LIMIT $perPage START $start FETCH reports",
        )
        .bind(("user", user.id.clone()))
        .bind(("perPage", page.per_page))
        .bind(("start", page.page * page.per_page))
        .await?;
    let Some(report_query) = response.take::<Option<ReportQuery>>(0)? else {
        return Ok(ApiResponse::error(
            StatusCode::NOT_FOUND,
            "Report not found".into(),
        ));
    };
    let reports: Vec<Report> = report_query.reports.into_iter().map(|r| r.into()).collect();
    Ok(ApiResponse::new(reports))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportQuery {
    reports: Vec<SurrealDBReport>,
}
