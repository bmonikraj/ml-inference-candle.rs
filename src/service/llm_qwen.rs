use std::error::Error;

use crate::{model::{request_message::RequestMessage, response_message::ResponseMessage}, outbound::writer::Writer};

use super::llm::Llm;

pub struct LLMQwen {
}

impl Llm for LLMQwen {
    fn load(
        &mut self,
        config: &std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    ) {
        log::info!("LLMQwen invoked for loading")
    }

    fn generate_and_persist(&mut self, request: &RequestMessage, writer: &mut Box<dyn Writer>) -> Result<bool, Box<dyn Error>>{
        log::info!("LLMQwen invoked for generation");
        if 1==1 {
            let r = ResponseMessage {
                id: "s1".to_string(),
                name: "s2".to_string(),
                created_at: "s3".to_string(),
                query: "s4".to_string(),
                answer: "s5".to_string(),
                embedding: vec![1.0],
            };
            match writer.persist(&r) {
                Ok(w) => return Ok(w),
                Err(e) => return  Err(e),
            };
        } else {
            return Err("e".into());
        }
    }
}

impl LLMQwen {
    pub fn new() -> Self {
        return Self {};
    }
}
