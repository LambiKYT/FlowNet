import { useCallback, useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import type { CapturedPacket } from "../lib/packet";

export function usePacketCapture() {
  const [packets, setPackets] = useState<CapturedPacket[]>([]);
  const [isCapturing, setIsCapturing] = useState(false);
  const [interfaces, setInterfaces] = useState<string[]>([]);
  const [error, setError] = useState<string | null>(null);
  const unlistenRef = useRef<(() => void)[]>([]);

  useEffect(() => {
    invoke<string[]>("list_interfaces")
      .then(setInterfaces)
      .catch((e) => setError(String(e)));
  }, []);

  const startCapture = useCallback(async (device: string) => {
    unlistenRef.current.forEach((u) => u());
    unlistenRef.current = [];
    setError(null);
    try {
      const unlistenBatch = await listen<CapturedPacket[]>(
        "capture-batch",
        (event) => {
          setPackets((prev) => [...prev, ...event.payload]);
        },
      );
      const unlistenError = await listen<string>("capture-error", (event) => {
        setError(event.payload);
      });
      const unlistenStop = await listen("capture-stopped", () => {
        setIsCapturing(false);
      });

      unlistenRef.current = [unlistenBatch, unlistenError, unlistenStop];
      await invoke("start_capture", { device });
      setIsCapturing(true);
    } catch (e) {
      setError(String(e));
    }
  }, []);

  const stopCapture = useCallback(async () => {
    try {
      await invoke("stop_capture");
      setIsCapturing(false);
    } catch (e) {
      setError(String(e));
    }
  }, []);

  const clearPackets = useCallback(() => setPackets([]), []);

  useEffect(() => {
    return () => {
      unlistenRef.current.forEach((u) => u());
    };
  }, []);

  return {
    packets,
    isCapturing,
    interfaces,
    error,
    startCapture,
    stopCapture,
    clearPackets,
  } as const;
}
