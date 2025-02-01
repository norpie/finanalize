use crate::api::ApiResponse;
use crate::db::SurrealDb;
use crate::models::{Report, ReportCreation, ReportStatus, ReportStatusEvent, SurrealDBReport};
use crate::prelude::FinanalizeError;
use crate::rabbitmq::RabbitMQPublisher;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, Responder};
use serde::{Deserialize, Serialize};

#[post("/reports")]
pub async fn create_report(
    db: Data<SurrealDb>,
    report_creation: Json<ReportCreation>,
) -> Result<impl Responder, FinanalizeError> {
    let new_report: SurrealDBReport = db
        .create("report")
        .content(report_creation)
        .await?
        .ok_or(FinanalizeError::InternalServerError)?;
    let mut created_report: Report = Report::from(new_report.clone());
    let status = ReportStatus::Pending;
    created_report.status = status;
    let report_status_event: ReportStatusEvent = ReportStatusEvent::from(created_report.clone());
    let publisher = RabbitMQPublisher::new().await?;
    publisher.publish_report_status(report_status_event).await?;
    Ok(ApiResponse::new(created_report.id))
}

#[get("/reports/{report_id}")]
pub async fn get_report(
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
    id: String,
    page: u32,
    #[serde(rename = "perPage")]
    per_page: u32,
}

#[get("/reports")]
pub async fn get_reports(
    db: Data<SurrealDb>,
    page: Json<UserReportPage>,
) -> Result<impl Responder, FinanalizeError> {
    let mut response = db
        .query("SELECT ->has_report->report FROM user:$id LIMIT $perPage OFFSET $offset")
        .bind(("id", page.id.to_string()))
        .bind(("perPage", page.per_page))
        .bind(("offset", page.page * page.per_page))
        .await?;
    let reports = response.take::<Vec<Report>>(0)?;
    Ok(ApiResponse::new(reports))
}
