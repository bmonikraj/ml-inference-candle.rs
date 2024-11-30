use std::error::Error;

use super::{consumer::Consumer, consumer_nats::ConsumerNats};

pub fn get_consumer(type_of_consumer: &String) -> Result<Box<dyn Consumer>, Box<dyn Error>> {
    match type_of_consumer.as_str() {
        "nats" => Ok(Box::new(ConsumerNats::new())),
        "kafka" => todo!(),
        _ => Err(format!("{} is not a valid consumer type", type_of_consumer).into()),
    }
}

#[cfg(test)]
mod tests_factory_consumer {
    use super::*;
    use std::any::type_name_of_val;

    #[test]
    fn test_get_consumer_invalid() {
        let param = String::from("invalid");
        assert!(get_consumer(&param).is_err());
    }

    #[test]
    fn test_get_consumer_valid_nats() {
        let param = String::from("nats");
        assert!(get_consumer(&param).is_ok());
        let r = get_consumer(&param).unwrap();
        assert_eq!(
            type_name_of_val(&r),
            "alloc::boxed::Box<dyn ml_inference_candle::inbound::consumer::Consumer>"
        );
    }

    #[test]
    #[should_panic]
    fn test_get_consumer_valid_kafka() {
        let param = String::from("kafka");
        assert!(get_consumer(&param).is_ok());
        let r = get_consumer(&param).unwrap();
        assert_eq!(type_name_of_val(&r), "ConsumerNats");
    }
}
