use crate::types::messages::GenericMessage;

pub struct MsgHandler {}

impl MsgHandler {
    pub fn handle_msg(&mut self, _msg: &GenericMessage) {}
    pub fn handle_error(&mut self, _msg: String) {}
}
