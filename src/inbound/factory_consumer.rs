use std::error::Error;

use super::{consumer::Consumer, consumer_nats::ConsumerNats};

pub fn get_consumer(type_of_consumer: &String) -> Result<Box<dyn Consumer>, Box<dyn Error>> {
    match type_of_consumer.as_str() {
        "nats" => Ok(Box::new(ConsumerNats::new())),
        "kafka" => todo!(),
        _ => Err(format!("{} is not a valid consumer type", type_of_consumer).into()),
    }
}
