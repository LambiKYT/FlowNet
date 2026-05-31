# FlowNet

**Real-time network traffic analyzer** built with Rust and [Tauri v2](https://v2.tauri.app).

![Rust](https://img.shields.io/badge/Rust-1.81%2B-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue)

FlowNet captures live network packets, parses Ethernet / IP / TCP / UDP headers, and displays them in a high-performance virtualized UI. Designed for network diagnostics, protocol analysis, and educational use.

---

## Downloads

Pre-built binaries are available on the [Releases page](https://github.com/your-username/flownet/releases/latest):

| Platform | Format |
|----------|--------|
| Windows  | `.msi` installer |
| Linux    | `.deb` / `.AppImage` |
| macOS    | `.dmg` (Intel + Apple Silicon) |

Building from source is also supported (see [Installation](#installation)).

---

## Visuals

![FlowNet Demo](assets/demo.gif)
*Real-time capture session on a local network. IP addresses are blurred for privacy.*

---

## Features

- **Live capture** — select any network interface and stream packets in real time
- **Protocol parsing** — Ethernet, IPv4, IPv6, TCP, UDP, ICMP header analysis
- **High-performance UI** — virtualized list (`react-window`) handles thousands of packets without lag
- **Dark-theme interface** — Tailwind CSS + Lucide React icons
- **Cross-platform** — Windows, Linux, macOS
- **JSON export** — packets serialized via `serde` for further processing

---

## Installation

### Prerequisites

- Rust 1.81+ (install via [rustup](https://rustup.rs/))
- Node.js 20+ and npm
- **Npcap** (Windows) or **libpcap** (Linux/macOS)

#### Windows — Npcap + SDK

1. Download and install [Npcap](https://npcap.com/). **Check "Install in WinPcap API‑compatible Mode"**.
2. Download the [Npcap SDK](https://npcap.com/#download) (same page) and extract to `C:\Program Files\Npcap SDK`, or set `$env:NPCAP_SDK_PATH` to your custom path.

Administrative privileges are required for packet capture.

#### Linux — libpcap

```bash
sudo apt-get install libpcap-dev   # Debian/Ubuntu
sudo dnf install libpcap-devel     # Fedora
```

#### macOS — libpcap

libpcap is pre-installed. Install the pcap crate's system dependency via Homebrew:

```bash
brew install pcap
```

### Build from source

```bash
git clone https://github.com/LambiKYT/FlowNet.git
cd flownet

# Build the Rust CLI (standalone packet capture)
cargo build --package flownet-core --release

# Build the full Tauri desktop app
npm --prefix ui install
cargo build --package flownet-tauri --release
```

---

## Usage

### CLI (standalone)

```bash
sudo cargo run --package flownet-core  # Linux/macOS
# or as Administrator on Windows
```

Select an interface from the list. Captured packets are printed as JSON lines to stdout.

### Desktop app (Tauri)

```bash
cargo tauri dev
```

1. Select a network interface from the dropdown.
2. Click **Start** to begin capturing.
3. Packets appear instantly in a virtualized table. Click any packet for details.
4. Click **Stop** to end capture.

> **Privileged access**: Packet capture requires elevated permissions. On Linux, grant `CAP_NET_RAW`:
> ```bash
> sudo setcap cap_net_raw+ep target/release/flownet-cli
> ```
> On Windows, launch the app **as Administrator**.

---

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                    Frontend (ui/)                    │
│  React + TypeScript + Tailwind + react-window       │
│  ┌──────────┐ ┌──────────────┐ ┌────────────────┐   │
│  │Controls  │ │ PacketList   │ │ PacketDetail    │   │
│  │(Start/   │ │ (virtualized)│ │ (side panel)    │   │
│  │ Stop)    │ │              │ │                 │   │
│  └────┬─────┘ └──────┬───────┘ └───────┬─────────┘   │
│       │              │                 │              │
│       └──────────────┼─────────────────┘              │
│                      │ Tauri IPC (events + commands)  │
├──────────────────────┼───────────────────────────────┤
│               Tauri  │  Bridge (src-tauri/)           │
│         ┌────────────┴────────────┐                   │
│         │  tauri::State<AppState> │                   │
│         │  Commands:              │                   │
│         │  list_interfaces()      │                   │
│         │  start_capture()        │                   │
│         │  stop_capture()         │                   │
│         └────────────┬────────────┘                   │
├──────────────────────┼───────────────────────────────┤
│           Core Library (src/)    │                    │
│  ┌───────────────────┴─────────────────────┐          │
│  │           CaptureEngine                 │          │
│  │  ┌──────────────┐  ┌──────────────────┐ │          │
│  │  │ pcap::Capture│  │ PacketAnalyzer   │ │          │
│  │  │ (separate    │  │ (etherparse)     │ │          │
│  │  │  thread)     │  │ → PacketHeader   │ │          │
│  │  └──────────────┘  └──────────────────┘ │          │
│  │  ↓ CapturedPacket (serde::Serialize)    │          │
│  └─────────────────────────────────────────┘          │
└─────────────────────────────────────────────────────┘
```

### Component roles

| Component | Responsibility |
|-----------|---------------|
| `CaptureEngine` | Opens a pcap capture on a device, runs in a dedicated OS thread, accumulates packets into a `Vec`, flushes via `CaptureEvent::PacketBatch` every 200 ms or 1000 packets |
| `PacketAnalyzer` | Parses raw bytes with `etherparse`, extracts Ethernet/IP/transport headers |
| `CapturedPacket` | Serializable model — fields: `id`, `timestamp`, `header`, `payload_preview` (first 16 bytes hex) |
| `tauri commands` | Bridge — `list_interfaces`, `start_capture`, `stop_capture` mapped to IPC |
| `tauri events` | Rust emits `capture-batch` (batched `Vec<CapturedPacket>`) — frontend listens via `@tauri-apps/api/event` |
| `react-window` | Renders only visible rows; thousands of packets without layout thrashing |

---

## Development

```bash
# Run the Tauri dev server (hot-reloads both Rust and UI)
cargo tauri dev

# Run the CLI capture tool (standalone)
cargo run --package flownet-core

# Run Rust tests
cargo test

# Lint
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cd ui && npx tsc --noEmit
```

---

## Future Roadmap

- **BPF Packet Filtering** — apply Berkeley Packet Filter expressions at the driver level to capture only relevant traffic (e.g. `tcp port 80`)
- **PCAP Export** — save captured packets to `.pcap` files for analysis in Wireshark / tshark
- **TCP Stream Visualizer** — reconstruct and display TCP conversation flow as sequence diagrams
- **Dark / Light Mode** — theme toggle with persistent preference across sessions

---

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).

---

## License

MIT
