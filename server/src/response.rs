pub enum Response {
    Hello,
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
            Response::Hello => "HELLO".into(),
            Response::Value { key, value } =>
                format!("{} = {}", key, value),
            Response::Set { key, value, previous } =>
                format!("set {}={} replaces {:?}", key, value, previous),
            Response::Error { msg } =>
                format!("error: {}", msg)
        }
    }
}