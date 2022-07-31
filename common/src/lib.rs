pub use kafka_consumer::KafkaConsumer;
pub use kafka_producer::KafkaProducer;

pub mod async_fn;
mod kafka_consumer;
mod kafka_producer;
pub mod recipe;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
