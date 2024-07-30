use super::messagetype::MessageType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Option {
    MagicCookie([u8; 4]),
    SubnetMask([u8; 4]),           // 1
    TimeOffset(u32),               // 2
    MessageType(MessageType),      // 53
    ClientIdentifier(u8, Vec<u8>), // 61
    End(),
    Unknown(u8, u8),
}
