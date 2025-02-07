use crate::models::ReportStatusEvent;
use crate::prelude::FinanalizeError;
use futures_util::TryStreamExt;
use lapin::options::{BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties};

pub struct RabbitMQPublisher {
    connection: Connection,
    channel: Channel,
}

impl RabbitMQPublisher {
    pub async fn new() -> Result<Self, FinanalizeError> {
        // TODO: change the connection string to use env variable
        let connection =
            Connection::connect("amqp://localhost", ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        Ok(Self {
            connection,
            channel,
        })
    }

    pub async fn publish_report_status(
        &self,
        message: ReportStatusEvent,
    ) -> Result<String, FinanalizeError> {
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
    pub async fn new() -> Result<Self, FinanalizeError> {
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

    pub async fn consume_report_status(&self) -> Result<(), FinanalizeError> {
        let consumer = self
            .channel
            .basic_consume(
                &self.queue_name,
                "report_status_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        let messages = consumer.try_for_each(|delivery| async move {
            let message = String::from_utf8_lossy(&delivery.data);
            println!("Received message: {:?}", message);
            self.channel
                .basic_ack(delivery.delivery_tag, Default::default())
                .await?;
            Ok(())
        });
        Ok(messages.await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ReportStatus, ReportStatusEvent};

    #[tokio::test]
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
    async fn test_rabbitmq_consumer() {
        let consumer = RabbitMQConsumer::new().await.unwrap();
        consumer.consume_report_status().await.unwrap();
    }
}
