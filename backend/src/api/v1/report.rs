use crate::api::ApiResponse;
use crate::db::SurrealDb;
use crate::models::{
    Report, ReportCreation, ReportStatus, ReportStatusEvent, SurrealDBReport, SurrealDBUser, User,
};
use crate::prelude::FinanalizeError;
use crate::rabbitmq::RabbitMQPublisher;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, Responder};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[post("/reports")]
pub async fn create_report(
    user: SurrealDBUser,
    db: Data<SurrealDb>,
    report_creation: Json<ReportCreation>,
) -> Result<impl Responder, FinanalizeError> {
    let report: SurrealDBReport = db
        .create("report")
        .content(report_creation)
        .await?
        .ok_or(FinanalizeError::InternalServerError)?;
    let relation = db
        .query("RELATE $user -> has -> $report")
        .bind(("user", user.id.clone()))
        .bind(("report", report.id.clone()))
        .await?;
    let created_report: Report = Report::from(report.clone());
    let report_status_event: ReportStatusEvent = ReportStatusEvent::from(created_report.clone());
    let publisher = RabbitMQPublisher::new().await?;
    publisher.publish_report_status(report_status_event).await?;
    Ok(ApiResponse::new(created_report))
}

#[get("/reports/{report_id}")]
pub async fn get_report( // FIXME: if you know the id you can see any report.
    db: Data<SurrealDb>,
    report_id: Path<String>,
) -> Result<impl Responder, FinanalizeError> {
    let report: SurrealDBReport = db
        .select(("report", report_id.to_string()))
        .await?
        .ok_or(FinanalizeError::NotFound)?;
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
        return Ok(ApiResponse::error(StatusCode::NOT_FOUND, "fucking hoe?".into()));
    };
    let reports: Vec<Report> = report_query.reports.into_iter().map(|r| r.into()).collect();
    Ok(ApiResponse::new(reports))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportQuery {
    reports: Vec<SurrealDBReport>
}
