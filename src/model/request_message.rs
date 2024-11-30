use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestMessage {
    pub query: String,
}
