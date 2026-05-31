import { useCallback } from "react";
import { FixedSizeList as List } from "react-window";
import type { CapturedPacket } from "../lib/packet";
import { ArrowRight, Activity } from "lucide-react";

interface Props {
  packets: CapturedPacket[];
  selectedId: number | null;
  onSelect: (pkt: CapturedPacket) => void;
}

const ROW_HEIGHT = 36;

export default function PacketList({ packets, selectedId, onSelect }: Props) {
  const total = packets.length;

  const Row = useCallback(
    ({ index, style }: { index: number; style: React.CSSProperties }) => {
      const pkt = packets[index];
      const h = pkt.header;
      const isSelected = pkt.id === selectedId;

      const protoColor = (p: string | null) => {
        switch (p) {
          case "TCP":
            return "text-cyan-400";
          case "UDP":
            return "text-yellow-400";
          case "ICMP":
            return "text-red-400";
          default:
            return "text-gray-400";
        }
      };

      return (
        <div
          style={style}
          onClick={() => onSelect(pkt)}
          className={`flex items-center px-3 border-b border-gray-800 cursor-pointer text-xs transition-colors ${
            isSelected
              ? "bg-cyan-900/30 border-l-2 border-l-cyan-400"
              : "hover:bg-gray-800/50"
          }`}
        >
          <span className="text-gray-500 w-12 shrink-0">{pkt.id}</span>
          <Activity
            className={`w-3.5 h-3.5 mr-2 shrink-0 ${protoColor(h.protocol_str)}`}
          />
          <span
            className={`w-12 shrink-0 font-medium ${protoColor(h.protocol_str)}`}
          >
            {h.protocol_str ?? "?"}
          </span>
          <span className="w-32 truncate text-gray-200 font-mono">
            {h.src_ip ?? h.src_mac ?? "??"}
          </span>
          {h.src_port && (
            <span className="text-gray-500 w-10 shrink-0 text-right font-mono">
              :{h.src_port}
            </span>
          )}
          <ArrowRight className="w-3 h-3 mx-1 text-gray-600 shrink-0" />
          <span className="w-32 truncate text-gray-200 font-mono">
            {h.dst_ip ?? h.dst_mac ?? "??"}
          </span>
          {h.dst_port && (
            <span className="text-gray-500 w-10 shrink-0 text-right font-mono">
              :{h.dst_port}
            </span>
          )}
          <span className="ml-auto text-gray-500 font-mono w-16 text-right">
            {h.length} B
          </span>
        </div>
      );
    },
    [packets, selectedId, onSelect],
  );

  return (
    <div className="flex-1 flex flex-col">
      <div className="flex items-center px-3 py-1.5 text-xs text-gray-500 border-b border-gray-800 bg-gray-900/50">
        <span className="w-12">ID</span>
        <span className="w-[52px]">Proto</span>
        <span className="w-32">Source</span>
        <span className="w-[68px]" />
        <span className="w-32">Destination</span>
        <span className="ml-auto w-16 text-right">Size</span>
      </div>
      <div className="flex-1">
        {total === 0 ? (
          <div className="flex items-center justify-center h-full text-gray-600 text-sm">
            No packets captured. Select an interface and press Start.
          </div>
        ) : (
          <List
            height={600}
            itemCount={total}
            itemSize={ROW_HEIGHT}
            width="100%"
            overscanCount={20}
          >
            {Row}
          </List>
        )}
      </div>
      <div className="px-3 py-1 text-xs text-gray-500 border-t border-gray-800">
        {total} packet{total !== 1 ? "s" : ""}
      </div>
    </div>
  );
}
