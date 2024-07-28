use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DhcpError {
    PacketTooShort,
    InvalidOpCode,
}

pub(crate) fn parse_dhcp_packet(buf: &[u8]) -> Result<Packet, DhcpError> {
    if buf.len() < 236 {
        return Err(DhcpError::PacketTooShort);
    }

    let op = match buf[0] {
        1 => OpType::BOOTREQUEST,
        2 => OpType::BOOTREPLY,
        _ => return Err(DhcpError::InvalidOpCode),
    };
    let htype = buf[1];
    let hlen = buf[2];
    let hops = buf[3];
    let xid = u32::from_be_bytes(buf[4..8].try_into().expect("Invalid xid length"));
    let secs = u16::from_be_bytes(buf[8..10].try_into().expect("invalid secs length"));
    let flags = Flags {
        broadcast: buf[10] & 0b10000000 != 0,
    };

    let ciaddr = buf[12..16].try_into().expect("Invalid ciaddr length");
    let yiaddr = buf[16..20].try_into().expect("invalid yiaddr length");
    let siaddr = buf[20..24].try_into().expect("invalid siaddr length");
    let giaddr = buf[24..28].try_into().expect("invalid giaddr length");
    let chaddr = buf[28..44].try_into().expect("invalid chaddr length");
    let sname = buf[44..108].try_into().expect("invalid sname length");
    let file = buf[108..236].try_into().expect("invalid file length");
    let mut options = Vec::new();

    for i in (236..buf.len()).step_by(2) {
        let code = buf[i];
        let value = buf[i + 1];

        if code == 0 {
            break;
        }

        let option = Option { code, value };
        options.push(option);
    }

    Ok(Packet {
        op,
        htype,
        hlen,
        hops,
        xid,
        secs,
        flags,
        ciaddr,
        yiaddr,
        siaddr,
        giaddr,
        chaddr,
        sname,
        file,
        options,
    })
}

#[derive(Debug)]
pub(crate) struct Packet {
    op: OpType,
    htype: u8,
    hlen: u8,
    hops: u8,
    xid: u32,
    secs: u16,
    flags: Flags,
    ciaddr: [u8; 4],
    yiaddr: [u8; 4],
    siaddr: [u8; 4],
    giaddr: [u8; 4],
    chaddr: [u8; 16],
    sname: [u8; 64],
    file: [u8; 128],

    // RFC951 (BOOTP) states that there should be a magic number at the beginning of this field, which consists of 4 octets.
    // RFC1533 (DHCP Options and BOOTP Vendor Extensions) states that these octets are 99, 130, 83, 99
    options: Vec<Option>,
}

#[derive(Debug)]
pub(crate) struct Option {
    code: u8,
    value: u8,
}

#[derive(Debug)]
pub(crate) struct Flags {
    broadcast: bool,
    // for now we do not need to implement the reserved fields
}

pub(crate) struct RawPacket {
    pub(crate) buf: [u8; 1024],
}

impl RawPacket {
    pub(crate) fn new() -> Self {
        RawPacket { buf: [0; 1024] }
    }

    pub(crate) fn set_op(&mut self, op: OpType) {
        self.buf[0] = match op {
            OpType::BOOTREQUEST => 1,
            OpType::BOOTREPLY => 2,
        };
    }

    pub(crate) fn set_htype(&mut self, htype: u8) {
        self.buf[1] = htype;
    }

    pub(crate) fn set_hlen(&mut self, hlen: u8) {
        self.buf[2] = hlen;
    }

    pub(crate) fn set_hops(&mut self, hops: u8) {
        self.buf[3] = hops;
    }

    pub(crate) fn set_xid(&mut self, xid: u32) {
        let bytes = xid.to_be_bytes();
        self.buf[4..8].copy_from_slice(&bytes);
    }

    pub(crate) fn set_secs(&mut self, secs: u16) {
        let bytes = secs.to_be_bytes();
        self.buf[8..10].copy_from_slice(&bytes);
    }

    pub(crate) fn set_broadcast(&mut self, broadcast: bool) {
        self.buf[10] = if broadcast { 0b10000000 } else { 0 };
    }

    // Implement setters for the remaining fields

    pub(crate) fn into_packet(self) -> Result<Packet, DhcpError> {
        parse_dhcp_packet(&self.buf)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum OpType {
    BOOTREQUEST,
    BOOTREPLY,
}

#[derive(Debug, PartialEq, Eq)]
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
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MessageType> {
        match self {
            1 => Ok(MessageType::DHCPDISCOVER),
            2 => Ok(MessageType::DHCPOFFER),
            3 => Ok(MessageType::DHCPREQUEST),
            4 => Ok(MessageType::DHCPACK),
            5 => Ok(MessageType::DHCPNAK),
            6 => Ok(MessageType::DHCPRELEASE),
            7 => Ok(MessageType::DHCPDECLINE),
            8 => Ok(MessageType::DHCPINFORM),
            _ => Err(anyhow!("Invalid message type")),
        }
    }
}

impl Packet {
    pub(crate) fn get_message_type(&self) -> Result<MessageType> {
        match self.options.iter().find(|o| o.code == 53) {
            Some(option) => option.value.try_into(),
            None => Err(anyhow!("Missing message type option")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn test_parse_packet() {
        let mut raw_packet = RawPacket::new();

        raw_packet.set_op(OpType::BOOTREQUEST);
        raw_packet.set_htype(1);
        raw_packet.set_hlen(6);
        raw_packet.set_hops(0);
        raw_packet.set_xid(10);
        raw_packet.set_secs(10);
        raw_packet.set_broadcast(true);

        let result = raw_packet.into_packet();
        assert!(result.is_ok());
        let packet = result.unwrap();

        assert_eq!(OpType::BOOTREQUEST, packet.op);
        assert_eq!(1, packet.htype);
        assert_eq!(6, packet.hlen);
        assert_eq!(0, packet.hops);
        assert_eq!(10, packet.xid);
        assert_eq!(10, packet.secs);
        assert_eq!(true, packet.flags.broadcast)
    }

    #[test]
    fn test_get_msg_type_should_return_correct_message_type() {
        let packet = Packet {
            op: OpType::BOOTREQUEST,
            htype: 1,
            hlen: 6,
            hops: 0,
            xid: 10,
            secs: 10,
            flags: Flags { broadcast: true },
            ciaddr: [192u8, 168, 1, 1],
            yiaddr: [0u8, 0, 0, 0],
            siaddr: [0u8, 0, 0, 0],
            giaddr: [0u8, 0, 0, 0],
            chaddr: [0u8; 16],
            sname: [0u8; 64],
            file: [0u8; 128],
            options: vec![
                Option {
                    code: 99,
                    value: 130,
                },
                Option {
                    code: 83,
                    value: 99,
                },
                Option { code: 53, value: 1 },
            ],
        };

        let result = packet.get_message_type();
        match result {
            Ok(message_type) => assert_eq!(MessageType::DHCPDISCOVER, message_type),
            Err(e) => panic!("Unexpected error: {:?}", e),
        }
    }

    #[test]
    fn should_return_invalid_opcode_error() {
        let mut buf = [0; 236];
        buf[0] = 3;

        let result = parse_dhcp_packet(&buf);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DhcpError::InvalidOpCode);
    }

    #[test]
    fn should_fail_when_not_enough_data_is_received() {
        let buf = [0; 235];

        let result = parse_dhcp_packet(&buf);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DhcpError::PacketTooShort);
    }
}
