use std::collections::HashMap;

use crate::{
    outbound::{client_grpc::ClientGrpc, writer::Writer},
    service::llm::Llm,
};

use super::{consumer::Consumer, worker_nats};

pub struct ConsumerNats {}

impl Consumer for ConsumerNats {
    fn start(
        &mut self,
        config: &HashMap<String, HashMap<String, String>>,
        llm: &mut Box<dyn Llm>,
        writer: &mut Box<dyn Writer>,
    ) {
        log::info!("starting nats consumer...");
        let _ = worker_nats::main_worker(config, llm, writer);
    }
}

impl ConsumerNats {
    pub fn new() -> Self {
        return Self {};
    }
}

#[cfg(test)]
mod test_consumer_nats {
    use crate::{outbound::writer_echo::WriterEcho, service::llm_smol::LLMSmol};

    use super::*;

    #[test]
    #[should_panic]
    fn test_start() {
        let mut c = ConsumerNats::new();
        let config: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut llm: Box<dyn Llm> = Box::new(LLMSmol::new());
        let mut writer: Box<dyn Writer> = Box::new(WriterEcho::new());
        let mut g_client = ClientGrpc::new();
        c.start(&config, &mut llm, &mut writer);
    }
}
