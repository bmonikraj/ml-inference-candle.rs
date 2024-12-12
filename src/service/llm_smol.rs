use std::{collections::HashMap, error::Error};

use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use tonic::{Code, Status};
use uuid::Uuid;

use crate::{
    model::{
        grpc::inference::{ModelInferRequest, ModelInferResponse},
        request_message::RequestMessage,
        response_message::ResponseMessage,
    },
    outbound::{client_grpc::ClientGrpc, writer::Writer},
};

use super::llm::Llm;

pub struct LLMSmol {}

impl Llm for LLMSmol {
    fn generate_and_persist(
        &mut self,
        request: &RequestMessage,
        writer: &mut Box<dyn Writer>,
        client: &mut ClientGrpc,
    ) -> Result<bool, Box<dyn Error>> {
        log::info!("LLMSmol invoked for generation");
        let query_encoded = general_purpose::STANDARD.encode(&request.query);

        let grpc_request = tonic::Request::new(ModelInferRequest {
            model_name: "llm".to_string(),
            model_version: "version".to_string(),
            id: "id".to_string(),
            parameters: HashMap::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            raw_input_contents: vec![query_encoded.into()],
        });

        let grpc_result =
            futures::executor::block_on(async { client.get().model_infer(grpc_request).await });

        let mut grpc_result_status = Status::new(Code::Unavailable, "yet to be figured out");
        let grpc_result_response = match grpc_result {
            Ok(r) => r,
            Err(s) => {
                grpc_result_status = s;
                tonic::Response::new(ModelInferResponse {
                    model_name: "xxx".to_string(),
                    model_version: "".to_string(),
                    id: "".to_string(),
                    parameters: HashMap::new(),
                    outputs: Vec::new(),
                    raw_output_contents: Vec::new(),
                })
            }
        };

        if grpc_result_status.code() == Code::Ok {
            let grpc_result_response_inner = grpc_result_response.get_ref();
            log::debug!("response => {:#?}", grpc_result_response_inner);
            let answer_from_grpc = grpc_result_response_inner.raw_output_contents[0].clone();
            log::info!("raw_output_contents[0] is {:?}", answer_from_grpc);

            let r = ResponseMessage {
                id: Uuid::new_v4().to_string(),
                name: grpc_result_response_inner.model_name.clone(),
                created_at: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                query: request.query.clone(),
                answer: "answer_from_grpc".to_string(),
                embedding: vec![1.0],
            };
            match writer.persist(&r) {
                Ok(w) => return Ok(w),
                Err(e) => return Err(e),
            };
        } else {
            return Err(
                format!("error in grpc call with status = {:#?}", grpc_result_status).into(),
            );
        }
    }
}

impl LLMSmol {
    pub fn new() -> Self {
        return Self {};
    }
}
