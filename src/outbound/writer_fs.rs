use std::{collections::HashMap, fs::File, io::Write};

use crate::model::response_message::ResponseMessage;

use super::writer::Writer;

pub struct WriterFs {
    config: Option<HashMap<String, HashMap<String, String>>>,
}

impl Writer for WriterFs {
    fn init(&mut self, config: &std::collections::HashMap<String, std::collections::HashMap<String, String>>) {
        self.config = Some(config.clone());
    }

    fn persist(&mut self, payload: &ResponseMessage) -> Result<bool, Box<dyn std::error::Error>> {
        let binding_endpoint = self.config.clone().unwrap();
        let endpoint = binding_endpoint["writer"]["endpoint"].as_str();
        let binding_path = self.config.clone().unwrap();
        let base_path = binding_path["writer"]["base_path"].as_str();
        
        log::info!("opening file = {}", format!("{}/{}/{}.json", endpoint, base_path, payload.name));
        let mut file_handler = File::create(format!("{}/{}/{}.json", endpoint, base_path, payload.name)).unwrap();

        let string_payload = serde_json::to_string_pretty(&payload).unwrap();
        match file_handler.write(string_payload.as_bytes()) {
            Ok(_) => {
                log::info!("data written to the file = {}", payload.name);
                return Ok(true);
            },
            Err(e) => {
                return Err(format!("file writing error: {}", e).into());
            },
        }
    }
}

impl WriterFs {
    pub fn new() -> Self {
        return Self {
            config: None,
        };
    }
}

#[cfg(test)]
mod test_writer_fs {
    use super::*;

    #[test]
    fn test_init() {
        let c: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut w = WriterFs::new();
        w.init(&c);
    }

    #[test]
    #[should_panic]
    fn test_persist() {
        let c: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut w = WriterFs::new();
        w.init(&c);
        let p = ResponseMessage {
            id: "x".to_string(),
            name: "x".to_string(),
            created_at: "x".to_string(),
            query: "x".to_string(),
            answer: "x".to_string(),
            embedding: vec![0.0],
        };
        let _ = w.persist(&p);
    }
}