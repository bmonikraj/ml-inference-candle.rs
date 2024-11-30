use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
pub struct ResponseMessage {
    id: String,
    created_at: String,
    query: String,
    answer: String,
    embedding: Vec<f64>,
}

#[cfg(test)]
mod test_response_message {
    use super::*;

    #[test]
    fn test_message() {
        let a = ResponseMessage {
            id: "a".to_string(),
            created_at: "b".to_string(),
            query: "c".to_string(),
            answer: "d".to_string(),
            embedding: vec![],
        };

        let b = ResponseMessage {
            id: "a".to_string(),
            created_at: "b".to_string(),
            query: "c".to_string(),
            answer: "d".to_string(),
            embedding: vec![],
        };

        assert_eq!(a, b);
    }
}
