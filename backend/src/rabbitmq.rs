use std::sync::Arc;

use crate::db::SurrealDb;
use crate::llm::LLMApi;
use crate::models::ReportStatusEvent;
use crate::scraper::BrowserWrapper;
use crate::search::SearchEngine;
use crate::{prelude::*, workflow};
use futures_util::TryStreamExt;
use lapin::message::Delivery;
use lapin::options::{BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties};

pub struct RabbitMQPublisher {
    connection: Connection,
    channel: Channel,
}

impl RabbitMQPublisher {
    pub async fn new() -> Result<Self> {
        // TODO: change the connection string to use env variable
        let connection =
            Connection::connect("amqp://localhost", ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        Ok(Self {
            connection,
            channel,
        })
    }

    pub async fn publish_report_status(&self, message: ReportStatusEvent) -> Result<String> {
        let message = serde_json::to_string(&message)?;
        let queue = self
            .channel
            .queue_declare(
                "report_status",
                QueueDeclareOptions::default(),
                Default::default(),
            )
            .await?;
        self.channel
            .basic_publish(
                "",
                queue.name().as_str(),
                BasicPublishOptions::default(),
                message.as_bytes(),
                BasicProperties::default(),
            )
            .await?;
        self.connection.close(200, "Connection closed").await?;
        Ok("Report status published successfully".to_string())
    }
}

pub struct RabbitMQConsumer {
    // connection: Connection,
    channel: Channel,
    // queue: Queue,
    queue_name: String,
}

impl RabbitMQConsumer {
    pub async fn new() -> Result<Self> {
        // TODO: change the connection string to use env variable
        let connection =
            Connection::connect("amqp://localhost", ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        let queue = channel
            .queue_declare(
                "report_status",
                QueueDeclareOptions::default(),
                Default::default(),
            )
            .await?;
        let queue_name = queue.name().to_string();
        Ok(RabbitMQConsumer {
            // connection,
            channel,
            // queue,
            queue_name,
        })
    }

    pub async fn consume_report_status(
        &self,
        db: SurrealDb,
        llm: Arc<dyn LLMApi>,
        search: Arc<dyn SearchEngine>,
        browser: BrowserWrapper,
    ) -> Result<()> {
        let publisher = Arc::new(RabbitMQPublisher::new().await?);
        let consumer = self
            .channel
            .basic_consume(
                &self.queue_name,
                "report_status_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        async fn consume(
            delivery: &Delivery,
            publisher: Arc<RabbitMQPublisher>,
            db: SurrealDb,
            llm: Arc<dyn LLMApi>,
            search: Arc<dyn SearchEngine>,
            browser: BrowserWrapper,
        ) -> Result<()> {
            let message = String::from_utf8_lossy(&delivery.data);
            let report_status: ReportStatusEvent = serde_json::from_str(&message)?;
            workflow::run_next_job(&report_status.report_id, db, llm, search, browser).await?;
            publisher.publish_report_status(report_status).await?;
            Ok(())
        }
        let messages = consumer.try_for_each(|delivery| {
            let publisher = publisher.clone();
            let db = db.clone();
            let llm = llm.clone();
            let search = search.clone();
            let browser = browser.clone();
            async move {
                let result = consume(
                    &delivery,
                    publisher,
                    db.clone(),
                    llm.clone(),
                    search.clone(),
                    browser.clone(),
                )
                .await;
                if result.is_err() {
                    panic!("{}", result.unwrap_err());
                }
                self.channel
                    .basic_ack(delivery.delivery_tag, Default::default())
                    .await?;
                Ok(())
            }
        });
        Ok(messages.await?)
    }
}

#[cfg(test)]
mod tests {
    use workflow::ReportStatus;

    use super::*;
    use crate::models::ReportStatusEvent;

    #[tokio::test]
    #[ignore = "Depends on extenal service"]
    async fn test_rabbitmq_publisher() {
        let publisher = RabbitMQPublisher::new().await.unwrap();
        let message = ReportStatusEvent {
            report_id: "123".to_string(),
            status: ReportStatus::Pending,
        };
        let result = publisher.publish_report_status(message).await.unwrap();
        assert_eq!(result, "Report status published successfully");
    }

    #[tokio::test]
    #[ignore = "Depends on extenal service"]
    async fn test_rabbitmq_consumer() {
        // let consumer = RabbitMQConsumer::new().await.unwrap();
        // consumer.consume_report_status().await.unwrap();
    }
}
