use lapin::{BasicProperties, Channel, Connection, ConnectionProperties};
use lapin::options::BasicPublishOptions;
use crate::models::ReportStatusEvent;
use crate::prelude::FinanalizeError;

pub struct RabbitMQPublisher {
    connection: Connection,
    channel: Channel,
}

impl RabbitMQPublisher {
    pub async fn new() -> Result<Self, FinanalizeError> {
        // TODO: change the connection string to use env variable
        let connection = Connection::connect("amqp://localhost", ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        Ok(Self { connection, channel })
    }

    pub async fn publish_report_status(&self, message: ReportStatusEvent) -> Result<String, FinanalizeError> {
        let message = serde_json::to_string(&message)?;
        self.channel.basic_publish(
            "report_status",
            "report_status",
            BasicPublishOptions::default(),
            message.as_bytes(),
            BasicProperties::default(),
        ).await?;
        self.connection.close(200, "Connection closed").await?;
        Ok("Report status published successfully".to_string())
    }
}

pub struct RabbitMQConsumer {
    // TODO: define
}

impl RabbitMQConsumer {
    // TODO: define
}