use std::{str::FromStr, string::ToString};
use std::str;
use strum_macros::EnumString;
use strum_macros::ToString;

#[derive(EnumString, ToString, Debug, PartialEq)]
enum Command {
    PASS,
    NICK,
}

#[derive(Debug, PartialEq)]
struct GenericMessage
{
    prefix: Option<String>,
    msg_type: Command,
    args: Vec<String>
}

const SPACE: u8 = b' ';
const COLON: u8 = b':';
const ENDLN: u8 = b'\n';
const CARRLN: u8 = b'\r';


impl GenericMessage
{
    fn to_message(&self) -> String {
        let mut msg = String::from("");
        if let Some(prefix) = &self.prefix {
            msg.push(COLON as char);
            msg.push_str(prefix);
            msg.push(SPACE as char);
        }

        msg.push_str(self.msg_type.to_string().as_str());

        let args_len = self.args.len();

        for item in self.args.iter().enumerate() {
            msg.push(SPACE as char);
            
            if item.0 == args_len-1 {
                msg.push(COLON as char);
            }
            msg.push_str(item.1);
        }

        msg.push(ENDLN as char);
        return msg;
    }
}

fn from_bytes(input: &[u8]) -> Option<GenericMessage> {
    let cleanup = input.iter().rposition(|&x| !(x == ENDLN || x == CARRLN)).unwrap_or(input.len()-1)+1;
    let mut bytes = &input[0..cleanup];
    let mut prefix = None;

    if *bytes.get(0)? == COLON {
        let sep = bytes.iter().position(|&x| x == SPACE)?;
        prefix = Some(String::from(str::from_utf8(&bytes[1..sep]).ok()?));
        bytes = &bytes[sep+1..];
    }

    let command_sep = bytes.iter().position(|&x| x == SPACE).unwrap_or(bytes.len());
    let command = Command::from_str(str::from_utf8(&bytes[0..command_sep]).ok()?).ok()?;
    bytes = &bytes[command_sep..];

    if bytes.len() == 0 {
        return Some(GenericMessage{prefix: prefix, msg_type: command, args: Vec::new()});
    }

    let mut args_sep = bytes.iter().position(|&x| x == COLON).unwrap_or(bytes.len());
    let args = String::from(str::from_utf8(&bytes[0..args_sep]).ok()?);
    let mut args_vec: Vec<String> = Vec::new();
    args_vec.extend(args.split(SPACE as char).map(|x| String::from(x)));
    args_sep += 1;
    if args_sep < bytes.len() {
        args_vec.push(String::from(str::from_utf8(&bytes[args_sep..]).ok()?));
    }
    
    return Some(GenericMessage{prefix: prefix, msg_type: command, args: args_vec});
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_object() -> GenericMessage {
        return GenericMessage{prefix: Some(String::from("a")), msg_type: Command::PASS, args: Vec::new()};
    }

    #[test]
    fn bytes_parser() {
        assert_eq!(Some(test_object()),  from_bytes(b":a PASS\n"));
        assert_eq!(Some(test_object()),  from_bytes(b":a PASS\r\n"));
        assert_eq!(Some(test_object()),  from_bytes(b":a PASS"));
        assert_eq!(Some(test_object()),  from_bytes(b":a PASS    "));
    }
}