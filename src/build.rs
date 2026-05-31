use std::path::Path;

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "windows" {
        println!("cargo:warning=FlowNet build.rs: skipping Npcap search on non-Windows target ({target_os})");
        return;
    }

    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let lib_subdir = match target_arch.as_str() {
        "x86_64" => "Lib\\x64",
        "x86" => "Lib\\x86",
        other => panic!(
            "FlowNet build.rs: unsupported target architecture '{other}'. \
             Npcap SDK only ships libraries for x86_64 and x86."
        ),
    };

    let candidates: &[&str] = &[
        "NPCAP_SDK_PATH",
        "C:\\Program Files\\Npcap SDK",
        "C:\\Program Files\\Npcap",
        "C:\\Program Files (x86)\\Npcap SDK",
        "C:\\Program Files (x86)\\Npcap",
        "C:\\NpcapSDK",
    ];

    let mut checked: Vec<String> = Vec::new();
    let mut resolved: Option<(String, String)> = None;

    for &candidate in candidates {
        let dir = resolve(candidate);
        let display = match &dir {
            Some(p) => p.clone(),
            None => candidate.to_string(),
        };
        println!("cargo:warning=Checked path: {display}");

        if let Some(path) = &dir {
            let root = path.trim_end_matches('\\').to_string();
            let lib_dir = format!("{root}\\{lib_subdir}");

            let wpcap = Path::new(&lib_dir).join("wpcap.lib");
            let packet = Path::new(&lib_dir).join("Packet.lib");

            if wpcap.exists() && packet.exists() {
                resolved = Some((root, lib_dir));
                break;
            }
        }

        checked.push(display);
    }

    let (_root, lib_dir) = resolved.unwrap_or_else(|| {
        let mut msg = "\nFlowNet build.rs: Npcap SDK not found.\n".to_string();
        msg.push_str("Checked the following locations:\n");
        for p in &checked {
            msg.push_str(&format!("  - {p}\n"));
        }
        msg.push_str("\nInstall the Npcap SDK from https://npcap.com/ ");
        msg.push_str("and make sure at least one of the paths above contains:\n");
        msg.push_str(&format!("  {lib_subdir}\\wpcap.lib\n"));
        msg.push_str(&format!("  {lib_subdir}\\Packet.lib\n"));
        panic!("{msg}");
    });

    println!("cargo:rustc-link-search=native={lib_dir}");
    println!("cargo:rustc-link-lib=wpcap");
    println!("cargo:rustc-link-lib=Packet");
    println!("cargo:rerun-if-env-changed=NPCAP_SDK_PATH");

    println!("cargo:warning=FlowNet build.rs: linked Npcap SDK from '{lib_dir}'");
}

fn resolve(candidate: &str) -> Option<String> {
    if candidate == "NPCAP_SDK_PATH" {
        return std::env::var("NPCAP_SDK_PATH").ok();
    }

    let p = Path::new(candidate);
    if p.exists() {
        Some(candidate.to_string())
    } else {
        None
    }
}
