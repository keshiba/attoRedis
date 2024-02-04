pub enum Response {
    Echo {
        msg: String
    },
    Value {
        value: String,
    },
    Set {
        key: String,
        value: String,
    },
    Error {
        msg: String
    },
    Keys {
        keys: Vec<String>
    }
}

impl Response {
    pub fn serialize(&self) -> String {
        match self {
            Response::Echo { msg } =>
                format!("REPLY {}", msg),
            Response::Keys { keys } =>
                keys.join("\r\n").to_string(),
            Response::Value { value } =>
                value.to_string(),
            Response::Set{ .. } =>
                "OK".into(),
            Response::Error { msg } =>
                format!("ERR: {}", msg),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::response::Response;

    #[test]
    fn serializes_echo_response() {
        let echo_response = Response::Echo{ msg: "yolo".into() };
        let expected = "REPLY yolo";

        let serialized = echo_response.serialize();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn serializes_keys_response() {
        let keys_response = Response::Keys{
            keys: vec!("key1".into(), "key2".into()),
        };
        let expected = "key1\r\nkey2";

        let serialized = keys_response.serialize();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn serializes_value_response() {
        let value_response = Response::Value{
            value: "value".into(),
        };
        let expected = "value";

        let serialized = value_response.serialize();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn serializes_set_response() {
        let value_response = Response::Set{
            key: "key".into(),
            value: "value".into(),
        };
        let expected = "OK";

        let serialized = value_response.serialize();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn serializes_error_response() {
        let value_response = Response::Error{
            msg: "error message".into(),
        };
        let expected = "ERR: error message";

        let serialized = value_response.serialize();

        assert_eq!(serialized, expected);
    }
}