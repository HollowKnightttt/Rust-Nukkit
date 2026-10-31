mod raknet;

use std::net::UdpSocket;
use std::time::{SystemTime, UNIX_EPOCH};

// --- Konfigurasi sementara (nanti dipindah ke server.toml) ---
const BIND_ADDR: &str = "0.0.0.0:19132";
const PORT_V4: u16 = 19132;
const PORT_V6: u16 = 19133;
const MOTD: &str = "Rust-Nukkit"; // jangan pakai tanda ';'
const WORLD_NAME: &str = "world";
const GAMEMODE: &str = "Survival";
const MAX_PLAYERS: u32 = 20;
// Harus cocok dengan versi klien yang dituju. Nomor protokol berubah tiap rilis Minecraft;
// cek https://github.com/Mojang/bedrock-protocol-docs/releases
const PROTOCOL: u32 = 2225; // 1.26.61
const GAME_VERSION: &str = "1.26.61";

fn server_guid() -> i64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(1);
    let pid = std::process::id() as u64;
    (nanos ^ (pid << 32)) as i64
}

fn status_string(guid: i64, online: u32) -> String {
    format!(
        "MCPE;{};{};{};{};{};{};{};{};1;{};{};",
        MOTD, PROTOCOL, GAME_VERSION, online, MAX_PLAYERS, guid, WORLD_NAME, GAMEMODE, PORT_V4, PORT_V6
    )
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind(BIND_ADDR)?;
    let guid = server_guid();
    println!("Rust-Nukkit listening on udp://{BIND_ADDR} (guid {guid})");

    let mut buf = [0u8; 2048];
    loop {
        let (len, peer) = match socket.recv_from(&mut buf) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("recv error: {e}");
                continue;
            }
        };
        let packet = &buf[..len];

        if let Some(time) = raknet::parse_unconnected_ping(packet) {
            let pong = raknet::build_unconnected_pong(&time, guid, &status_string(guid, 0));
            let _ = socket.send_to(&pong, peer);
            continue;
        }

        // TODO Milestone 2: Open Connection Request 1/2 dan frame set RakNet.
    }
}
