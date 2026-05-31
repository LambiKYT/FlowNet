use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Instant;
use tracing::{error, info, warn};

use crate::models::packet::{payload_preview, CapturedPacket};
use crate::processor::analyzer::PacketAnalyzer;

const BATCH_INTERVAL_MS: u64 = 200;
const BATCH_MAX_SIZE: usize = 1000;

pub enum CaptureEvent {
    PacketBatch(Vec<CapturedPacket>),
    Error(String),
    Stopped,
}

pub struct CaptureEngine {
    running: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Default for CaptureEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CaptureEngine {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    pub fn list_devices() -> Result<Vec<String>, String> {
        let cap = pcap::Device::list().map_err(|e| format!("Failed to list devices: {}", e))?;
        Ok(cap.into_iter().map(|d| d.name).collect())
    }

    pub fn start<F>(&mut self, device: &str, mut on_event: F)
    where
        F: FnMut(CaptureEvent) + Send + 'static,
    {
        if self.running.load(Ordering::SeqCst) {
            warn!("Capture engine is already running");
            return;
        }

        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let dev_name = device.to_string();

        self.handle = Some(thread::spawn(move || {
            let mut cap = match Self::open_device(&dev_name) {
                Ok(c) => c,
                Err(e) => {
                    on_event(CaptureEvent::Error(e));
                    return;
                }
            };

            info!("Capture started on interface '{}'", dev_name);

            let analyzer = PacketAnalyzer::new();
            let mut batch = Vec::with_capacity(BATCH_MAX_SIZE);
            let mut last_flush = Instant::now();
            let batch_interval = std::time::Duration::from_millis(BATCH_INTERVAL_MS);

            let flush = |b: &mut Vec<CapturedPacket>, cb: &mut F| {
                if !b.is_empty() {
                    cb(CaptureEvent::PacketBatch(std::mem::take(b)));
                }
            };

            loop {
                if !running.load(Ordering::SeqCst) {
                    flush(&mut batch, &mut on_event);
                    break;
                }

                match cap.next_packet() {
                    Ok(pkt) => {
                        let parsed = analyzer.analyze(pkt.data);
                        let captured = CapturedPacket {
                            id: pkt.header.len as u64,
                            timestamp: chrono::Utc::now(),
                            header: parsed,
                            payload_preview: payload_preview(pkt.data),
                        };
                        batch.push(captured);

                        if batch.len() >= BATCH_MAX_SIZE || last_flush.elapsed() >= batch_interval {
                            flush(&mut batch, &mut on_event);
                            last_flush = Instant::now();
                        }
                    }
                    Err(pcap::Error::TimeoutExpired) => {
                        if !batch.is_empty() && last_flush.elapsed() >= batch_interval {
                            flush(&mut batch, &mut on_event);
                            last_flush = Instant::now();
                        }
                    }
                    Err(e) => {
                        error!("Capture error: {}", e);
                        flush(&mut batch, &mut on_event);
                        on_event(CaptureEvent::Error(format!("{}", e)));
                        break;
                    }
                }
            }

            info!("Capture stopped");
            on_event(CaptureEvent::Stopped);
        }));
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
        info!("Capture engine shut down");
    }

    fn open_device(name: &str) -> Result<pcap::Capture<pcap::Active>, String> {
        let device = pcap::Device::list()
            .map_err(|e| format!("Cannot list devices: {}", e))?
            .into_iter()
            .find(|d| d.name == name)
            .ok_or_else(|| format!("Device '{}' not found", name))?;

        pcap::Capture::from_device(device)
            .map_err(|e| format!("Cannot open device: {}", e))?
            .promisc(true)
            .snaplen(65535)
            .timeout(100)
            .immediate_mode(true)
            .open()
            .map_err(|e| format!("Cannot open capture: {}", e))
    }
}

impl Drop for CaptureEngine {
    fn drop(&mut self) {
        self.stop();
    }
}
