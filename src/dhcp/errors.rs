use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DhcpError {
    PacketTooShort,
    InvalidOpCode,
    InvalidMessageType,
    InvalidMagicCookie,
}

impl fmt::Display for DhcpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DhcpError::PacketTooShort => write!(f, "Packet is too short"),
            DhcpError::InvalidOpCode => write!(f, "Invalid DHCP opcode"),
            DhcpError::InvalidMessageType => write!(f, "Invalid DHCP message type"),
            DhcpError::InvalidMagicCookie => write!(f, "Invalid DHCP magic cookie"),
        }
    }
}

impl std::error::Error for DhcpError {}
