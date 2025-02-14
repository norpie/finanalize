use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{models::SurrealDBReport, prelude::*, prompting, tasks::Task};

use std::sync::Arc;

use crate::{db::SurrealDb, llm::LLMApi, scraper::BrowserWrapper, search::SearchEngine};

use super::Job;

#[derive(Debug, Serialize)]
struct TitleTaskInpput {
    message: String,
}

#[derive(Debug, Deserialize)]
struct TitleTaskOutput {
    output: Option<String>,
    error: Option<String>,
}

pub struct TitleJob;

#[derive(Debug, Serialize)]
pub struct ReportTitle {
    title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SurrealDBTitle {
    id: Thing,
    title: String,
}

#[async_trait]
impl Job for TitleJob {
    async fn run(
        &self,
        report: &SurrealDBReport,
        db: SurrealDb,
        llm: Arc<dyn LLMApi>,
        _search: Arc<dyn SearchEngine>,
        _browser: BrowserWrapper,
    ) -> Result<()> {
        let prompt = prompting::get_prompt(db.clone(), "title".into()).await?;
        let title_task = Task::new(&prompt);
        let title_input = TitleTaskInpput {
            message: report.user_input.clone(),
        };
        let title_output: TitleTaskOutput = title_task.run(llm, &title_input).await?;

        // TODO: check if title_output.error is None, return with error
        if let Some(err) = title_output.error {
            return Err(FinanalizeError::TaskExecutionError(err));
        }

        let report_title = ReportTitle {
            title: title_output.output.clone().unwrap_or_else(|| "Default Title".to_string()),
        };

        let sdb_title: SurrealDBTitle = db
            .create("report_title")
            .content(report_title)
            .await?
            .ok_or(FinanalizeError::UnableToCreateReportTitle)?;
        db.query("RELATE $report -> has_title -> $title")
            .bind(("report", report.id.clone()))
            .bind(("title", sdb_title.id))
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{db, llm::ollama::Ollama, models::ReportCreation, scraper, search::SearxNG};

    use super::*;

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test() {
        dotenvy::from_path(".env").unwrap();
        let db = db::connect().await.unwrap();
        let llm = Arc::new(Ollama::default());
        let search = Arc::new(SearxNG::new("http://localhost:8081"));
        scraper::setup_browser().await.unwrap();
        let browser = scraper::INSTANCE.get().unwrap().clone();
        let creation = ReportCreation::new("Apple 2025 Q4 outlook".into());
        let report: SurrealDBReport = db
            .create("report")
            .content(creation)
            .await
            .unwrap()
            .unwrap();
        dbg!(&report);
        let job = TitleJob;
        job.run(&report, db, llm, search, browser).await.unwrap();
    }
}
