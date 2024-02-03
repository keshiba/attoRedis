pub enum Request {
    Echo { msg: String },
    Get { key: String },
    Set { key: String, value: String },
}

impl Request {
    pub fn parse(input: &str) -> Result<Request, String> {
        let mut parts = input.splitn(3, ' ');
        match parts.next() {
            Some("ECHO") => match parts.next() {
                None =>
                    Err("no message".into()),
                Some(msg) =>
                    Ok(Request::Echo {
                        msg: msg.into()
                    })
            },
            Some("GET") => match parts.next() {
                None =>
                    Err("no key".into()),
                Some(key) =>
                    Ok(Request::Get {
                        key: key.into()
                    })
            },
            Some("SET") => match parts.next() {
                None =>
                    Err("no key".into()),
                Some(key) => match parts.next() {
                    None =>
                        Err("no value".into()),
                    Some(value) => {
                        Ok(Request::Set { key: key.into(), value: value.into() })
                    }
                }
            }
            Some(cmd) => Err(format!("unknown command: {}", cmd)),
            None => Err("invalid command".into())
        }
    }
}