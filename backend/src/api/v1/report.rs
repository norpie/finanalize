use crate::api::ApiResponse;
use crate::db::SurrealDb;
use crate::jwt::TokenFactory;
use crate::models::{
    FrontendReport, FullReport, FullSDBReport, Report, ReportCreation, SurrealDBReport,
    SurrealDBUser,
};
use crate::prelude::FinanalizeError;
use crate::prelude::*;
use crate::rabbitmq::PUBLISHER;
use crate::workflow::{JobType, SDBWorkflowState, WorkflowState};
use actix_files::NamedFile;
use actix_web::web::{self, Data, Json, Path};
use actix_web::{get, post, rt, HttpRequest, Responder};
use actix_ws::Message;
use futures_util::StreamExt;
use log::debug;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportSize {
    #[serde(rename = "s")]
    Small,
    #[serde(rename = "m")]
    Medium,
    #[serde(rename = "l")]
    Large,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportModel {
    #[serde(rename = "l")]
    Llama,
    #[serde(rename = "q")]
    Qwen,
    #[serde(rename = "o")]
    OpenAI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReportCreationLight {
    user_input: String,
    size: ReportSize,
    model: ReportModel,
}

#[post("/reports")]
pub async fn create_report(
    user: SurrealDBUser,
    db: Data<SurrealDb>,
    report_creation: Json<ReportCreationLight>,
) -> Result<impl Responder> {
    let report_creation = ReportCreation::new(
        report_creation.user_input.clone(),
        report_creation.size.clone(),
        report_creation.model.clone(),
    );
    let report: FullSDBReport = db
        .create("report")
        .content(report_creation)
        .await?
        .ok_or(FinanalizeError::InternalServerError)?;
    debug!("Created db report: {:#?}", report);
    db.query("RELATE $user -> has -> $report")
        .bind(("user", user.id.clone()))
        .bind(("report", report.id.clone()))
        .await?;
    let created_report: FullReport = FullReport::from(report);
    debug!("Created report object: {:#?}", created_report);
    let workflow_status_update = WorkflowState {
        id: created_report.id.to_string(),
        last_job_type: JobType::Pending,
        state: created_report.clone(),
    };
    PUBLISHER
        .get()
        .unwrap()
        .channel
        .basic_publish(
            "",
            "report_status",
            Default::default(),
            serde_json::to_string(&workflow_status_update)?.as_bytes(),
            Default::default(),
        )
        .await?;
    debug!(
        "Published workflow status update: {:#?}",
        workflow_status_update
    );
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

#[get("/reports/{report_id}")]
pub async fn get_report(
    user: SurrealDBUser,
    db: Data<SurrealDb>,
    report_id: Path<String>,
) -> Result<impl Responder> {
    let db_report_id = Thing::from(("report", report_id.as_str()));
    let _report = db
        .query("SELECT * FROM (SELECT ->has->report as reports FROM $user FETCH reports).reports[0] WHERE id = $report;")
        .bind(("user", user.id.clone()))
        .bind(("report", db_report_id.clone()))
        .await?.take::<Option<SurrealDBReport>>(0)?.ok_or(FinanalizeError::NotFound)?;
    debug!("Report: {:#?}", _report);

    let workflow_state_id_thing: Thing = ("workflow_state", report_id.as_str()).into();

    let workflow_state = db
        .query("SELECT * FROM $workflow_state")
        .bind(("workflow_state", workflow_state_id_thing))
        .await?
        .take::<Option<SDBWorkflowState>>(0)?
        .ok_or(FinanalizeError::NotFound)?;
    debug!("Workflow state: {:#?}", workflow_state);
    let (valid, error) = workflow_state
        .state
        .validation
        .map(|validation| (validation.valid, validation.error))
        .unwrap_or((
            false,
            Some("Validation has not been performed yet.".to_string()),
        ));
    let frontend_report = FrontendReport {
        user_input: workflow_state.state.user_input,
        status: workflow_state.state.status,
        size: workflow_state.state.size,
        model: workflow_state.state.model,
        title: workflow_state.state.title,
        valid: Some(valid),
        error,
    };
    Ok(ApiResponse::new(frontend_report))
}

#[post("/reports/{report_id}/retry")]
pub async fn retry(
    _report_id: Path<String>,
    _user: SurrealDBUser,
    _db: Data<SurrealDb>,
) -> Result<impl Responder> {
    // let _sdb_report = db
    //     .query("SELECT * FROM (SELECT ->has->report as reports FROM $user FETCH reports).reports[0] WHERE id = $report;")
    //     .bind(("user", user.id.clone()))
    //     .bind(("report", Thing::from(("report", report_id.as_str()))))
    //     .await?.take::<Option<SurrealDBReport>>(0)?.ok_or(FinanalizeError::NotFound)?;
    // let sdb_workflow_state = db
    //     .query("SELECT * FROM $workflow_state")
    //     .bind(("workflow_state", Thing::from(("workflow_state", report_id.as_str()))))
    //     .await?.take::<Option<SDBWorkflowState>>(0)?.ok_or(FinanalizeError::NotFound)?;
    // let workflow_status: WorkflowState = WorkflowState::from(sdb_workflow_state);
    //
    //    PUBLISHER
    //         .get()
    //         .unwrap()
    //         .channel
    //         .basic_publish(
    //             "",
    //             "report_status",
    //             Default::default(),
    //             serde_json::to_string(&workflow_status)?.as_bytes(),
    //             Default::default(),
    //         )
    //         .await?;
    // TODO: Implement retry logic
    //
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
#[derive(Deserialize)]
struct AuthQuery {
    bearer: String,
}

#[get("/reports/{report_id}/document.pdf")]
pub async fn get_document(db: Data<SurrealDb>, report_id: Path<String>) -> Result<impl Responder> {
    let workflow_state_id_thing: Thing = ("workflow_state", report_id.as_str()).into();
    dbg!("here");
    let workflow_state = db
        .query("SELECT * FROM $workflow_state")
        .bind(("workflow_state", workflow_state_id_thing))
        .await?
        .take::<Option<SDBWorkflowState>>(0)?
        .ok_or(FinanalizeError::NotFound)?;
    dbg!("here2");

    let Some(src) = workflow_state.state.report.clone() else {
        dbg!("here3");
        return Err(FinanalizeError::NotFound);
    };
    dbg!("here4");
    let report = NamedFile::open_async(src).await?;
    Ok(report)
}

#[get("/live/reports/{report_id}")]
pub async fn get_live_report(
    req: HttpRequest,
    stream: web::Payload,
    db: Data<SurrealDb>,
    report_id: Path<String>,
    query: web::Query<AuthQuery>,
    token_factory: Data<TokenFactory>,
) -> Result<impl Responder> {
    let token = &query.bearer;
    debug!("Bearer token: {:#?}", token);
    let user_id = token_factory.subject(token)?;
    debug!("User ID: {:#?}", user_id);
    let db_report_id = Thing::from(("report", report_id.as_str()));
    let db_user_id = Thing::from(("user", user_id.as_str()));
    let _report = db
         .query("SELECT * FROM (SELECT ->has->report as reports FROM $user FETCH reports).reports[0] WHERE id = $report;")
         .bind(("user", db_user_id))
         .bind(("report", db_report_id.clone()))
         .await?.take::<Option<SurrealDBReport>>(0)?.ok_or(FinanalizeError::NotFound)?;
    debug!("Report: {:#?}", _report);
    let (res, mut session, mut ws) = actix_ws::handle(&req, stream)?;

    let db = db.get_ref().clone();
    let mut stream: surrealdb::method::Stream<Option<SDBWorkflowState>> = db
        .select(("workflow_state", report_id.as_str()))
        .live()
        .await?;

    rt::spawn(async move {
        debug!("Starting live query stream");
        while let Some(res) = stream.next().await {
            let Ok(notification) = res else {
                debug!("error {:#?}", res.unwrap_err());
                break;
            };
            debug!("update");
            let report = &notification.data;
            let frontend_report = FrontendReport {
                user_input: report.state.user_input.clone(),
                status: report.state.status,
                size: report.state.size.clone(),
                model: report.state.model.clone(),
                title: report.state.title.clone(),
                valid: Some(report.state.validation.clone().unwrap().valid),
                error: None,
            };
            session
                .text(serde_json::to_string_pretty(&frontend_report).unwrap())
                .await
                .unwrap();
        }
        while let Some(msg) = ws.next().await {
            let Ok(msg) = msg else {
                debug!("Error in message stream: {:?}", msg.unwrap_err());
                break;
            };
            debug!("Received WebSocket message: {:?}", msg);
            if let Message::Close(_) = msg {
                debug!("Client closed WebSocket connection.");
                session.close(None).await.ok();
                break;
            }
        }
        debug!("Ending live query stream");
    });
    Ok(res)
}
