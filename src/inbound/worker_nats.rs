use std::collections::HashMap;

use futures::StreamExt;

use crate::{model::request_message::RequestMessage, service::llm::Llm};

#[tokio::main]
pub async fn main_worker(
    config: &HashMap<String, HashMap<String, String>>,
    llm: &mut Box<dyn Llm>,
) -> Result<(), async_nats::Error> {
    let nats_url = config["consumer"]["url"].to_string();

    let client = match async_nats::connect(nats_url).await {
        Ok(c) => c,
        Err(e) => panic!("nats connection error: {}", e),
    };
    log::info!("nats consumer client created");

    let mut subscriber = match client
        .subscribe(config["consumer"]["subject"].to_string())
        .await
    {
        Ok(s) => s,
        Err(e) => panic!("nats subscription error: {}", e),
    };
    log::info!("nats consumer subscriber created");

    while let Some(message_) = subscriber.next().await {
        match serde_json::from_slice::<RequestMessage>(message_.payload.as_ref()) {
            Ok(m) => {
                // do something with payload
                log::info!("nats message received = {:#?}", message_);
                log::info!("payload data : RequestMessage.query = {}", m.query);
                llm.generate_and_persist();
            }
            Err(e) => {
                log::info!("nats message received = {:#?}", message_);
                log::error!(
                    "error in nats message json deserialization: {}",
                    e.to_string()
                );
            }
        };
    }

    Ok(())
}

#[cfg(test)]
mod test_worker_nats {
    use crate::service::llm_smol::LLMSmol;

    use super::*;

    #[test]
    #[should_panic]
    fn test_no_url() {
        let config: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut llm: Box<dyn Llm> = Box::new(LLMSmol::new());
        let _ = main_worker(&config, &mut llm);
    }

    #[test]
    #[should_panic]
    fn test_no_subject() {
        let mut config: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut consumer_config = HashMap::new();
        consumer_config.insert("url".to_owned(), "nats:://demo.nats.io:4222".to_owned());
        config.insert("consumer".to_owned(), consumer_config);
        let mut llm: Box<dyn Llm> = Box::new(LLMSmol::new());
        let _ = main_worker(&config, &mut llm);
    }
}