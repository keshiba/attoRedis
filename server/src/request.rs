#[derive(Debug, PartialEq, Eq)]
pub enum Request {
    Echo { msg: String },
    Get { key: String },
    Set { key: String, value: String },
    Keys
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
            Some("KEYS") =>
                Ok(Request::Keys),
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
            _ => Err("unknown command".into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_echo_request() {
        let command = "ECHO 16AN1ms";
        let expected = Ok(Request::Echo { msg: "16AN1ms".into() });

        let echo_request = Request::parse(command);

        assert_eq!(echo_request, expected);
    }

    #[test]
    fn generates_error_if_message_is_empty_in_echo_request() {
        let command = "ECHO";
        let expected: Result<Request, String> = Err("no message".into());

        let echo_request = Request::parse(command);

        assert!(echo_request.is_err());
        assert_eq!(echo_request, expected);
    }

    #[test]
    fn ignores_extra_operands_while_parsing_echo_request() {
        let command = "ECHO message extraOperand";
        let expected = Ok(Request::Echo { msg: "message".into() });

        let echo_request = Request::parse(command);

        assert_eq!(echo_request, expected);
    }

    #[test]
    fn parses_keys_request() {
        let command = "KEYS";
        let expected = Ok(Request::Keys);

        let keys_request = Request::parse(command);

        assert_eq!(keys_request, expected);
    }

    #[test]
    fn ignores_extra_operands_while_parsing_keys_request() {
        let command = "KEYS op1 op2";
        let expected = Ok(Request::Keys);

        let keys_request = Request::parse(command);

        assert_eq!(keys_request, expected);
    }

    #[test]
    fn parses_get_request() {
        let command = "GET key";
        let expected = Ok(Request::Get { key: "key".into() });

        let get_request = Request::parse(command);

        assert_eq!(get_request, expected);
    }

    #[test]
    fn generates_error_if_key_is_empty_in_get_request() {
        let command = "GET";
        let expected: Result<Request, String> = Err("no key".into());

        let echo_request = Request::parse(command);

        assert!(echo_request.is_err());
        assert_eq!(echo_request, expected);
    }

    #[test]
    fn ignores_extra_operands_while_parsing_get_request() {
        let command = "GET key1 key2";
        let expected = Ok(Request::Get {key: "key1".into() });

        let keys_request = Request::parse(command);

        assert_eq!(keys_request, expected);
    }

    #[test]
    fn parses_set_request() {
        let command = "SET key value";
        let expected = Ok(Request::Set {
            key: "key".into(),
            value: "value".into()
        });

        let get_request = Request::parse(command);

        assert_eq!(get_request, expected);
    }

    #[test]
    fn generates_error_if_key_is_empty_in_set_request() {
        let command = "SET";
        let expected: Result<Request, String> = Err("no key".into());

        let echo_request = Request::parse(command);

        assert!(echo_request.is_err());
        assert_eq!(echo_request, expected);
    }

    #[test]
    fn generates_error_if_value_is_empty_in_set_request() {
        let command = "SET key";
        let expected: Result<Request, String> = Err("no value".into());

        let echo_request = Request::parse(command);

        assert!(echo_request.is_err());
        assert_eq!(echo_request, expected);
    }

    #[test]
    fn generates_error_if_command_not_found() {
        let command = "MEH key";
        let expected: Result<Request, String> = Err("unknown command".into());

        let echo_request = Request::parse(command);

        assert!(echo_request.is_err());
        assert_eq!(echo_request, expected);
    }

    #[test]
    fn generates_error_if_command_is_empty() {
        let command = "";
        let expected: Result<Request, String> = Err("unknown command".into());

        let echo_request = Request::parse(command);

        assert!(echo_request.is_err());
        assert_eq!(echo_request, expected);
    }
}