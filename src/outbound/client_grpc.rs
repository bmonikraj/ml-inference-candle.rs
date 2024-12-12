use tonic::transport::{Channel, ClientTlsConfig, Endpoint, Uri};

use crate::model::grpc::inference::grpc_inference_service_client::GrpcInferenceServiceClient;

use std::{error::Error, str::FromStr};

pub struct ClientGrpc {
    client: Option<GrpcInferenceServiceClient<Channel>>,
}

impl ClientGrpc {
    pub fn new() -> Self {
        return Self { client: None };
    }

    pub fn init(&mut self, grpc_endpoint_uri: &String) -> Result<bool, Box<dyn Error>> {
        // let certs = std::fs::read_to_string("/etc/ssl/cert.pem")?;
        // let tls_config = ClientTlsConfig::new().ca_certificate(Certificate::from_pem(certs));

        let tls_config = ClientTlsConfig::new().with_native_roots();

        let mut grpc_endpoint = Endpoint::from(grpc_endpoint_uri.parse::<Uri>().unwrap());
        grpc_endpoint = grpc_endpoint.tls_config(tls_config).unwrap();
        let grpc_client = futures::executor::block_on(async {
            GrpcInferenceServiceClient::connect(grpc_endpoint).await
        });

        match grpc_client {
            Ok(g) => {
                self.client = Some(g);
                return Ok(true);
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    pub fn get(&mut self) -> GrpcInferenceServiceClient<Channel> {
        return self.client.clone().unwrap();
    }
}
