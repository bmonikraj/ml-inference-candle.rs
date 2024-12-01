use std::error::Error;

use crate::{model::request_message::RequestMessage, outbound::writer::Writer};

use super::llm::Llm;

pub struct LLMSmol {}

impl Llm for LLMSmol {
    fn load(
        &mut self,
        config: &std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    ) {
        log::info!("LLMSmol invoked for loading")
    }

    fn generate_and_persist(&mut self, request: &RequestMessage, writer: &mut Box<dyn Writer>) -> Result<bool, Box<dyn Error>> {
        log::info!("LLMSmol invoked for generation");
        if 1==1 {
            return Ok(true);
        } else {
            return Err("e".into());
        }
    }
}

impl LLMSmol {
    pub fn new() -> Self {
        return Self {};
    }
}
