use std::{
    str::{self, FromStr},
    string::ToString,
};

use super::enums::Command;
use super::enums::ErrorType;

#[derive(Debug, PartialEq)]
pub struct GenericMessage<'a> {
    prefix: Option<&'a str>,
    msg_type: Command<'a>,
    args: Vec<&'a str>,
}

const SPACE: char = ' ';
const COLON: char = ':';
const ENDLN: char = '\n';
const CARRLN: char = '\r';

impl<'a> GenericMessage<'a> {
    pub fn to_message(&self) -> String {
        let mut msg = String::from("");
        if let Some(prefix) = &self.prefix {
            msg.push(COLON);
            msg.push_str(prefix);
            msg.push(SPACE);
        }

        match &self.msg_type {
            Command::INTERROR(v) => msg.push_str(v),
            msgtype => msg.push_str(msgtype.to_string().as_str()),
        }

        let args_len = self.args.len();

        for item in self.args.iter().enumerate() {
            msg.push(SPACE);

            if item.0 == args_len - 1 {
                msg.push(COLON);
            }
            msg.push_str(item.1);
        }

        return msg;
    }

    pub fn from_bytes(input: &'a [u8]) -> Result<Self, ErrorType> {
        let mut utf8_str = str::from_utf8(input).map_err(|_| ErrorType::InvalidString)?;

        utf8_str = utf8_str.trim_end_matches(|x| x == ENDLN || x == CARRLN);
        utf8_str = utf8_str.trim_start();

        if utf8_str.len() == 0 {
            return Err(ErrorType::EmptyString);
        }

        let prefix = parse_prefix(&mut utf8_str);
        let msg_type = parse_command(&mut utf8_str)?;
        let args = parse_args(&mut utf8_str);

        Ok(GenericMessage {
            prefix,
            msg_type,
            args,
        })
    }
}

fn parse_prefix<'a>(input: &mut &'a str) -> Option<&'a str> {
    if input.starts_with(COLON) {
        if let Some(sep) = input.find(|x| x == SPACE) {
            let val = Some(&input[1..sep]);
            *input = &input[sep + SPACE.len_utf8()..];
            return val;
        }
    }
    None
}

fn parse_command<'a>(input: &mut &'a str) -> Result<Command<'a>, ErrorType> {
    let (cmd_end, end) = match input.find(|x| x == SPACE) {
        Some(sep) => (sep, sep + SPACE.len_utf8()),
        None => (input.len(), input.len()),
    };

    let cmd = &input[0..cmd_end];
    if cmd.chars().all(|x| x >= '0' && x <= '9') {
        *input = &input[end..];
        return Ok(Command::INTERROR(cmd));
    }

    let command = Command::from_str(cmd).map_err(|_| ErrorType::ComandNotFound)?;
    *input = &input[end..];
    return Ok(command);
}

fn parse_args<'a>(input: &mut &'a str) -> Vec<&'a str> {
    let mut v = Vec::<&str>::new();
    match input.find(|x| x == COLON) {
        Some(sep) => {
            let itr = input[0..sep].split_ascii_whitespace();

            v.extend(itr);
            v.push(&input[sep + COLON.len_utf8()..]);
        }
        None => {
            let itr = input.split_ascii_whitespace();
            v.extend(itr);
        }
    }

    *input = &input[0..0];
    return v;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn string_split() {
        let hello = "hello_world";
        assert_eq!("hello", &hello[0..5]);
        assert_eq!(5, hello[0..5].len());
    }

    #[test]
    fn command_tests_leaves_empty() {
        let mut v = "NICK";
        assert_eq!(parse_command(&mut v), Ok(Command::NICK));
        assert_eq!(v, "");
    }

    #[test]
    fn command_tests_leaves_residual() {
        let mut v = "NICK somethingelse after a space";
        assert_eq!(parse_command(&mut v), Ok(Command::NICK));
        assert_eq!(v, "somethingelse after a space");
    }

    #[test]
    fn command_tests_not_recognized() {
        let mut v = "NICK2 somethingelse after a space";
        assert_eq!(parse_command(&mut v), Err(ErrorType::ComandNotFound));
        assert_eq!(v, "NICK2 somethingelse after a space");
    }

    #[test]
    fn prefix_tests_all_good() {
        let mut v = ":hello ";
        assert_eq!(parse_prefix(&mut v), Some("hello"));
        assert_eq!(v, "");
    }

    #[test]
    fn prefix_tests_no_space() {
        let mut v = ":hello";
        assert_eq!(parse_prefix(&mut v), None);
        assert_eq!(v, ":hello");
    }

    #[test]
    fn prefix_tests_no_colon() {
        let mut v = "hello ";
        assert_eq!(parse_prefix(&mut v), None);
        assert_eq!(v, "hello ");
    }

    #[test]
    fn prefix_tests_long_string() {
        let mut v = ":very long stuff but should be parsed ";
        assert_eq!(parse_prefix(&mut v), Some("very"));
        assert_eq!(v, "long stuff but should be parsed ");
    }

    #[test]
    fn prefix_tests_long_string_w_garbage() {
        let mut v = ":very_long@stuff\"but...should
        be parsed ";

        assert_eq!(
            parse_prefix(&mut v),
            Some(
                "very_long@stuff\"but...should
"
            )
        );
        assert_eq!(v, "       be parsed ");
    }

    #[test]
    fn args_test() {
        let mut v = ":very long stuff but should be parsed ";
        assert_eq!(
            parse_args(&mut v),
            vec!["very long stuff but should be parsed "]
        );
        assert_eq!(v, "");
    }

    #[test]
    fn args_test_multiple() {
        let mut v = "very long :stuff but :should be parsed ";
        assert_eq!(
            parse_args(&mut v),
            vec!["very", "long", "stuff but :should be parsed "]
        );
        assert_eq!(v, "");
    }
    #[test]
    fn args_test_last_colon() {
        let mut v = "very long stuff but should be parsed :";
        assert_eq!(
            parse_args(&mut v),
            vec!["very", "long", "stuff", "but", "should", "be", "parsed", ""]
        );
        assert_eq!(v, "");
    }
    #[test]
    fn args_empty() {
        let mut v = "";
        assert_eq!(parse_args(&mut v), Vec::<String>::new());
        assert_eq!(v, "");
    }
    #[test]
    fn args_empty_colon() {
        let mut v = ":";
        assert_eq!(parse_args(&mut v), vec![""]);
        assert_eq!(v, "");
    }
    #[test]
    fn args_something_colon() {
        let mut v = ":only you";
        assert_eq!(parse_args(&mut v), vec!["only you"]);
        assert_eq!(v, "");
    }

    #[test]
    fn test_msgs() {
        let v = ":WiZ!jto@tolsun.oulu.fi TOPIC #test :New topic ".as_bytes();
        let var = GenericMessage {
            prefix: Some("WiZ!jto@tolsun.oulu.fi"),
            msg_type: Command::TOPIC,
            args: vec!["#test", "New topic "],
        };

        assert_eq!(GenericMessage::from_bytes(v).unwrap(), var);
        assert_eq!(var.to_message().as_bytes(), v);
    }

    #[test]
    fn test_msg_2() {
        let v = "TOPIC #test :New topic ".as_bytes();
        let var = GenericMessage {
            prefix: None,
            msg_type: Command::TOPIC,
            args: vec!["#test", "New topic "],
        };

        assert_eq!(GenericMessage::from_bytes(v).unwrap(), var);
        assert_eq!(var.to_message().as_bytes(), v);
    }
    #[test]
    fn test_msg_3() {
        let v = "NICK".as_bytes();
        let var = GenericMessage {
            prefix: None,
            msg_type: Command::NICK,
            args: vec![],
        };

        assert_eq!(GenericMessage::from_bytes(v).unwrap(), var);
        assert_eq!(var.to_message().as_bytes(), v);
    }

    #[test]
    fn test_msg_4() {
        let v = "800 :oh snap".as_bytes();
        let var = GenericMessage {
            prefix: None,
            msg_type: Command::INTERROR("800"),
            args: vec!["oh snap"],
        };

        assert_eq!(GenericMessage::from_bytes(v).unwrap(), var);
        assert_eq!(var.to_message().as_bytes(), v);
    }
}
