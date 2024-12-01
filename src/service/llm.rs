use std::collections::HashMap;

use std::error::Error;

use crate::{model::request_message::RequestMessage, outbound::writer::Writer};

pub trait Llm {
    fn load(&mut self, config: &HashMap<String, HashMap<String, String>>);
    fn generate_and_persist(&mut self, request: &RequestMessage, writer: &mut Box<dyn Writer>) -> Result<bool, Box<dyn Error>>;
}
