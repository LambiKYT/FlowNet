use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketHeader {
    pub src_mac: Option<String>,
    pub dst_mac: Option<String>,
    pub ether_type: Option<u16>,
    pub src_ip: Option<String>,
    pub dst_ip: Option<String>,
    pub protocol: Option<u8>,
    pub protocol_str: Option<String>,
    pub src_port: Option<u16>,
    pub dst_port: Option<u16>,
    pub ttl: Option<u8>,
    pub length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedPacket {
    pub id: u64,
    pub timestamp: DateTime<Utc>,
    pub header: PacketHeader,
    pub payload_preview: String,
}

impl CapturedPacket {
    pub fn summary(&self) -> String {
        let h = &self.header;
        let src = h.src_ip.as_deref().or(h.src_mac.as_deref()).unwrap_or("??");
        let dst = h.dst_ip.as_deref().or(h.dst_mac.as_deref()).unwrap_or("??");
        let proto = h.protocol_str.as_deref().unwrap_or("?");
        let ports = match (h.src_port, h.dst_port) {
            (Some(sp), Some(dp)) => format!(":{} → :{}", sp, dp),
            _ => String::new(),
        };
        format!("{} {} → {}{} ({} B)", proto, src, dst, ports, h.length)
    }
}

pub fn payload_preview(data: &[u8]) -> String {
    let preview_len = data.len().min(16);
    let mut out = String::with_capacity(preview_len * 3);
    for &byte in data.iter().take(preview_len) {
        out.push_str(&format!("{:02x} ", byte));
    }
    out.pop();
    out
}
