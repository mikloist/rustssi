use std::{
    str::{self, FromStr},
    string::ToString,
};
use strum_macros::EnumString;
use strum_macros::ToString;

#[derive(Debug, PartialEq)]
enum ErrorType {
    EmptyString,
    InvalidString,
    ComandNotFound,
}

#[derive(EnumString, ToString, Debug, PartialEq)]
enum Command {
    PASS,
    NICK,
    TOPIC,
}

#[derive(Debug, PartialEq)]
struct GenericMessage {
    prefix: Option<String>,
    msg_type: Command,
    args: Vec<String>,
}

const SPACE: char = ' ';
const COLON: char = ':';
const ENDLN: char = '\n';
const CARRLN: char = '\r';

impl GenericMessage {
    fn to_message(&self) -> String {
        let mut msg = String::from("");
        if let Some(prefix) = &self.prefix {
            msg.push(COLON);
            msg.push_str(prefix);
            msg.push(SPACE);
        }

        msg.push_str(self.msg_type.to_string().as_str());

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
}

fn parse_prefix(input: &mut &str) -> Option<String> {
    if input.starts_with(COLON) {
        if let Some(sep) = input.find(|x| x == SPACE) {
            let val = Some(String::from(&input[1..sep]));
            *input = &input[sep + SPACE.len_utf8()..];
            return val;
        }
    }
    None
}

fn parse_command(input: &mut &str) -> Result<Command, ErrorType> {
    let (cmd_end, end) = match input.find(|x| x == SPACE) {
        Some(sep) => (sep, sep + SPACE.len_utf8()),
        None => (input.len(), input.len()),
    };

    let command = Command::from_str(&input[0..cmd_end]).map_err(|_| ErrorType::ComandNotFound)?;
    *input = &input[end..];
    return Ok(command);
}
fn parse_args(input: &mut &str) -> Vec<String> {
    let mut v = Vec::<String>::new();
    match input.find(|x| x == COLON) {
        Some(sep) => {
            let itr = input[0..sep]
                .split_ascii_whitespace()
                .map(|x| String::from(x));

            v.extend(itr);
            v.push(String::from(&input[sep + COLON.len_utf8()..]));
        }
        None => {
            let itr = input.split_ascii_whitespace().map(|x| String::from(x));
            v.extend(itr);
        }
    }

    *input = &input[0..0];
    return v;
}

fn from_bytes(input: &[u8]) -> Result<GenericMessage, ErrorType> {
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
        assert_eq!(parse_prefix(&mut v), Some(String::from("hello")));
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
        assert_eq!(parse_prefix(&mut v), Some(String::from("very")));
        assert_eq!(v, "long stuff but should be parsed ");
    }

    #[test]
    fn prefix_tests_long_string_w_garbage() {
        let mut v = ":very_long@stuff\"but...should
        be parsed ";

        assert_eq!(
            parse_prefix(&mut v),
            Some(String::from(
                "very_long@stuff\"but...should
"
            ))
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
            prefix: Some(String::from("WiZ!jto@tolsun.oulu.fi")),
            msg_type: Command::TOPIC,
            args: vec!["#test", "New topic "]
                .iter()
                .map(|&x| String::from(x))
                .collect(),
        };

        assert_eq!(var.to_message().as_bytes(), v);
        assert_eq!(from_bytes(v), Ok(var));
    }

    #[test]
    fn test_msg_2() {
        let v = "TOPIC #test :New topic ".as_bytes();
        let var = GenericMessage {
            prefix: None,
            msg_type: Command::TOPIC,
            args: vec!["#test", "New topic "]
                .iter()
                .map(|&x| String::from(x))
                .collect(),
        };

        assert_eq!(var.to_message().as_bytes(), v);
        assert_eq!(from_bytes(v), Ok(var));
    }
    #[test]
    fn test_msg_3() {
        let v = "NICK".as_bytes();
        let var = GenericMessage {
            prefix: None,
            msg_type: Command::NICK,
            args: vec![],
        };

        assert_eq!(var.to_message().as_bytes(), v);
        assert_eq!(from_bytes(v), Ok(var));
    }
}
