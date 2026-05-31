import { useState } from "react";
import { Play, Square, Trash2, Wifi } from "lucide-react";

interface Props {
  interfaces: string[];
  isCapturing: boolean;
  onStart: (device: string) => void;
  onStop: () => void;
  onClear: () => void;
}

export default function CaptureControls({
  interfaces,
  isCapturing,
  onStart,
  onStop,
  onClear,
}: Props) {
  const [selected, setSelected] = useState(interfaces[0] ?? "");

  return (
    <div className="flex items-center gap-3 p-3 border-b border-gray-800 bg-gray-900">
      <Wifi className="w-5 h-5 text-cyan-400" />
      <select
        className="bg-gray-800 border border-gray-700 rounded px-2 py-1 text-sm focus:outline-none focus:border-cyan-500"
        value={selected}
        onChange={(e) => setSelected(e.target.value)}
        disabled={isCapturing}
      >
        {interfaces.map((iface) => (
          <option key={iface} value={iface}>
            {iface}
          </option>
        ))}
      </select>

      {isCapturing ? (
        <button
          onClick={onStop}
          className="flex items-center gap-1 px-3 py-1 bg-red-600 hover:bg-red-500 rounded text-sm font-medium"
        >
          <Square className="w-4 h-4" /> Stop
        </button>
      ) : (
        <button
          onClick={() => onStart(selected)}
          disabled={!selected}
          className="flex items-center gap-1 px-3 py-1 bg-green-600 hover:bg-green-500 disabled:opacity-40 rounded text-sm font-medium"
        >
          <Play className="w-4 h-4" /> Start
        </button>
      )}

      <button
        onClick={onClear}
        disabled={isCapturing}
        className="flex items-center gap-1 px-3 py-1 bg-gray-700 hover:bg-gray-600 disabled:opacity-40 rounded text-sm"
      >
        <Trash2 className="w-4 h-4" /> Clear
      </button>
    </div>
  );
}
