import type { CapturedPacket } from "../lib/packet";
import { X, Network, Link, Hash } from "lucide-react";

interface Props {
  packet: CapturedPacket;
  onClose: () => void;
}

export default function PacketDetail({ packet, onClose }: Props) {
  const h = packet.header;

  return (
    <div className="border-l border-gray-800 bg-gray-900 w-96 overflow-y-auto">
      <div className="flex items-center justify-between p-3 border-b border-gray-800">
        <h2 className="text-sm font-semibold">Packet #{packet.id}</h2>
        <button
          onClick={onClose}
          className="p-1 hover:bg-gray-700 rounded"
        >
          <X className="w-4 h-4" />
        </button>
      </div>

      <div className="p-3 space-y-4 text-xs">
        <Section icon={<Network className="w-4 h-4" />} title="Network">
          <Row label="Protocol" value={h.protocol_str ?? "-"} />
          <Row label="TTL" value={h.ttl?.toString() ?? "-"} />
          <Row
            label="Length"
            value={`${h.length} B`}
          />
        </Section>

        <Section icon={<Link className="w-4 h-4" />} title="Addresses">
          <Row label="Src IP" value={h.src_ip ?? "-"} />
          <Row label="Dst IP" value={h.dst_ip ?? "-"} />
          <Row label="Src MAC" value={h.src_mac ?? "-"} />
          <Row label="Dst MAC" value={h.dst_mac ?? "-"} />
          <Row label="Src Port" value={h.src_port?.toString() ?? "-"} />
          <Row label="Dst Port" value={h.dst_port?.toString() ?? "-"} />
        </Section>

        <Section icon={<Hash className="w-4 h-4" />} title="Raw">
          <Row label="Raw Length" value={`${packet.header.length} B`} />
          <Row
            label="Timestamp"
            value={new Date(packet.timestamp).toLocaleTimeString()}
          />
        </Section>
      </div>
    </div>
  );
}

function Section({
  icon,
  title,
  children,
}: {
  icon: React.ReactNode;
  title: string;
  children: React.ReactNode;
}) {
  return (
    <div>
      <div className="flex items-center gap-1.5 mb-1.5 text-gray-400 uppercase tracking-wider font-medium">
        {icon}
        <span>{title}</span>
      </div>
      <div className="space-y-1">{children}</div>
    </div>
  );
}

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex justify-between">
      <span className="text-gray-500">{label}</span>
      <span className="text-gray-200 font-mono truncate ml-2">{value}</span>
    </div>
  );
}
