#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DhcpError {
    PacketTooShort,
    InvalidOpCode,
    InvalidMessageType,
    InvalidMagicCookie,
}
