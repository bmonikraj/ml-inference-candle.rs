use std::collections::HashMap;

use std::error::Error;

use crate::{
    model::request_message::RequestMessage,
    outbound::{client_grpc::ClientGrpc, writer::Writer},
};

pub trait Llm {
    fn generate_and_persist(
        &mut self,
        request: &RequestMessage,
        writer: &mut Box<dyn Writer>,
        client: &mut ClientGrpc,
    ) -> Result<bool, Box<dyn Error>>;
}
