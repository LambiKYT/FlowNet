import { useState } from "react";
import { usePacketCapture } from "./hooks/usePacketCapture";
import CaptureControls from "./components/CaptureControls";
import PacketList from "./components/PacketList";
import PacketDetail from "./components/PacketDetail";
import type { CapturedPacket } from "./lib/packet";
import { Activity } from "lucide-react";

export default function App() {
  const {
    packets,
    isCapturing,
    interfaces,
    error,
    startCapture,
    stopCapture,
    clearPackets,
  } = usePacketCapture();

  const [selected, setSelected] = useState<CapturedPacket | null>(null);

  return (
    <div className="h-screen flex flex-col">
      <header className="flex items-center gap-2 px-4 py-2 bg-gray-900 border-b border-gray-800">
        <Activity className="w-5 h-5 text-cyan-400" />
        <h1 className="text-sm font-semibold tracking-wide">FlowNet</h1>
        <span className="text-xs text-gray-500 ml-1">Network Traffic Analyzer</span>
      </header>

      <CaptureControls
        interfaces={interfaces}
        isCapturing={isCapturing}
        onStart={startCapture}
        onStop={stopCapture}
        onClear={clearPackets}
      />

      {error && (
        <div className="px-3 py-1.5 bg-red-900/50 text-red-300 text-xs border-b border-red-800">
          {error}
        </div>
      )}

      <div className="flex flex-1 overflow-hidden">
        <PacketList
          packets={packets}
          selectedId={selected?.id ?? null}
          onSelect={setSelected}
        />
        {selected && (
          <PacketDetail packet={selected} onClose={() => setSelected(null)} />
        )}
      </div>
    </div>
  );
}
