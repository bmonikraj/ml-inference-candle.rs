use std::collections::HashMap;

use crate::service::llm::Llm;

use super::{consumer::Consumer, worker_nats};

pub struct ConsumerNats {}

impl Consumer for ConsumerNats {
    fn start(&mut self, config: &HashMap<String, HashMap<String, String>>, llm: &mut Box<dyn Llm>) {
        log::info!("starting nats consumer...");
        let _ = worker_nats::main_worker(config, llm);
    }
}

impl ConsumerNats {
    pub fn new() -> Self {
        return Self {};
    }
}
