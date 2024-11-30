use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct RequestMessage {
    pub query: String,
}

#[cfg(test)]
mod test_request_message {
    use super::*;

    #[test]
    fn test_message() {
        let a = RequestMessage {
            query: "a".to_string()
        };

        let b = RequestMessage {
            query: "a".to_string()
        };

        assert_eq!(a, b);
    }
}
