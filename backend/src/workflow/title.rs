use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{models::SurrealDBReport, prelude::*, prompting, tasks::Task};

use std::sync::Arc;

use crate::{db::SurrealDb, llm::LLMApi, scraper::BrowserWrapper, search::SearchEngine};

use super::Job;

#[derive(Debug, Serialize)]
struct TitleTaskInpput {
    user_input: String,
}

#[derive(Debug, Deserialize)]
struct TitleTaskOutput {
    output: String,
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
            user_input: report.user_input.clone(),
        };
        let title_output: TitleTaskOutput = title_task.run(llm, &title_input).await?;

        let report_title = ReportTitle {
            title: title_output.output,
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
