use std::fmt::format;

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
        previous: Option<String>,
    },
    Error {
        msg: String
    },
}

impl Response {
    pub fn serialize(&self) -> String {
        match self {
            Response::Echo { msg } =>
                format!("REPLY {}", msg),
            Response::Value { key, value } =>
                format!("{} = {}", key, value),
            Response::Set { key: _key, value, previous } =>
                format!("{} replaces {:?}", value, previous),
            Response::Error { msg } =>
                format!("error: {}", msg)
        }
    }
}