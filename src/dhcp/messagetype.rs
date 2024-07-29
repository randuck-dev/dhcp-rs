use super::errors::DhcpError;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum MessageType {
    DHCPDISCOVER,
    DHCPOFFER,
    DHCPREQUEST,
    DHCPACK,
    DHCPNAK,
    DHCPRELEASE,
    DHCPDECLINE,
    DHCPINFORM,
}

impl TryInto<MessageType> for u8 {
    type Error = DhcpError;

    fn try_into(self) -> Result<MessageType, DhcpError> {
        match self {
            1 => Ok(MessageType::DHCPDISCOVER),
            2 => Ok(MessageType::DHCPOFFER),
            3 => Ok(MessageType::DHCPREQUEST),
            4 => Ok(MessageType::DHCPACK),
            5 => Ok(MessageType::DHCPNAK),
            6 => Ok(MessageType::DHCPRELEASE),
            7 => Ok(MessageType::DHCPDECLINE),
            8 => Ok(MessageType::DHCPINFORM),
            _ => Err(DhcpError::InvalidMessageType),
        }
    }
}

impl TryFrom<MessageType> for u8 {
    type Error = DhcpError;

    fn try_from(value: MessageType) -> Result<u8, DhcpError> {
        match value {
            MessageType::DHCPDISCOVER => Ok(1),
            MessageType::DHCPOFFER => Ok(2),
            MessageType::DHCPREQUEST => Ok(3),
            MessageType::DHCPACK => Ok(4),
            MessageType::DHCPNAK => Ok(5),
            MessageType::DHCPRELEASE => Ok(6),
            MessageType::DHCPDECLINE => Ok(7),
            MessageType::DHCPINFORM => Ok(8),
        }
    }
}
