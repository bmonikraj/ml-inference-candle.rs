use std::error::Error;

use super::{writer::Writer, writer_echo::WriterEcho, writer_fs::WriterFs};

pub fn get_writer(type_of_writer: &String) -> Result<Box<dyn Writer>, Box<dyn Error>> {
    match type_of_writer.as_str() {
        "fs" => Ok(Box::new(WriterFs::new())),
        "echo" => Ok(Box::new(WriterEcho::new())),
        _ => Err(format!("{} is not a valid writer type", type_of_writer).into()),
    }
}

#[cfg(test)]
mod tests_factory_writer {
    use super::*;
    use std::any::type_name_of_val;

    #[test]
    fn test_get_writer_invalid() {
        let param = String::from("invalid");
        assert!(get_writer(&param).is_err());
    }

    #[test]
    fn test_get_writer_valid_fs() {
        let param = String::from("fs");
        assert!(get_writer(&param).is_ok());
        let r = get_writer(&param).unwrap();
        assert_eq!(
            type_name_of_val(&r),
            "alloc::boxed::Box<dyn ml_inference_candle::outbound::writer::Writer>"
        );
    }

    #[test]
    fn test_get_consumer_valid_echo() {
        let param = String::from("echo");
        assert!(get_writer(&param).is_ok());
        let r = get_writer(&param).unwrap();
        assert_eq!(
            type_name_of_val(&r),
            "alloc::boxed::Box<dyn ml_inference_candle::outbound::writer::Writer>"
        );
    }
}
