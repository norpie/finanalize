use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{models::SurrealDBReport, prelude::*, prompting, tasks::Task, workflow::JobType};

use std::sync::Arc;

use crate::{db::SurrealDb, llm::LLMApi, search::SearchEngine};

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

#[derive(Clone, Debug, Serialize)]
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
            .content(verdict.clone())
            .await?
            .ok_or(FinanalizeError::UnableToCreateReportVerdict)?;
        let mut report = report.clone();
        if !verdict.valid {
            report.status = JobType::Invalid;
        }
        let report: SurrealDBReport = db
            .update(("report", report.id.id.to_string().as_str()))
            .content(report)
            .await?
            .ok_or(FinanalizeError::UnableToUpdateReport)?;
        dbg!(&report);
        db.query("RELATE $report -> has_verdict -> $verdict")
            .bind(("report", report.id.clone()))
            .bind(("verdict", sdb_verdict.id))
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{db, llm::ollama::Ollama, models::ReportCreation, search::SearxNG};

    use super::*;

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test() {
        let db = db::connect().await.unwrap();
        let llm = Arc::new(Ollama::default());
        let search = Arc::new(SearxNG::new("http://localhost:8081"));
        let creation = ReportCreation::new("Apple 2025 Q4 outlook".into());
        let report: SurrealDBReport = db
            .create("report")
            .content(creation)
            .await
            .unwrap()
            .unwrap();
        dbg!(&report);
        let job = ValidationJob;
        job.run(&report, db, llm, search).await.unwrap();
    }
}
