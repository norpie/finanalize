use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{models::SurrealDBReport, prelude::*, prompting, tasks::Task};

use std::sync::Arc;

use crate::{db::SurrealDb, llm::LLMApi, scraper::BrowserWrapper, search::SearchEngine};

use super::Job;

#[derive(Debug, Serialize)]
struct ValidationTaskInpput {
    message: String,
}

#[derive(Debug, Deserialize)]
struct ValidationTaskOutput {
    valid: bool,
    error: Option<String>,
}

pub struct ValidationJob;

#[derive(Debug, Serialize)]
pub struct ReportVerdict {
    justification: String,
    valid: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SurrealDBVerdict {
    id: Thing,
    justification: String,
    valid: bool,
}

#[async_trait]
impl Job for ValidationJob {
    async fn run(
        &self,
        report: &SurrealDBReport,
        db: SurrealDb,
        llm: Arc<dyn LLMApi>,
        _search: Arc<dyn SearchEngine>,
        _browser: BrowserWrapper,
    ) -> Result<()> {
        let prompt = prompting::get_prompt(db.clone(), "validation".into()).await?;
        let validation_task = Task::new(&prompt);
        let validation_input = ValidationTaskInpput {
            message: report.user_input.clone(),
        };
        let validation_output: ValidationTaskOutput =
            validation_task.run(llm, &validation_input).await?;
        let verdict = ReportVerdict {
            justification: validation_output.error.unwrap_or("N/A".into()),
            valid: validation_output.valid,
        };
        let sdb_verdict: SurrealDBVerdict = db
            .create("report_verdict")
            .content(verdict)
            .await?
            .ok_or(FinanalizeError::UnableToCreateReportVerdict)?;
        db.query("RELATE $report -> has_verdict -> $verdict")
            .bind(("report", report.id.clone()))
            .bind(("verdict", sdb_verdict.id))
            .await?;
        Ok(())
    }
}
