use flownet_core::capture::engine::{CaptureEngine, CaptureEvent};
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let devices = CaptureEngine::list_devices().expect("Failed to list network interfaces");
    if devices.is_empty() {
        eprintln!("No network interfaces found. Are you running with sufficient privileges?");
        std::process::exit(1);
    }

    println!("Available network interfaces:");
    for (i, dev) in devices.iter().enumerate() {
        println!("  [{}] {}", i, dev);
    }

    let idx: usize = dialoguer::Select::new()
        .with_prompt("Select an interface to capture")
        .items(&devices)
        .default(0)
        .interact()
        .unwrap_or(0);

    let device = &devices[idx];
    println!(
        "Starting capture on '{}'... Press Ctrl+C to stop.\n",
        device
    );

    let mut engine = CaptureEngine::new();
    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    engine.start(device, move |event| match event {
        CaptureEvent::PacketBatch(batch) => {
            for pkt in batch {
                println!("{}", serde_json::to_string(&pkt).unwrap());
            }
        }
        CaptureEvent::Error(e) => {
            eprintln!("Capture error: {}", e);
        }
        CaptureEvent::Stopped => {
            println!("Capture stopped.");
        }
    });

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    engine.stop();
    println!("Exiting.");
}
