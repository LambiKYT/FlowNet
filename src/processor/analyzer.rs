use etherparse::*;
use std::net::{Ipv4Addr, Ipv6Addr};
use tracing::trace;

use crate::models::packet::PacketHeader;

pub struct PacketAnalyzer;

impl Default for PacketAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl PacketAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, data: &[u8]) -> PacketHeader {
        let mut h = PacketHeader {
            src_mac: None,
            dst_mac: None,
            ether_type: None,
            src_ip: None,
            dst_ip: None,
            protocol: None,
            protocol_str: None,
            src_port: None,
            dst_port: None,
            ttl: None,
            length: data.len(),
        };

        if let Ok(eth) = Ethernet2Header::from_slice(data) {
            let (eth_hdr, rest) = eth;
            h.src_mac = Some(mac_to_string(&eth_hdr.source));
            h.dst_mac = Some(mac_to_string(&eth_hdr.destination));
            h.ether_type = Some(eth_hdr.ether_type);

            match eth_hdr.ether_type {
                0x0800 => {
                    if let Ok(ip) = Ipv4Header::from_slice(rest) {
                        let (ip_hdr, payload) = ip;
                        h.src_ip = Some(Ipv4Addr::from(ip_hdr.source).to_string());
                        h.dst_ip = Some(Ipv4Addr::from(ip_hdr.destination).to_string());
                        h.protocol = Some(ip_hdr.protocol);
                        h.ttl = Some(ip_hdr.time_to_live);
                        h.protocol_str = Some(protocol_name(ip_hdr.protocol).to_string());
                        Self::parse_transport(ip_hdr.protocol, payload, &mut h);
                    }
                }
                0x86DD => {
                    if let Ok(ip) = Ipv6Header::from_slice(rest) {
                        let (ip_hdr, payload) = ip;
                        h.src_ip = Some(Ipv6Addr::from(ip_hdr.source).to_string());
                        h.dst_ip = Some(Ipv6Addr::from(ip_hdr.destination).to_string());
                        h.protocol = Some(ip_hdr.next_header);
                        h.ttl = Some(ip_hdr.hop_limit);
                        h.protocol_str = Some(protocol_name(ip_hdr.next_header).to_string());
                        Self::parse_transport(ip_hdr.next_header, payload, &mut h);
                    }
                }
                _ => {
                    trace!("Unknown ether type: 0x{:04X}", eth_hdr.ether_type);
                }
            }
        }

        h
    }

    fn parse_transport(protocol: u8, data: &[u8], h: &mut PacketHeader) {
        if let 6 | 17 = protocol {
            if data.len() >= 4 {
                h.src_port = Some(u16::from_be_bytes([data[0], data[1]]));
                h.dst_port = Some(u16::from_be_bytes([data[2], data[3]]));
            }
        }
    }
}

fn mac_to_string(mac: &[u8; 6]) -> String {
    format!(
        "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
    )
}

fn protocol_name(p: u8) -> &'static str {
    match p {
        1 => "ICMP",
        6 => "TCP",
        17 => "UDP",
        58 => "ICMPv6",
        89 => "OSPF",
        132 => "SCTP",
        _ => "OTHER",
    }
}
