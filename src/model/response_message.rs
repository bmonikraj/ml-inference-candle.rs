use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseMessage {
    id: String,
    created_at: String,
    query: String,
    answer: String,
    embedding: Vec<f64>
}
