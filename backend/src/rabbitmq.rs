use std::sync::Arc;

use crate::{prelude::*, workflow};
use futures_util::TryStreamExt;
use lapin::options::{BasicConsumeOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{Channel, Connection, ConnectionProperties, Queue};
use log::debug;
use tokio::sync::OnceCell;

#[derive(Debug)]
pub struct RabbitMQPublisher {
    pub channel: Channel,
    pub queue: Queue,
}

pub static PUBLISHER: OnceCell<Arc<RabbitMQPublisher>> = OnceCell::const_new();

impl RabbitMQPublisher {
    pub async fn setup() -> Result<()> {
        debug!("Setting up RabbitMQ publisher...");
        // TODO: change the connection string to use env variable
        let mut default_address = "amqp://localhost".into();
        if let Ok(env_address) = std::env::var("RABBITMQ_URL") {
            default_address = env_address;
        }
        let connection =
            Connection::connect(&default_address, ConnectionProperties::default()).await?;
        debug!("Connected to RabbitMQ instance");
        let channel = connection.create_channel().await?;
        debug!("Created channel: {:?}", channel.id());
        let queue = channel
            .queue_declare(
                "report_status",
                QueueDeclareOptions::default(),
                Default::default(),
            )
            .await?;
        debug!("Declared queue: {:?}", queue.name());
        PUBLISHER.set(Arc::new(Self { channel, queue })).unwrap();
        debug!("RabbitMQ publisher setup complete");
        Ok(())
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
        debug!("Setting up RabbitMQ consumer...");
        // TODO: change the connection string to use env variable
        let mut default_address = "amqp://localhost".into();
        if let Ok(env_address) = std::env::var("RABBITMQ_URL") {
            default_address = env_address;
        }
        let connection =
            Connection::connect(&default_address, ConnectionProperties::default()).await?;
        debug!("Connected to RabbitMQ instance");
        let channel = connection.create_channel().await?;
        debug!("Created channel: {:?}", channel.id());
        let queue = channel
            .queue_declare(
                "report_status",
                QueueDeclareOptions::default(),
                Default::default(),
            )
            .await?;
        let queue_name = queue.name().to_string();
        debug!("Declared queue: {:?}", queue_name);
        Ok(RabbitMQConsumer {
            // connection,
            channel,
            // queue,
            queue_name,
        })
    }

    pub async fn consume_report_status(&self) -> Result<()> {
        let consumer = self
            .channel
            .basic_consume(
                &self.queue_name,
                "report_status_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        debug!("Consuming messages from queue: {:?}", self.queue_name);
        Ok(consumer
            .try_for_each(|delivery| async move {
                let res = workflow::consume_report_status(&self.channel, &delivery).await;
                if res.is_err() {
                    println!("Error consuming message: {:?}", res.unwrap_err());
                }
                Ok(())
            })
            .await?)
    }
}
