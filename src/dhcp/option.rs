use super::messagetype::MessageType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Option {
    MagicCookie([u8; 4]),
    MessageType(MessageType),
    SubnetMask([u8; 4]),
    ClientIdentifier(u8, Vec<u8>),
    Unknown(u8, u8),
}
