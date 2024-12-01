use crate::model::response_message::ResponseMessage;
use std::{collections::HashMap, error::Error};

pub trait Writer {
    fn init(&mut self, config: &HashMap<String, HashMap<String, String>>);
    fn persist(&mut self, payload: &ResponseMessage) -> Result<bool, Box<dyn Error>>;
}