use anyhow::Result;

use super::{errors::DhcpError, messagetype::MessageType, option::Option, packet::Packet};

const MAGIC_COOKIE: Option = Option::MagicCookie([99, 130, 83, 99]);

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
    let options = parse_options(buf)?;

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

fn parse_options(buf: &[u8]) -> Result<Vec<Option>, DhcpError> {
    let mut options = Vec::new();

    let magic_cookie = &buf[236..240];
    if magic_cookie != [99, 130, 83, 99] {
        return Err(DhcpError::InvalidMagicCookie);
    }

    options.push(Option::MagicCookie([99, 130, 83, 99]));

    let mut i = 240;
    while i < buf.len() {
        let code = buf[i];

        if code == 0 {
            break;
        }

        let (inc, option) = match code {
            1 => (6, Option::SubnetMask(buf[i + 2..i + 6].try_into().unwrap())),
            53 => (3, Option::MessageType(buf[i + 2].try_into()?)),
            61 => {
                let len = buf[i + 1] as usize;
                let t = buf[i + 2];
                let start = i + 2;
                let end = start + len;

                // increment for: CODE + LENGTH + TYPE + LEN(IDENTIFIER)
                let inc = 2 + len;
                (inc, Option::ClientIdentifier(t, buf[start..end].to_vec()))
            }
            // This is the end of the options
            255 => break,

            _ => panic!("Unknown option code: {}", code),
        };
        options.push(option);

        i += inc;
    }

    Ok(options)
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

    pub(crate) fn default() -> Self {
        let mut p = RawPacket::new();
        p.set_op(OpType::BOOTREQUEST);
        p.set_htype(1);
        p.set_hlen(6);
        p.set_hops(0);
        p.set_xid(10);
        p.set_secs(10);
        p.set_broadcast(true);
        p.set_options(vec![
            MAGIC_COOKIE,
            Option::MessageType(MessageType::DHCPDISCOVER),
            Option::SubnetMask([255, 255, 255, 0]),
        ]);
        p
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

    pub(crate) fn set_options(&mut self, options: Vec<Option>) {
        let mut i = 236;
        if options.is_empty() {
            return;
        }

        for option in options {
            match option {
                Option::MagicCookie(_) => {
                    self.buf[i..i + 4].copy_from_slice(&[99, 130, 83, 99]);
                    i += 4;
                }
                Option::SubnetMask(mask) => {
                    self.buf[i] = 1;
                    self.buf[i + 1] = 4;
                    self.buf[i + 2..i + 6].copy_from_slice(&mask);
                    i += 6;
                }
                Option::MessageType(value) => {
                    self.buf[i] = 53;
                    self.buf[i + 1] = value.try_into().unwrap();
                    i += 2;
                }
                Option::ClientIdentifier(t, identifier) => {
                    self.buf[i] = 61;
                    self.buf[i + 1] = t;
                    self.buf[i + 2..i + 2 + identifier.len()].copy_from_slice(&identifier);
                    i += 2 + identifier.len();
                }
                Option::Unknown(code, value) => {
                    self.buf[i] = code;
                    self.buf[i + 1] = value;
                    i += 2;
                }
            }
        }

        self.buf[i] = 0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_packet() {
        let raw_packet = RawPacket::default();

        let packet = match raw_packet.into_packet() {
            Ok(packet) => packet,
            Err(e) => panic!("Failed to parse packet: {:?}", e),
        };

        assert_eq!(OpType::BOOTREQUEST, packet.op);
        assert_eq!(1, packet.htype);
        assert_eq!(6, packet.hlen);
        assert_eq!(0, packet.hops);
        assert_eq!(10, packet.xid);
        assert_eq!(10, packet.secs);
        assert_eq!(true, packet.flags.broadcast);

        let subnet_mask = packet
            .options
            .iter()
            .find_map(|option| match option {
                Option::SubnetMask(mask) => Some(mask),
                _ => None,
            })
            .unwrap();

        assert_eq!([255, 255, 255, 0], *subnet_mask);
    }

    #[test]
    fn should_fail_on_non_present_magic_cookie() {
        let mut raw_packet = RawPacket::default();
        raw_packet.set_options(vec![Option::MessageType(MessageType::DHCPDISCOVER)]);

        let result = raw_packet.into_packet();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DhcpError::InvalidMagicCookie);
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
