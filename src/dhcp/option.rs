use super::messagetype::MessageType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Option {
    MagicCookie([u8; 4]),
    MessageType(MessageType),
    Unknown(u8, u8),
}
