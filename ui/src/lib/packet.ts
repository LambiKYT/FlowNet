export interface PacketHeader {
  src_mac: string | null;
  dst_mac: string | null;
  ether_type: number | null;
  src_ip: string | null;
  dst_ip: string | null;
  protocol: number | null;
  protocol_str: string | null;
  src_port: number | null;
  dst_port: number | null;
  ttl: number | null;
  length: number;
}

export interface CapturedPacket {
  id: number;
  timestamp: string;
  header: PacketHeader;
  payload_preview: string;
}
