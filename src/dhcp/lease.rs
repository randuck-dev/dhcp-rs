type IPAddress = [u8; 4];
type MACAddress = [u8; 6];

struct Lease {
    ip: IPAddress,
    mac: MACAddress,
    expires: u64,

    host: Option<String>,
}
