use std::collections::HashMap;

use crate::{
    outbound::{client_grpc::ClientGrpc, writer::Writer},
    service::llm::Llm,
};

pub trait Consumer {
    /**
     * start() could have been tokio::main entry method, but if done,
     * then other impl of this trait will bound to follow async start() method
     */
    fn start(
        &mut self,
        config: &HashMap<String, HashMap<String, String>>,
        llm: &mut Box<dyn Llm>,
        writer: &mut Box<dyn Writer>,
    );
}
