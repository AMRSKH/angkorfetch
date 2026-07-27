use colored::*;
use std::env;
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{
    CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System,
};

#[cfg(windows)]
#[link(name = "user32")]
extern "system" {
    fn GetDC(hWnd: *const std::ffi::c_void) -> *const std::ffi::c_void;
    fn ReleaseDC(hWnd: *const std::ffi::c_void, hDC: *const std::ffi::c_void) -> i32;
}

#[cfg(windows)]
#[link(name = "gdi32")]
extern "system" {
    fn GetDeviceCaps(hdc: *const std::ffi::c_void, nIndex: i32) -> i32;
}

#[cfg(windows)]
#[link(name = "kernel32")]
extern "system" {
    fn GetSystemPowerStatus(lpSystemPowerStatus: *mut SYSTEM_POWER_STATUS) -> u32;
}

#[cfg(windows)]
#[repr(C)]
struct SYSTEM_POWER_STATUS {
    ac_line_status: u8,
    battery_flag: u8,
    battery_life_percent: u8,
    system_status_flag: u8,
    battery_life_time: u32,
    battery_full_life_time: u32,
}

fn bytes_to_string(bytes: Vec<u8>) -> String {
    if bytes.len() >= 2 && bytes[0] == 0xFF && bytes[1] == 0xFE {
        let u16: Vec<u16> = bytes[2..]
            .chunks(2)
            .filter(|c| c.len() == 2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        String::from_utf16_lossy(&u16)
    } else {
        String::from_utf8_lossy(&bytes).to_string()
    }
}

fn run_stdout(cmd: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(cmd)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()
        .and_then(|c| c.wait_with_output().ok())?;
    if output.status.success() {
        let s = bytes_to_string(output.stdout);
        let trimmed = s.trim().to_string();
        if trimmed.is_empty() { None } else { Some(trimmed) }
    } else { None }
}

fn run_powershell(script: &str) -> Option<String> {
    run_stdout("powershell", &["-NoProfile", "-Command", script])
}

fn reg_val(key: &str, name: &str) -> Option<String> {
    run_stdout("reg", &["query", key, "/v", name]).and_then(|s| {
        s.lines().last().and_then(|line| {
            let parts: Vec<&str> = line.split("REG_SZ").collect();
            if parts.len() >= 2 { Some(parts[1].trim().to_string()) } else { None }
        })
    }).filter(|s| !s.is_empty())
}

fn print_help(prog: &str) {
    println!("Usage: {} [OPTIONS]", prog);
    println!();
    println!("A fast, cross-platform system fetch tool");
    println!();
    println!("Options:");
    println!("  -h, --help              Show this help message and exit");
    println!("  -v, --version           Show version information and exit");
    println!("      --hinfo, --hard     Show detailed hardware information");
}

fn get_arch() -> String {
    if cfg!(target_os = "windows") {
        env::var("PROCESSOR_ARCHITECTURE")
            .unwrap_or_default()
            .replace("AMD64", "x86_64")
            .replace("ARM64", "aarch64")
    } else {
        run_stdout("uname", &["-m"]).unwrap_or_else(|| "Unknown".to_string())
    }
}

fn get_os_name(arch: &str) -> String {
    let os = System::long_os_version().unwrap_or_else(|| "Unknown".to_string());
    if cfg!(target_os = "windows") {
        let build = reg_val(
            r"HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion",
            "DisplayVersion",
        ).unwrap_or_default();
        if build.is_empty() {
            format!("{} [{}]", os, arch)
        } else {
            format!("{} - {} [{}]", os, build, arch)
        }
    } else {
        format!("{} [{}]", os, arch)
    }
}

fn get_host() -> String {
    System::host_name()
        .or_else(|| env::var("COMPUTERNAME").ok())
        .or_else(|| env::var("HOSTNAME").ok())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn get_model() -> String {
    if cfg!(target_os = "windows") {
        let manu = reg_val(r"HKLM\HARDWARE\DESCRIPTION\System\BIOS", "SystemManufacturer").unwrap_or_default();
        let prod = reg_val(r"HKLM\HARDWARE\DESCRIPTION\System\BIOS", "SystemProductName").unwrap_or_default();
        if manu.is_empty() && prod.is_empty() { String::new() } else { format!("{} {}", manu, prod).trim().to_string() }
    } else if cfg!(target_os = "linux") {
        std::fs::read_to_string("/sys/class/dmi/id/product_name").ok().map(|s| s.trim().to_string()).unwrap_or_default()
    } else if cfg!(target_os = "macos") {
        run_stdout("sysctl", &["-n", "hw.model"]).unwrap_or_default()
    } else { String::new() }
}

fn get_serial() -> String {
    if cfg!(target_os = "windows") {
        run_powershell("(Get-CimInstance Win32_BIOS).SerialNumber")
            .unwrap_or_else(|| "Unknown".to_string())
    } else if cfg!(target_os = "linux") {
        std::fs::read_to_string("/sys/class/dmi/id/product_serial").ok()
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    } else if cfg!(target_os = "macos") {
        run_stdout("sh", &["-c", "system_profiler SPHardwareDataType 2>/dev/null | grep 'Serial Number' | head -1 | cut -d: -f2"])
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    } else {
        "Unknown".to_string()
    }
}

fn get_motherboard() -> String {
    if cfg!(target_os = "windows") {
        let key = r"HKLM\HARDWARE\DESCRIPTION\System\BIOS";
        let manu = reg_val(key, "BaseBoardManufacturer").unwrap_or_default();
        let prod = reg_val(key, "BaseBoardProduct").unwrap_or_default();
        let combined = format!("{} {}", manu, prod).trim().to_string();
        if combined.is_empty() { "Unknown".to_string() } else { combined }
    } else if cfg!(target_os = "linux") {
        let vendor = std::fs::read_to_string("/sys/class/dmi/id/board_vendor")
            .ok().map(|s| s.trim().to_string()).unwrap_or_default();
        let name = std::fs::read_to_string("/sys/class/dmi/id/board_name")
            .ok().map(|s| s.trim().to_string()).unwrap_or_default();
        let combined = format!("{} {}", vendor, name).trim().to_string();
        if combined.is_empty() { "Unknown".to_string() } else { combined }
    } else if cfg!(target_os = "macos") {
        run_stdout("sysctl", &["-n", "hw.model"])
            .map(|m| format!("Apple {}", m))
            .unwrap_or_else(|| "Apple".to_string())
    } else {
        "Unknown".to_string()
    }
}

fn get_bios() -> String {
    if cfg!(target_os = "windows") {
        let key = r"HKLM\HARDWARE\DESCRIPTION\System\BIOS";
        let vendor = reg_val(key, "BIOSVendor").unwrap_or_default();
        let version = reg_val(key, "BIOSVersion").unwrap_or_default();
        let combined = format!("{} {}", vendor, version).trim().to_string();
        if combined.is_empty() { "Unknown".to_string() } else { combined }
    } else if cfg!(target_os = "linux") {
        let vendor = std::fs::read_to_string("/sys/class/dmi/id/bios_vendor")
            .ok().map(|s| s.trim().to_string()).unwrap_or_default();
        let version = std::fs::read_to_string("/sys/class/dmi/id/bios_version")
            .ok().map(|s| s.trim().to_string()).unwrap_or_default();
        let combined = format!("{} {}", vendor, version).trim().to_string();
        if combined.is_empty() { "Unknown".to_string() } else { combined }
    } else {
        "Unknown".to_string()
    }
}

fn get_uptime() -> String {
    let secs = System::uptime();
    let d = secs / 86400;
    let h = (secs % 86400) / 3600;
    let m = (secs % 3600) / 60;
    let mut parts = Vec::new();
    if d > 0 { parts.push(format!("{}d", d)); }
    if h > 0 { parts.push(format!("{}h", h)); }
    parts.push(format!("{}m", m));
    parts.join(" ")
}

fn fmt_cpu_brand(model: &str) -> String {
    model
        .replace("(R)", "®")
        .replace("(TM)", "™")
        .replace(" CPU", "")
        .replace(" @ 0.00GHz", "")
}

fn get_cpu_info(sys: &System) -> String {
    let cpus = sys.cpus();
    if cpus.is_empty() { return "Unknown".to_string(); }
    let model = fmt_cpu_brand(cpus[0].brand().trim());
    let freq = cpus[0].frequency() as f64 / 1000.0;
    format!("{} ({} cores) @ {:.2} GHz", model, cpus.len(), freq)
}

fn get_gpu() -> String {
    if cfg!(target_os = "windows") {
        if let Some(name) = reg_val(
            r"HKLM\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\0000",
            "DriverDesc",
        ) {
            return name;
        }
        "Unknown".to_string()
    } else if cfg!(target_os = "linux") {
        run_stdout("sh", &["-c", "lspci | grep -i 'vga\\|3d' | head -1 | cut -d: -f3-"])
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    } else if cfg!(target_os = "macos") {
        run_stdout("sh", &["-c", "system_profiler SPDisplaysDataType 2>/dev/null | grep 'Chipset Model' | head -1 | cut -d: -f2"])
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    } else {
        "Unknown".to_string()
    }
}

fn get_gpu_usage() -> String {
    if let Some(out) = run_stdout(
        "nvidia-smi",
        &["--query-gpu=utilization.gpu", "--format=csv,noheader,nounits"],
    ) {
        if let Some(first) = out.lines().next() {
            if let Ok(n) = first.trim().parse::<u32>() {
                return format!("{}%", n);
            }
        }
    }
    if cfg!(target_os = "linux") {
        for path in &[
            "/sys/class/drm/card0/device/gpu_busy_percent",
            "/sys/class/drm/card1/device/gpu_busy_percent",
        ] {
            if let Ok(s) = std::fs::read_to_string(path) {
                let t = s.trim();
                if !t.is_empty() {
                    return format!("{}%", t);
                }
            }
        }
    }
    "N/A".to_string()
}

fn get_memory(sys: &System) -> String {
    let total = sys.total_memory() as f64 / 1_073_741_824.0;
    let used = sys.used_memory() as f64 / 1_073_741_824.0;
    let pct = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };
    format!("{:.1} GiB / {:.1} GiB ({:.0}%)", used, total, pct)
}

fn get_ram_type() -> String {
    if cfg!(target_os = "windows") {
        run_powershell("$t=(Get-CimInstance Win32_PhysicalMemory | Select-Object -First 1).SMBIOSMemoryType; switch($t){20{'DDR'}21{'DDR2'}24{'DDR3'}26{'DDR4'}34{'DDR5'}default{\"Unknown ($t)\"}}")
            .unwrap_or_else(|| "Unknown".to_string())
    } else if cfg!(target_os = "linux") {
        run_stdout("sh", &["-c", "dmidecode -t memory 2>/dev/null | grep 'Type:' | head -1 | cut -d: -f2"])
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    } else if cfg!(target_os = "macos") {
        run_stdout("sh", &["-c", "system_profiler SPMemoryDataType 2>/dev/null | grep 'Type:' | head -1 | cut -d: -f2"])
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    } else {
        "Unknown".to_string()
    }
}

fn get_ram_speed_mhz() -> Option<u32> {
    if cfg!(target_os = "windows") {
        run_powershell("(Get-CimInstance Win32_PhysicalMemory | Select-Object -First 1).Speed")
            .and_then(|s| s.trim().parse::<u32>().ok())
    } else if cfg!(target_os = "linux") {
        run_stdout(
            "sh",
            &["-c", "dmidecode -t memory 2>/dev/null | grep -m1 -E 'Speed:[[:space:]]+[0-9]+ MT/s' | grep -oE '[0-9]+'"],
        )
        .and_then(|s| s.trim().parse::<u32>().ok())
    } else if cfg!(target_os = "macos") {
        run_stdout(
            "sh",
            &["-c", "system_profiler SPMemoryDataType 2>/dev/null | grep -m1 Speed | grep -oE '[0-9]+'"],
        )
        .and_then(|s| s.trim().parse::<u32>().ok())
    } else {
        None
    }
}

fn get_ram_manufacturer() -> String {
    if cfg!(target_os = "windows") {
        run_powershell("(Get-CimInstance Win32_PhysicalMemory | Select-Object -First 1).Manufacturer")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty() && s != "Unknown")
            .unwrap_or_default()
    } else if cfg!(target_os = "linux") {
        run_stdout(
            "sh",
            &["-c", "dmidecode -t memory 2>/dev/null | grep -m1 -E 'Manufacturer:[[:space:]]*[^ ]' | cut -d: -f2"],
        )
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && !s.eq_ignore_ascii_case("Unknown") && !s.eq_ignore_ascii_case("Not Specified"))
        .unwrap_or_default()
    } else if cfg!(target_os = "macos") {
        run_stdout(
            "sh",
            &["-c", "system_profiler SPMemoryDataType 2>/dev/null | grep -m1 Manufacturer | cut -d: -f2"],
        )
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_default()
    } else {
        String::new()
    }
}

/// Combines RAM type, speed and (when available) manufacturer into one
/// readable line, e.g. "DDR4 @ 3200 MHz (Corsair)".
fn get_ram_details() -> String {
    let ty = get_ram_type();
    let speed = get_ram_speed_mhz();
    let vendor = get_ram_manufacturer();

    let base = match speed {
        Some(mhz) if mhz > 0 => format!("{} @ {} MHz", ty, mhz),
        _ => ty,
    };

    if vendor.is_empty() {
        base
    } else {
        format!("{} ({})", base, vendor)
    }
}

fn get_disk() -> String {
    let disks = Disks::new_with_refreshed_list();
    let total: u64 = disks.iter().map(|d| d.total_space()).sum();
    let avail: u64 = disks.iter().map(|d| d.available_space()).sum();
    let used = total - avail;
    let total_gb = total as f64 / 1_000_000_000.0;
    let used_gb = used as f64 / 1_000_000_000.0;
    let pct = if total > 0 { (used as f64 / total as f64) * 100.0 } else { 0.0 };
    format!("{:.1} GiB / {:.1} GiB ({:.0}%)", used_gb, total_gb, pct)
}

/// Brand/model string of the primary disk, e.g. "Samsung SSD 970 EVO Plus 1TB".
fn get_disk_model() -> String {
    if cfg!(target_os = "windows") {
        run_powershell("(Get-CimInstance Win32_DiskDrive | Select-Object -First 1).Model")
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Unknown".to_string())
    } else if cfg!(target_os = "linux") {
        run_stdout(
            "sh",
            &[
                "-c",
                "for d in /sys/block/*/device/model; do v=$(cat \"$d\" 2>/dev/null | sed 's/[[:space:]]*$//'); [ -n \"$v\" ] && echo \"$v\" && break; done",
            ],
        )
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Unknown".to_string())
    } else if cfg!(target_os = "macos") {
        run_stdout(
            "sh",
            &[
                "-c",
                "system_profiler SPNVMeDataType SPSerialATADataType 2>/dev/null | grep -m1 -E '(Device Name|Model):' | cut -d: -f2",
            ],
        )
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Unknown".to_string())
    } else {
        "Unknown".to_string()
    }
}

/// Storage medium classification (NVMe SSD / SATA SSD / HDD).
fn get_disk_support() -> String {
    if cfg!(target_os = "windows") {
        let model = run_powershell("(Get-CimInstance Win32_DiskDrive | Select-Object -First 1).Model").unwrap_or_default();
        let bus = run_powershell("(Get-CimInstance Win32_DiskDrive | Select-Object -First 1).InterfaceType").unwrap_or_default();
        let upper = format!("{} {}", model, bus).to_uppercase();
        if upper.contains("NVME") {
            "NVMe SSD".to_string()
        } else if upper.contains("SSD") {
            "SATA SSD".to_string()
        } else if upper.contains("HDD") {
            "HDD".to_string()
        } else {
            "Unknown".to_string()
        }
    } else if cfg!(target_os = "linux") {
        run_stdout(
            "sh",
            &[
                "-c",
                "d=$(lsblk -dno NAME,TYPE 2>/dev/null | awk '$2==\"disk\"{print $1; exit}'); \
                 if echo \"$d\" | grep -q '^nvme'; then echo 'NVMe SSD'; \
                 elif [ \"$(cat /sys/block/$d/queue/rotational 2>/dev/null)\" = '0' ]; then echo 'SATA SSD'; \
                 elif [ -n \"$d\" ]; then echo HDD; else echo Unknown; fi",
            ],
        )
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Unknown".to_string())
    } else if cfg!(target_os = "macos") {
        run_stdout(
            "sh",
            &[
                "-c",
                "if system_profiler SPNVMeDataType 2>/dev/null | grep -q .; then echo 'NVMe SSD'; \
                 else system_profiler SPStorageDataType 2>/dev/null | grep -m1 'Medium Type' | cut -d: -f2; fi",
            ],
        )
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "Unknown".to_string())
    } else {
        "Unknown".to_string()
    }
}

fn get_display() -> String {
    if cfg!(target_os = "windows") {
        #[cfg(windows)]
        {
            unsafe {
                let hdc = GetDC(std::ptr::null());
                if hdc.is_null() { return "Unknown".to_string(); }
                let w = GetDeviceCaps(hdc, 8);
                let h = GetDeviceCaps(hdc, 10);
                let r = GetDeviceCaps(hdc, 116);
                ReleaseDC(std::ptr::null(), hdc);
                if w > 0 && h > 0 {
                    format!("{}x{} @ {}Hz", w, h, if r > 0 { r } else { 60 })
                } else { "Unknown".to_string() }
            }
        }
        #[cfg(not(windows))]
        { "Unknown".to_string() }
    } else if cfg!(target_os = "linux") {
        // Pick the mode line marked with '*' (the currently active mode),
        // which carries both the resolution and the true refresh rate.
        run_stdout(
            "sh",
            &[
                "-c",
                "xrandr --current 2>/dev/null | awk '/\\*/{print $1, $2; exit}' | tr -d '*+'",
            ],
        )
        .and_then(|s| {
            let parts: Vec<&str> = s.split_whitespace().collect();
            if parts.len() >= 2 {
                let hz: f64 = parts[1].parse().unwrap_or(0.0);
                Some(format!("{} @ {:.0}Hz", parts[0], hz))
            } else if !s.trim().is_empty() {
                Some(s.trim().to_string())
            } else {
                None
            }
        })
        .or_else(|| {
            run_stdout(
                "sh",
                &["-c", "xrandr 2>/dev/null | grep ' connected' | head -1 | grep -oP '\\d{3,4}x\\d{3,4}'"],
            )
        })
        .unwrap_or_else(|| "Unknown".to_string())
    } else if cfg!(target_os = "macos") {
        run_stdout("sh", &["-c", "system_profiler SPDisplaysDataType 2>/dev/null | grep Resolution | head -1"])
            .map(|s| s.replace("Resolution:", "").trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    } else {
        "Unknown".to_string()
    }
}

fn get_ports() -> String {
    if cfg!(target_os = "windows") {
        let usb = run_powershell("@(Get-CimInstance Win32_USBControllerDevice).Count").unwrap_or_default();
        let video = run_powershell("@(Get-CimInstance Win32_VideoController).Count").unwrap_or_default();
        let audio = run_powershell("@(Get-CimInstance Win32_SoundDevice).Count").unwrap_or_default();
        let mut parts = Vec::new();
        if let Ok(n) = usb.trim().parse::<u32>() { if n > 0 { parts.push(format!("USB x{}", n)); } }
        if let Ok(n) = video.trim().parse::<u32>() { if n > 0 { parts.push(format!("Video Out x{}", n)); } }
        if let Ok(n) = audio.trim().parse::<u32>() { if n > 0 { parts.push(format!("Audio x{}", n)); } }
        if parts.is_empty() { "Unknown".to_string() } else { parts.join(", ") }
    } else if cfg!(target_os = "linux") {
        let mut parts = Vec::new();
        if let Some(out) = run_stdout("sh", &["-c", "lsusb 2>/dev/null | wc -l"]) {
            if let Ok(n) = out.trim().parse::<u32>() { if n > 0 { parts.push(format!("USB x{}", n)); } }
        }
        if let Some(out) = run_stdout(
            "sh",
            &["-c", "for f in /sys/class/drm/*/status; do grep -q '^connected' \"$f\" 2>/dev/null && echo 1; done | wc -l"],
        ) {
            if let Ok(n) = out.trim().parse::<u32>() { if n > 0 { parts.push(format!("Video Out x{}", n)); } }
        }
        if run_stdout("sh", &["-c", "lspci 2>/dev/null | grep -i audio | head -1"]).is_some() {
            parts.push("Audio".to_string());
        }
        if parts.is_empty() { "Unknown".to_string() } else { parts.join(", ") }
    } else if cfg!(target_os = "macos") {
        let mut parts = Vec::new();
        if let Some(out) = run_stdout("sh", &["-c", "system_profiler SPUSBDataType 2>/dev/null | grep -c 'Product ID'"]) {
            if let Ok(n) = out.trim().parse::<u32>() { if n > 0 { parts.push(format!("USB x{}", n)); } }
        }
        if run_stdout("sh", &["-c", "system_profiler SPAudioDataType 2>/dev/null | head -1"]).is_some() {
            parts.push("Audio".to_string());
        }
        if parts.is_empty() { "Unknown".to_string() } else { parts.join(", ") }
    } else {
        "Unknown".to_string()
    }
}

fn get_wifi() -> String {
    if cfg!(target_os = "windows") {
        let ssid = run_powershell(
            "(netsh wlan show interfaces | Select-String '^\\s+SSID\\s+:' | Select-Object -First 1) -replace '.*:\\s+'",
        )
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
        let signal = run_powershell(
            "(netsh wlan show interfaces | Select-String '^\\s+Signal\\s+:' | Select-Object -First 1) -replace '.*:\\s+'",
        )
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
        match (ssid, signal) {
            (Some(s), Some(sig)) => format!("{} ({})", s, sig),
            (Some(s), None) => s,
            _ => "None".to_string(),
        }
    } else if cfg!(target_os = "linux") {
        let ssid = run_stdout("sh", &["-c", "iwgetid -r 2>/dev/null"]).filter(|s| !s.is_empty());
        let signal = run_stdout(
            "sh",
            &[
                "-c",
                "nmcli -t -f active,signal dev wifi 2>/dev/null | awk -F: '$1==\"yes\"{print $2; exit}'",
            ],
        )
        .filter(|s| !s.is_empty());
        match (ssid, signal) {
            (Some(s), Some(sig)) => format!("{} ({}%)", s, sig),
            (Some(s), None) => s,
            _ => "None".to_string(),
        }
    } else if cfg!(target_os = "macos") {
        run_stdout("sh", &["-c", "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport -I 2>/dev/null | grep ' SSID' | cut -d: -f2"])
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "None".to_string())
    } else {
        "None".to_string()
    }
}

fn get_shell() -> String {
    if cfg!(target_os = "windows") {
        if env::var("PSModulePath").is_ok() {
            return "PowerShell".to_string();
        }
        env::var("COMSPEC")
            .ok()
            .and_then(|p| {
                std::path::Path::new(&p)
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
            })
            .unwrap_or_else(|| "cmd".to_string())
    } else {
        env::var("SHELL")
            .ok()
            .and_then(|p| {
                std::path::Path::new(&p)
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
            })
            .unwrap_or_else(|| "Unknown".to_string())
    }
}

fn get_terminal() -> String {
    if cfg!(target_os = "windows") {
        env::var("WT_SESSION").ok().map(|_| "Windows Terminal".to_string())
            .or_else(|| env::var("TERM_PROGRAM").ok())
            .or_else(|| Some(env::var("TERM").unwrap_or_else(|_| "console".to_string())))
            .unwrap_or_else(|| "console".to_string())
    } else {
        env::var("TERM_PROGRAM").ok()
            .or_else(|| env::var("TERM").ok())
            .unwrap_or_else(|| "Unknown".to_string())
    }
}

fn get_de() -> String {
    if cfg!(target_os = "windows") { "Windows Explorer".to_string() }
    else if cfg!(target_os = "macos") { "Aqua".to_string() }
    else {
        env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .or_else(|_| env::var("GDMSESSION"))
            .unwrap_or_else(|_| "None".to_string())
    }
}

fn get_packages() -> String {
    let mut parts: Vec<String> = Vec::new();

    if cfg!(target_os = "windows") {
        if let Some(out) = run_stdout("winget", &["list"]) {
            let n = out.lines().count().saturating_sub(2);
            if n > 0 { parts.push(format!("{}(winget)", n)); }
        }
        if let Some(out) = run_stdout("npm", &["list", "-g", "--depth=0"]) {
            let n = out.lines().count().saturating_sub(1);
            if n > 0 { parts.push(format!("{}(npm)", n)); }
        }
        if let Some(out) = run_stdout(
            "reg",
            &["query", r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall"],
        ) {
            let n = out.lines().filter(|l| l.starts_with("HKEY_")).count();
            if n > 0 { parts.push(format!("{}(apps)", n)); }
        }
    } else if cfg!(target_os = "linux") {
        for &(cmd, args, label) in &[
            ("dpkg", &["--list"] as &[&str], "dpkg"),
            ("rpm", &["-qa"], "rpm"),
            ("pacman", &["-Q"], "pacman"),
            ("apk", &["info"], "apk"),
            ("flatpak", &["list"], "flatpak"),
            ("snap", &["list"], "snap"),
            ("npm", &["list", "-g", "--depth=0"], "npm"),
        ] {
            if let Some(out) = run_stdout(cmd, args) {
                let n = if label == "npm" { out.lines().count().saturating_sub(1) } else { out.lines().count() };
                if n > 0 { parts.push(format!("{}({})", n, label)); }
            }
        }
    } else if cfg!(target_os = "macos") {
        for path in &["/opt/homebrew/bin/brew", "/usr/local/bin/brew"] {
            if let Some(out) = run_stdout(path, &["list"]) {
                let n = out.lines().count();
                if n > 0 { parts.push(format!("{}(brew)", n)); break; }
            }
        }
        if let Some(out) = run_stdout("npm", &["list", "-g", "--depth=0"]) {
            let n = out.lines().count().saturating_sub(1);
            if n > 0 { parts.push(format!("{}(npm)", n)); }
        }
    }

    if parts.is_empty() { "None".to_string() } else { parts.join(", ") }
}

fn get_battery() -> String {
    if cfg!(target_os = "windows") {
        #[cfg(windows)]
        {
            let mut bat_pct = 0u8;
            let mut bat_ac = 1u8;
            unsafe {
                let mut status: SYSTEM_POWER_STATUS = std::mem::zeroed();
                if GetSystemPowerStatus(&mut status) != 0 {
                    bat_pct = status.battery_life_percent;
                    bat_ac = status.ac_line_status;
                }
            }
            if bat_pct > 0 || bat_ac != 1 {
                let s = match bat_ac { 0 => "Discharging", 1 => "AC", _ => "Unknown" };
                let health = run_powershell(
                    "$d=(Get-CimInstance -Namespace root\\wmi -ClassName BatteryStaticData -ErrorAction SilentlyContinue | Select-Object -First 1).DesignedCapacity; \
                     $f=(Get-CimInstance -Namespace root\\wmi -ClassName BatteryFullChargedCapacity -ErrorAction SilentlyContinue | Select-Object -First 1).FullChargedCapacity; \
                     if($d -and $f -and $d -gt 0){[math]::Round(($f/$d)*100)}",
                )
                .and_then(|h| h.trim().parse::<u32>().ok());
                match health {
                    Some(h) => format!("{}% [{}] - Health {}%", bat_pct, s, h),
                    None => format!("{}% [{}]", bat_pct, s),
                }
            } else {
                "AC (No Battery)".to_string()
            }
        }
        #[cfg(not(windows))]
        { "AC".to_string() }
    } else if cfg!(target_os = "linux") {
        match (
            std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity").ok(),
            std::fs::read_to_string("/sys/class/power_supply/BAT0/status").ok(),
        ) {
            (Some(c), Some(s)) => {
                let design = std::fs::read_to_string("/sys/class/power_supply/BAT0/energy_full_design")
                    .or_else(|_| std::fs::read_to_string("/sys/class/power_supply/BAT0/charge_full_design"))
                    .ok();
                let full = std::fs::read_to_string("/sys/class/power_supply/BAT0/energy_full")
                    .or_else(|_| std::fs::read_to_string("/sys/class/power_supply/BAT0/charge_full"))
                    .ok();
                let health = match (design, full) {
                    (Some(d), Some(f)) => {
                        let dn: f64 = d.trim().parse().unwrap_or(0.0);
                        let fv: f64 = f.trim().parse().unwrap_or(0.0);
                        if dn > 0.0 { Some((fv / dn * 100.0).round() as i64) } else { None }
                    }
                    _ => None,
                };
                match health {
                    Some(h) => format!("{}% [{}] - Health {}%", c.trim(), s.trim(), h),
                    None => format!("{}% [{}]", c.trim(), s.trim()),
                }
            }
            _ => "AC (No Battery)".to_string(),
        }
    } else if cfg!(target_os = "macos") {
        let batt = run_stdout("sh", &["-c", "pmset -g batt 2>/dev/null | tail -1"]).filter(|s| s.contains("%"));
        let cycles = run_stdout(
            "sh",
            &["-c", "ioreg -l 2>/dev/null | grep -i CycleCount | head -1 | grep -oE '[0-9]+'"],
        );
        match (batt, cycles) {
            (Some(b), Some(cy)) => format!("{} - {} cycles", b, cy),
            (Some(b), None) => b,
            _ => "AC (No Battery)".to_string(),
        }
    } else {
        "AC".to_string()
    }
}

fn get_ip() -> String {
    let networks = Networks::new_with_refreshed_list();
    for (name, data) in &networks {
        for ipnet in data.ip_networks() {
            let ip = ipnet.addr;
            if ip.is_ipv4() && !ip.is_loopback() {
                return format!("{} ({})", ip, name);
            }
        }
    }
    "Disconnected".to_string()
}

/// A warm stone-to-gold gradient evoking Angkor Wat's sandstone towers at
/// sunrise, with small platform-flavored variants so the banner still feels
/// native on each OS.
fn logo_gradient() -> [Color; 6] {
    if cfg!(target_os = "windows") {
        [
            Color::BrightGreen, Color::Green, Color::BrightCyan,
            Color::Cyan, Color::BrightGreen, Color::Green,
        ]
    } else if cfg!(target_os = "macos") {
        [
            Color::BrightWhite, Color::White, Color::BrightCyan,
            Color::Cyan, Color::BrightWhite, Color::White,
        ]
    } else {
        [
            Color::BrightYellow, Color::Yellow, Color::BrightRed,
            Color::Red, Color::BrightMagenta, Color::Magenta,
        ]
    }
}

fn accent_color() -> Color {
    logo_gradient()[0]
}

/// Prints a box drawn to fit `lines` exactly, so it never misaligns
/// no matter how long the version string or text gets.
fn print_boxed(lines: &[String], color: Color) {
    let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
    let top = format!("╔{}╗", "═".repeat(width + 2));
    let bottom = format!("╚{}╝", "═".repeat(width + 2));
    println!("{}", top.color(color));
    for line in lines {
        let pad = width - line.chars().count();
        println!("{}", format!("║ {}{} ║", line, " ".repeat(pad)).color(color));
    }
    println!("{}", bottom.color(color));
}

fn print_logo_banner() {
    let gradient = logo_gradient();
    // Each row spells one letter of ANGKOR; a gentle top-to-bottom gradient
    // gives the block letters more depth than a single flat color.
    let logo_lines = [
        "  █████╗  ███╗   ██╗  ██████╗  ██╗  ██╗  ██████╗  ██████╗ ",
        " ██╔══██╗ ████╗  ██║ ██╔════╝ ██║ ██╔╝ ██╔══██╗ ██╔══██╗",
        " ███████║ ██╔██╗ ██║ ██║  ███╗ █████╔╝  ██║  ██║ ██████╔╝",
        " ██╔══██║ ██║╚██╗██║ ██║   ██║ ██╔═██╗  ██║  ██║ ██╔══██╗",
        " ██║  ██║ ██║ ╚████║ ╚██████╔╝ ██║  ██╗ ╚██████╔╝ ██║  ██║",
        " ╚═╝  ╚═╝ ╚═╝  ╚═══╝  ╚═════╝  ╚═╝  ╚═╝  ╚═════╝  ╚═╝  ╚═╝",
    ];

    println!();
    for (i, line) in logo_lines.iter().enumerate() {
        println!("{}", line.color(gradient[i % gradient.len()]));
    }
    println!();

    let color = accent_color();
    let banner = format!(
        "AngkorFetch v{}  •  Fast Cross-Platform System Fetch  •  by AMSDev",
        env!("CARGO_PKG_VERSION")
    );
    print_boxed(&[banner], color);
}

fn print_hardware_info(sys: &System) {
    print_logo_banner();
    println!();

    let entries: Vec<(&str, String)> = vec![
        ("Motherboard", get_motherboard()),
        ("BIOS", get_bios()),
        ("Serial", get_serial()),
        ("CPU", get_cpu_info(sys)),
        ("GPU", get_gpu()),
        ("Memory", get_memory(sys)),
        ("RAM", get_ram_details()),
        ("Disk", get_disk()),
        ("Disk Model", get_disk_model()),
        ("Disk Type", get_disk_support()),
        ("Display", get_display()),
        ("Ports", get_ports()),
        ("WiFi", get_wifi()),
        ("Battery", get_battery()),
    ];

    let max_label = entries.iter().map(|(l, _)| l.len()).max().unwrap_or(0);
    let dot_colors = [
        Color::BrightCyan, Color::BrightMagenta, Color::BrightBlue, Color::BrightYellow,
        Color::BrightGreen, Color::BrightRed, Color::Cyan, Color::Magenta, Color::Blue,
        Color::Yellow, Color::Green, Color::Red, Color::BrightWhite,
    ];

    for (i, (label, value)) in entries.iter().enumerate() {
        let dot = "●".color(dot_colors[i % dot_colors.len()]);
        let padded = format!("{:width$}", label, width = max_label);
        let sep = " ❯".bright_black();
        println!(" {} {} {} {}", dot, padded.cyan().bold(), sep, value.white());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = args.first().map(|s| s.as_str()).unwrap_or("angkorfetch");

    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        print_help(prog);
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v") {
        print_logo_banner();
        return;
    }

    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything()),
    );
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());
    sys.refresh_memory_specifics(MemoryRefreshKind::everything());
    sleep(Duration::from_millis(200));
    sys.refresh_cpu_specifics(CpuRefreshKind::everything());

    if args.iter().any(|a| a == "--hinfo" || a == "--hard") {
        print_hardware_info(&sys);
        return;
    }

    let os = get_os_name(&get_arch());
    let hostname = get_host();
    let model = get_model();
    let uptime = get_uptime();
    let cpu_info = get_cpu_info(&sys);
    let cpu_usage = format!("{:.1}%", sys.global_cpu_usage());
    let gpu = get_gpu();
    let gpu_usage = get_gpu_usage();
    let memory = get_memory(&sys);
    let disk = get_disk();
    let display = get_display();
    let shell = get_shell();
    let terminal = get_terminal();
    let de = get_de();
    let packages = get_packages();
    let battery = get_battery();
    let ip = get_ip();

    print_logo_banner();
    println!();

    let entries: Vec<(&str, String)> = vec![
        ("OS", os),
        ("Host", hostname),
        ("Model", model),
        ("Uptime", uptime),
        ("CPU", cpu_info),
        ("CPU Usage", cpu_usage),
        ("GPU", gpu),
        ("GPU Usage", gpu_usage),
        ("Memory", memory),
        ("Disk", disk),
        ("Display", display),
        ("Shell", shell),
        ("Terminal", terminal),
        ("DE", de),
        ("Packages", packages),
        ("Battery", battery),
        ("Local IP", ip),
    ];

    let max_label = entries.iter().map(|(l, _)| l.len()).max().unwrap_or(0);
    let dot_colors = [
        Color::BrightCyan, Color::BrightMagenta, Color::BrightBlue, Color::BrightYellow,
        Color::BrightGreen, Color::BrightRed, Color::Cyan, Color::Magenta, Color::Blue,
        Color::Yellow, Color::Green, Color::Red, Color::BrightWhite,
    ];

    for (i, (label, value)) in entries.iter().enumerate() {
        let dot = "●".color(dot_colors[i % dot_colors.len()]);
        let padded = format!("{:width$}", label, width = max_label);
        let sep = " ❯".bright_black();
        println!(" {} {} {} {}", dot, padded.cyan().bold(), sep, value.white());
    }
    println!();
}
