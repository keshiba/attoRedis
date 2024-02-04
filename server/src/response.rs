pub enum Response {
    Echo {
        msg: String
    },
    Value {
        key: String,
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
                format!("{}", keys.join("\r\n")),
            Response::Value { key: _key, value } =>
                format!("{}", value),
            Response::Set{ .. } =>
                "OK".into(),
            Response::Error { msg } =>
                format!("ERR: {}", msg),
        }
    }
}