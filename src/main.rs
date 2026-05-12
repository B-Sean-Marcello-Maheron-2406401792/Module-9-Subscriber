use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{
    options::*, types::FieldTable, Connection,
    ConnectionProperties,
};
use futures_lite::StreamExt;

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() {
    let addr = "amqp://guest:guest@localhost:5672";
    let conn = Connection::connect(addr, ConnectionProperties::default())
        .await
        .unwrap();

    let channel = conn.create_channel().await.unwrap();

    channel
        .queue_declare(
            "user_created",
            QueueDeclareOptions {
                durable: false,
                auto_delete: false,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await
        .unwrap();

    println!(" [*] Menunggu pesan di Sean's Computer [2406401792]. Untuk keluar tekan CTRL+C");

    let mut consumer = channel
        .basic_consume(
            "user_created",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    // Sekarang consumer.next() bisa ditemukan karena StreamExt sudah di-import
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");

        let message = UserCreatedEventMessage::try_from_slice(&delivery.data).unwrap();

        let ten_millis = std::time::Duration::from_millis(1000);

        // Di dalam loop/while subscriber
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;

        println!("In Sean's Computer [2406401792]. Message received: {:?}", message);

        delivery
            .ack(BasicAckOptions::default())
            .await
            .expect("ack");
    }
}