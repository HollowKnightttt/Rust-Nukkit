//! Lapisan RakNet minimal (Milestone 1): Unconnected Ping / Pong.
//! Cukup agar server muncul di daftar server Minecraft Bedrock (tab Servers/Friends via LAN-style ping).

/// "Offline message data ID" yang dipakai semua paket RakNet offline.
pub const MAGIC: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe, 0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78,
];

pub const ID_UNCONNECTED_PING: u8 = 0x01;
pub const ID_UNCONNECTED_PING_OPEN_CONNECTIONS: u8 = 0x02;
pub const ID_UNCONNECTED_PONG: u8 = 0x1c;

/// Layout ping: [id u8][time i64 BE][magic 16][client guid i64 BE]
/// Mengembalikan 8 byte timestamp klien (untuk dipantulkan di pong) bila paket valid.
pub fn parse_unconnected_ping(buf: &[u8]) -> Option<[u8; 8]> {
    if buf.len() < 25 {
        return None;
    }
    if buf[0] != ID_UNCONNECTED_PING && buf[0] != ID_UNCONNECTED_PING_OPEN_CONNECTIONS {
        return None;
    }
    if buf[9..25] != MAGIC[..] {
        return None;
    }
    let mut time = [0u8; 8];
    time.copy_from_slice(&buf[1..9]);
    Some(time)
}

/// Layout pong: [0x1c][time 8][server guid i64 BE][magic 16][len u16 BE][status string]
pub fn build_unconnected_pong(time: &[u8; 8], server_guid: i64, status: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(35 + status.len());
    out.push(ID_UNCONNECTED_PONG);
    out.extend_from_slice(time);
    out.extend_from_slice(&server_guid.to_be_bytes());
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&(status.len() as u16).to_be_bytes());
    out.extend_from_slice(status.as_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_roundtrip() {
        let mut ping = vec![ID_UNCONNECTED_PING];
        ping.extend_from_slice(&42i64.to_be_bytes());
        ping.extend_from_slice(&MAGIC);
        ping.extend_from_slice(&7i64.to_be_bytes());

        let time = parse_unconnected_ping(&ping).expect("ping valid");
        assert_eq!(i64::from_be_bytes(time), 42);

        let pong = build_unconnected_pong(&time, 99, "MCPE;test");
        assert_eq!(pong[0], ID_UNCONNECTED_PONG);
        assert_eq!(&pong[1..9], &time);
        assert_eq!(&pong[17..33], &MAGIC);
        assert_eq!(u16::from_be_bytes([pong[33], pong[34]]) as usize, "MCPE;test".len());
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse_unconnected_ping(&[0x01, 0x02]).is_none());
        assert!(parse_unconnected_ping(&[0u8; 40]).is_none());
    }
}
