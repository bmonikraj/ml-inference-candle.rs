use std::collections::HashMap;

use super::writer::Writer;

pub struct WriterEcho {
    config: Option<HashMap<String, HashMap<String, String>>>,
}

impl Writer for WriterEcho {
    fn init(&mut self, config: &std::collections::HashMap<String, std::collections::HashMap<String, String>>) {
        self.config = Some(config.clone());
    }

    fn persist(&mut self, payload: &crate::model::response_message::ResponseMessage) -> Result<bool, Box<dyn std::error::Error>> {
        let binding_endpoint = self.config.clone().unwrap();
        let endpoint = binding_endpoint["writer"]["endpoint"].as_str();
        let binding_path = self.config.clone().unwrap();
        let base_path = binding_path["writer"]["base_path"].as_str();
        let url = format!("{}/{}", endpoint, base_path);

        let client = reqwest::Client::new();
        let string_payload = serde_json::to_string(&payload).unwrap();
        let response_from_http = futures::executor::block_on(async {
            client.post(url).body(string_payload).send().await?.text().await
        });
        match response_from_http {
            Ok(r) => {
                log::info!("echo response: \n{}", r);
                return Ok(true);
            },
            Err(e) => {
                return Err(format!("echo sending error: {}", e).into());
            },
        }
    }
}

impl WriterEcho {
    pub fn new() -> Self {
        return Self {
            config: None,
        };
    }
}

#[cfg(test)]
mod test_writer_echo {
    use crate::model::response_message::ResponseMessage;

    use super::*;

    #[test]
    fn test_init() {
        let c: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut w = WriterEcho::new();
        w.init(&c);
    }

    #[test]
    #[should_panic]
    fn test_persist() {
        let c: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut w = WriterEcho::new();
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