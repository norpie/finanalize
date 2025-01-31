use crate::api::ApiResponse;
use crate::db::SurrealDb;
use crate::models::{Report, ReportCreation, ReportStatus, ReportStatusEvent, SurrealDBReport};
use crate::prelude::FinanalizeError;
use crate::rabbitmq::RabbitMQPublisher;
use actix_web::web::{Data, Json, Path, Query};
use actix_web::{get, post, Responder};

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
    let created_report: Report = Report::from(new_report.clone());
    let report_status_event: ReportStatusEvent = ReportStatusEvent::from(created_report.clone());
    let publisher = RabbitMQPublisher::new().await?;
    publisher.publish_report_status(report_status_event).await?;
    Ok(ApiResponse::new(ReportStatus::Pending))
}

#[get("/reports/{report_id}")]
pub async fn report_status(
    db: Data<SurrealDb>,
    report_id: Path<String>,
) -> Result<impl Responder, FinanalizeError> {
    let mut response = db
        .query("SELECT * FROM report WHERE id = $id")
        .bind(("id", report_id.to_string()))
        .await?;
    let report = response
        .take::<Option<Report>>(0)?
        .ok_or(FinanalizeError::NotFound)?;

    Ok(ApiResponse::new(report))
}

#[get("/reports")]
pub async fn get_reports(
    db: Data<SurrealDb>,
    user_id: Query<String>,
) -> Result<impl Responder, FinanalizeError> {
    let mut response = db
        .query("SELECT * FROM reports WHERE id = $id")
        .bind(("id", user_id.to_string()))
        .await?;
    let mut reports = Vec::new();
    let mut index = 0;
    while let Some(report) = response.take::<Option<Report>>(index)? {
        reports.push(report);
        index += 1;
    }
    Ok(ApiResponse::new(reports))
}
