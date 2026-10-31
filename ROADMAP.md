# Roadmap Rust-Nukkit

Nukkit adalah proyek besar yang dikerjakan banyak kontributor selama bertahun-tahun.
Daftar ini memecahnya jadi milestone yang bisa diuji satu per satu.

| # | Milestone | Hasil yang bisa dicek | Status |
|---|-----------|-----------------------|--------|
| 1 | RakNet Unconnected Ping/Pong | Server muncul di daftar server dengan MOTD | Kode ada, belum dikompilasi |
| 2 | Koneksi RakNet penuh | Open Connection 1/2, frame set, reliability, ACK/NACK, split packet | - |
| 3 | Lapisan jaringan Bedrock | Batch 0xFE, kompresi, NetworkSettings, Login + JWT, enkripsi (mode offline dulu, Xbox auth menyusul) | - |
| 4 | Urutan join | ResourcePacks, StartGame (butuh block palette + item registry sesuai versi), chunk radius, LevelChunk, PlayerSpawn | - |
| 5 | Dasar gameplay | Gerak, chat, daftar pemain, command, break/place block | - |
| 6 | Dunia | Format LevelDB Bedrock, generator Flat lalu Normal, Nether/End | - |
| 7 | Entity & item | Inventory, crafting, enchant, mob dasar, damage | - |
| 8 | Plugin & admin | Event bus, plugin API (WASM atau dylib; plugin Java Nukkit tidak bisa jalan langsung), permission, op/whitelist/ban, konsol, config | - |
| 9 | Fitur lanjutan | Form UI, scoreboard, resource pack, query/RCON, multi-versi | - |

Catatan: untuk M2-M4 pertimbangkan memakai crate yang sudah ada (mis. bedrock-rs) daripada menulis ulang semuanya.
