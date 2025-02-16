use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::ser::PrettyFormatter;
use surrealdb::sql::Thing;

use crate::{models::SurrealDBReport, prelude::*, prompting, tasks::Task};

use std::sync::Arc;

use crate::{db::SurrealDb, llm::LLMApi, scraper::BrowserWrapper, search::SearchEngine};

use super::Job;

#[derive(Debug, Serialize, Deserialize)]
struct HeadingDetail {
    heading: String,
    paragraphs: Vec<String>,
}

#[derive(Debug, Serialize)]
struct SearchGenerationTaskInput {
    #[serde(rename = "currentDate")]
    current_date: String,
    headings: String,
    #[serde(rename = "firstHeading")]
    first_heading: String,
}

#[derive(Debug, Deserialize)]
struct HeadingQueries {
    heading: String,
    search_queries: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct SearchGenerationTaskOutput {
    headings: Vec<HeadingQueries>,
}

#[derive(Debug, Serialize)]
struct SearchQuery {
    query: String,
}

#[derive(Debug, Deserialize)]
struct SDBSearchQuery {
    id: Thing,
    query: String,
}

pub struct SearchGenerationJob;

#[async_trait]
impl Job for SearchGenerationJob {
    async fn run(
        &self,
        report: &SurrealDBReport,
        db: SurrealDb,
        llm: Arc<dyn LLMApi>,
        _search: Arc<dyn SearchEngine>,
        _browser: BrowserWrapper,
    ) -> Result<()> {
        let prompt = prompting::get_prompt(db.clone(), "searchquery".into()).await?;
        let search_generation_task = Task::new(&prompt);
        let date = Utc::now().format("%Y-%m-%d").to_string();

        let paragraphs: Vec<HeadingDetail> = db
            .query(
                "SELECT * FROM (SELECT ->has_paragraph->paragraph as paragraphs FROM \
                 $report)[0].paragraphs;",
            )
            .bind(("report", report.id.clone()))
            .await?
            .take(0)?;

        fn ser<T: Serialize>(obj: T) -> Result<String> {
            let mut buf = Vec::new();
            let formatter = PrettyFormatter::with_indent(b"    ");
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            obj.serialize(&mut ser)?;
            Ok(String::from_utf8(buf)?)
        }

        let headings_json = ser(&paragraphs)?;

        let search_generation_input = SearchGenerationTaskInput {
            current_date: date,
            headings: headings_json,
            first_heading: paragraphs[0].heading.clone(),
        };

        let output: SearchGenerationTaskOutput = search_generation_task
            .run(llm, &search_generation_input)
            .await?;

        let mut all_queries = vec![];
        for heading in output.headings {
            for query in heading.search_queries {
                all_queries.push(SearchQuery { query });
            }
        }

        for query in all_queries {
            let sdb_query: SDBSearchQuery = db
                .create("search_query")
                .content(query)
                .await?
                .ok_or(FinanalizeError::UnableToSaveSearchQuery)?;
            db.query("RELATE $report -> has_search_query -> $query")
                .bind(("report", report.id.clone()))
                .bind(("query", sdb_query.id.clone()))
                .await?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use surrealdb::sql::Thing;

    use crate::{db, llm::ollama::Ollama, models::ReportCreation, scraper, search::SearxNG};

    use super::*;

    #[derive(Debug, Deserialize)]
    struct SDBHeading {
        id: Thing,
        report_section_heading: String,
        description: String,
    }

    #[derive(Debug, Serialize)]
    struct Heading {
        report_section_heading: String,
        description: String,
    }

    #[derive(Debug, Deserialize)]
    struct SDBParagraph {
        id: Thing,
        heading: String,
        paragraph: Vec<String>,
    }

    #[derive(Debug, Serialize)]
    struct Paragraph {
        heading: String,
        paragraph: Vec<String>,
    }

    #[derive(Debug)]
    struct Combo {
        heading: Heading,
        paragraphs: Vec<String>,
    }

    #[tokio::test]
    #[ignore = "Depends on external service"]
    async fn test() {
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
        let paragraphs = vec![
            Paragraph {
                heading: "Overview of Apple Stock".into(),
                paragraph: vec![
                    "Provide a brief introduction to Apple Inc., highlighting its position as a \
                     leading technology company and its significance in the market."
                        .into(),
                    "Discuss Apple’s stock ticker (AAPL) and its listing on the Nasdaq Stock \
                     Market, including its historical listing date and key milestones."
                        .into(),
                    "Examine Apple’s industry presence, particularly in consumer electronics, \
                     smartphones, software, and services, and its market share compared to \
                     competitors."
                        .into(),
                    "Highlight Apple’s historical significance, including its role in the \
                     personal computing revolution, influential product launches, and its impact \
                     on consumer behavior."
                        .into(),
                ],
            },
            Paragraph {
                heading: "Recent Market Performance".into(),
                paragraph: vec![
                    "Analyze Apple’s recent stock movements, focusing on key price points, \
                     significant gains, and losses over the past few weeks or months."
                        .into(),
                    "Discuss any important trading volume trends, noting periods of high activity \
                     and their correlation with specific company events or market conditions."
                        .into(),
                ],
            },
        ];
        for paragraph in paragraphs {
            let sdb_paragraph: SDBParagraph = db
                .create("paragraph")
                .content(paragraph)
                .await
                .unwrap()
                .unwrap();
            let result = db
                .query("RELATE $report -> has_paragraph -> $paragraph")
                .bind(("report", report.id.clone()))
                .bind(("paragraph", sdb_paragraph.id.clone()))
                .await
                .unwrap();
            dbg!(result);
        }
        let job = SearchGenerationJob;
        job.run(&report, db, llm, search, browser).await.unwrap();
    }
}
