use std::sync::Arc;
use crate::llm::API;
use crate::prelude::*;
use crate::prompting;
use crate::tasks::Task;
use crate::workflow::job::Job;
use crate::workflow::WorkflowState;
use async_trait::async_trait;
use chrono::Utc;
use log::debug;
use markup5ever::interface::TreeSink;
use regex::Regex;
use schemars::schema_for;
use scraper::{Html, HtmlTreeSink, Selector};
use tokio::sync::{Mutex, Semaphore};
use tokio::task::{JoinHandle, JoinSet};
use crate::models::PreClassificationSource;
use crate::workflow::job::search_before_questions::models::SingleSearchOutput;
use crate::search::SEARCH;
use crate::workflow::job::content_formatter::models::FormatContentJobInput;
use crate::workflow::job::scrape_pages::scrape_page;

pub struct SearchBeforeQuestionsJob;

pub mod models {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SingleSearchInput {
        pub input: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    pub struct SingleSearchOutput {
        pub companies: Vec<String>,
    }
}

#[async_trait]
impl Job for SearchBeforeQuestionsJob {
    async fn run(&self, mut state: WorkflowState) -> Result<WorkflowState> {
        // Generate initial search query
        let user_input = state.state.user_input.clone();
        let prompt = prompting::get_prompt("single-search".into())?;
        let task = Task::new(&prompt);
        let prompt_input = models::SingleSearchInput {
            input: user_input.clone(),
        };
        let search_output: SingleSearchOutput = task
            .run_structured(
                API.clone(),
                &prompt_input,
                serde_json::to_string_pretty(&schema_for!(SingleSearchOutput))?,
            )
            .await?;
        for company in &search_output.companies {
            debug!("Company: {:#?}", company);
        };
        let search_query = format!("{:#?} recent events", search_output.companies.join(" and"));
        debug!("Search query: {}", search_query);
        dbg!(&search_query);
        // Perform single search
        let mut search_futures = JoinSet::new();
        search_futures.spawn(async move { SEARCH.clone().search(&search_query).await });
        let search_results = search_futures.join_all().await;
        let mut all_urls = Vec::new();
        for result in search_results.into_iter() {
            debug!("Adding search result: {:?}", result);
            all_urls.extend(result?);
        }
        // Sort and deduplicate URLs
        debug!("Sorting and deduplicating URLs");
        all_urls.sort();
        all_urls.dedup();
        debug!("Search results: {:?}", all_urls.len());
        // Scrape top 3 url's
        let sources: Arc<Mutex<Vec<PreClassificationSource>>> = Arc::new(Mutex::new(vec![]));
        let browsers = Arc::new(crate::workflow::job::scrape_pages::make_browsers(1).await?);
        let total_to_search = 3;
        let sources_to_search = all_urls.get(..3).unwrap().to_vec();
        let mut join_set = JoinSet::new();
        for(i, source) in sources_to_search.into_iter().enumerate() {
            let browser = browsers.clone();
            let sources = sources.clone();
            debug!("Spawning task for scraping URL {}: {}", i + 1, source);
            join_set.spawn(async move {
                let Ok(browser) = browser.clone().get().await else {
                    return Err(FinanalizeError::InternalServerError);
                };
                debug!("Scraping ({}/{}): {}", i + 1, total_to_search, source);
                let Ok(html) = scrape_page(&browser, &source).await else {
                    debug!("Failed to scrape page: {}", source);
                    // return Err(FinanalizeError::InternalServerError);
                    return Ok(());
                };
                debug!("Scraped ({}/{}): {}", i + 1, total_to_search, &source);
                sources.clone().lock().await.push(PreClassificationSource {
                    url: source.to_string(),
                    content: html,
                });
                Ok(())
            });
        }

        let results: Result<Vec<()>> = join_set.join_all().await.into_iter().collect();
        results?;
        debug!("Scraped all pages, closing browser instances...");
        let browser = browsers.remove().await?;
        browser.close().await?;
        
        let scraped_html_sources = sources.lock().await.clone();
        // Extract top 3 sources content
        let mut mds = vec![];
        let total = scraped_html_sources.len();
        let pattern = Regex::new("(?i)<span[^>]*>")?;
        for (i, source) in scraped_html_sources.into_iter().enumerate() {
            debug!("Extracting content from HTML source ({}/{})", i + 1, total);
            // let content = extractor
            //     .extract(FileType::Html(html))
            //     .await?
            //     .into_iter()
            //     .next()
            //     .ok_or(FinanalizeError::ParseError("No content extracted".into()))?;
            // match content {
            //     Content::MarkDown(md) => mds.push(md),
            //     _ => continue,
            // }
            let document = Html::parse_document(&source.content);
            // Selectors for header and footer
            let header_selector = Selector::parse("header")?;
            let footer_selector = Selector::parse("footer")?;

            let mut removables = vec![];

            // Remove the selected elements
            for node in document.select(&header_selector).collect::<Vec<_>>() {
                removables.push(node.id());
            }

            for node in document.select(&footer_selector).collect::<Vec<_>>() {
                removables.push(node.id());
            }

            let tree = HtmlTreeSink::new(document);

            for removable in removables {
                tree.remove_from_parent(&removable);
            }

            let filtered: String = tree.finish().html();

            let mut md = mdka::from_html(&filtered);

            // Replace
            md = pattern.replace_all(&md, "").to_string();
            md = md.replace("</span>", "");
            md = md
                .trim()
                .lines()
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
                .join("\n");

            mds.push(PreClassificationSource {
                url: source.url,
                content: md,
            })
        }
        // Format top 3 sources content
        let prompt = prompting::get_prompt("source-formatter".into())?;
        let task = Task::new(&prompt);
        let md_sources = state.state.md_sources.clone().unwrap();
        let max_jobs = 1;
        let sem = Arc::new(Semaphore::new(max_jobs));
        let len = md_sources.len();
        let mut handles = vec![];
        for (i, source) in md_sources.into_iter().enumerate() {
            let task = task.clone();
            let sem = sem.clone();
            let handle: JoinHandle<Result<PreClassificationSource>> = tokio::spawn(async move {
                let permit = sem.acquire().await.unwrap();
                debug!("Formatting source {} of {}", i + 1, len);
                let input = FormatContentJobInput {
                    date: Utc::now().format("%Y-%m-%d").to_string(),
                    content: source.content,
                    url: source.url,
                };
                let output = task.run_raw(API.clone(), &input).await?;
                let source = PreClassificationSource {
                    url: input.url,
                    content: output,
                };
                drop(permit);
                Ok(source)
            });
            handles.push(handle);
        }
        let mut sources = Vec::new();
        for handle in handles {
            let source = handle.await??;
            sources.push(source);
        }
        state.state.initial_search_sources = Some(sources);
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        models::FullReport,
        workflow::{job::validation::models::ValidationOutput, JobType, WorkflowState},
    };

    #[tokio::test]
    // #[ignore = "Uses LLM API (External Service)"]
    async fn test_search_before_questions_job_company() {
        env_logger::init();
        let job = SearchBeforeQuestionsJob;
        let state = WorkflowState {
            id: "tlksajbdfaln".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("sjaudnhcrlas".into(), "Tesla stock in 2025".into())
                .with_validation(ValidationOutput {
                    valid: true,
                    error: None,
                }),
        };
        job.run(state).await.unwrap().state.initial_search_sources.unwrap(); 
    }


    #[tokio::test]
    // #[ignore = "Uses LLM API (External Service)"]
    async fn test_search_before_questions_job_companies() {
        env_logger::init();
        let job = SearchBeforeQuestionsJob;
        let state = WorkflowState {
            id: "tlksajbdfaln".into(),
            last_job_type: JobType::Pending,
            state: FullReport::new("sjaudnhcrlas".into(), "Apple and Microsoft stocks in 2025".into())
                .with_validation(ValidationOutput {
                    valid: true,
                    error: None,
                }),
        };
        job.run(state).await.unwrap().state.initial_search_sources.unwrap(); 
    }
}
