use crate::api::ApiResponse;
use crate::db::SurrealDb;
use crate::models::{
    Report, ReportCreation, ReportStatus, ReportStatusEvent, SurrealDBReport, SurrealDBUser, User,
};
use crate::prelude::FinanalizeError;
use crate::rabbitmq::RabbitMQPublisher;
use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, Responder};
use serde::{Deserialize, Serialize};

#[post("/reports")]
pub async fn create_report(
    user: SurrealDBUser,
    db: Data<SurrealDb>,
    report_creation: Json<ReportCreation>,
) -> Result<impl Responder, FinanalizeError> {
    dbg!("here 1");
    let report: SurrealDBReport = db
        .create("report")
        .content(report_creation)
        .await?
        .ok_or(FinanalizeError::InternalServerError)?;
    dbg!("here 2");
    let relation = db
        .query("$user->has->$report")
        .bind(("$user", user.id.clone()))
        .bind(("$report", report.id.clone()))
        .await?;
    dbg!(relation);
    let created_report: Report = Report::from(report.clone());
    let report_status_event: ReportStatusEvent = ReportStatusEvent::from(created_report.clone());
    let publisher = RabbitMQPublisher::new().await?;
    dbg!("here 3");
    publisher.publish_report_status(report_status_event).await?;
    dbg!("here 4");
    Ok(ApiResponse::new(created_report))
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
