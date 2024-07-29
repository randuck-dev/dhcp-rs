use anyhow::{anyhow, Result};

use super::lib::{Flags, OpType};
use super::messagetype::MessageType;
use super::option::Option;

#[derive(Debug)]
pub(crate) struct Packet {
    pub op: OpType,
    pub htype: u8,
    pub hlen: u8,
    pub hops: u8,
    pub xid: u32,
    pub secs: u16,
    pub flags: Flags,
    pub ciaddr: [u8; 4],
    pub yiaddr: [u8; 4],
    pub siaddr: [u8; 4],
    pub giaddr: [u8; 4],
    pub chaddr: [u8; 16],
    pub sname: [u8; 64],
    pub file: [u8; 128],
    pub options: Vec<Option>,
}

impl Packet {
    pub(crate) fn get_message_type(&self) -> Result<MessageType> {
        self.options
            .iter()
            .find_map(|option| match option {
                Option::MessageType(message_type) => Some(*message_type),
                _ => None,
            })
            .ok_or_else(|| anyhow!("Message type not found"))
    }
}
