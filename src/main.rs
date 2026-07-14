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
    println!("  -hard, --hard   Show only hardware information");
    println!("  -h, --help      Show this help message and exit");
    println!("  -v, --version   Show version information and exit");
    println!();
    println!("Without options, shows software information only.");
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

fn get_memory(sys: &System) -> String {
    let total = sys.total_memory() as f64 / 1_073_741_824.0;
    let used = sys.used_memory() as f64 / 1_073_741_824.0;
    let pct = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };
    format!("{:.1} GiB / {:.1} GiB ({:.0}%)", used, total, pct)
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

fn get_display() -> String {
    if cfg!(target_os = "linux") {
        run_stdout("sh", &["-c", "xrandr 2>/dev/null | grep ' connected' | head -1 | grep -oP '\\d{3,4}x\\d{3,4}'"])
            .map(|s| format!("{} @ 60Hz", s))
            .unwrap_or_else(|| "Unknown".to_string())
    } else if cfg!(target_os = "macos") {
        run_stdout("sh", &["-c", "system_profiler SPDisplaysDataType 2>/dev/null | grep Resolution | head -1"])
            .map(|s| s.replace("Resolution:", "").trim().to_string())
            .unwrap_or_else(|| "Unknown".to_string())
    } else {
        "Unknown".to_string()
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
    if cfg!(target_os = "windows") {
        if let Some(out) = run_stdout(
            "reg",
            &["query", r"HKLM\Software\Microsoft\Windows\CurrentVersion\Uninstall"],
        ) {
            let n = out.lines().filter(|l| l.starts_with("HKEY_")).count();
            if n > 0 { return format!("{} (apps)", n); }
        }
        "None".to_string()
    } else if cfg!(target_os = "linux") {
        for &(cmd, args) in &[
            ("dpkg", &["--list"] as &[&str]),
            ("rpm", &["-qa"]),
            ("pacman", &["-Q"]),
            ("flatpak", &["list"]),
            ("snap", &["list"]),
        ] {
            if let Some(lines) = run_stdout(cmd, args) {
                let n = lines.lines().count();
                if n > 0 { return format!("{} ({})", n, cmd); }
            }
        }
        "None".to_string()
    } else if cfg!(target_os = "macos") {
        for path in &["/opt/homebrew/bin/brew", "/usr/local/bin/brew"] {
            if let Some(lines) = run_stdout(path, &["list"]) {
                let n = lines.lines().count();
                if n > 0 { return format!("{} (brew)", n); }
            }
        }
        "None".to_string()
    } else {
        "None".to_string()
    }
}

fn get_battery() -> String {
    if cfg!(target_os = "linux") {
        match (
            std::fs::read_to_string("/sys/class/power_supply/BAT0/capacity").ok(),
            std::fs::read_to_string("/sys/class/power_supply/BAT0/status").ok(),
        ) {
            (Some(c), Some(s)) => format!("{}% [{}]", c.trim(), s.trim()),
            _ => "AC".to_string(),
        }
    } else if cfg!(target_os = "macos") {
        run_stdout("sh", &["-c", "pmset -g batt 2>/dev/null | head -1"])
            .filter(|s| s.contains("%"))
            .unwrap_or_else(|| "AC".to_string())
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

fn logo_color() -> Color {
    if cfg!(target_os = "windows") { Color::BrightGreen }
    else if cfg!(target_os = "macos") { Color::BrightWhite }
    else { Color::BrightRed }
}

fn print_logo_banner() {
    let color = logo_color();
    let logo = r"
  █████╗  ███╗   ██╗  ██████╗  ██╗  ██╗  ██████╗  ██████╗ 
 ██╔══██╗ ████╗  ██║ ██╔════╝  ██║ ██╔╝  ██╔══██╗ ██╔══██╗
 ███████║ ██╔██╗ ██║ ██║  ███╗ █████╔╝   ██║  ██║ ██████╔╝
 ██╔══██║ ██║╚██╗██║ ██║   ██║ ██╔═██╗   ██║  ██║ ██╔══██╗
 ██║  ██║ ██║ ╚████║ ╚██████╔╝ ██║  ██╗  ██████╔╝ ██║  ██║
 ╚═╝  ╚═╝ ╚═╝  ╚═══╝  ╚═════╝  ╚═╝  ╚═╝  ╚═════╝  ╚═╝  ╚═╝
╔═════════════════════════════════════════════════════════╗
║       AngkorFetch v0.2 - Fast System Fetch Tool         ║
║       Built with OpenCode + DeepSeekV4 by AMSDev        ║
╚═════════════════════════════════════════════════════════╝";
    for line in logo.trim_end_matches('\n').lines() {
        println!("{}", line.color(color));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog = args.first().map(|s| s.as_str()).unwrap_or("angkorfetch");

    if args.iter().any(|a| a == "--version" || a == "-v") {
        println!("angkorfetch v{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.iter().any(|a| a == "--help" || a == "-h" || a == "/?") {
        print_help(prog);
        return;
    }

    let hardware_mode = args.iter().any(|a| a == "--hard" || a == "-hard");

    // Phase 1: Instant data from env vars (zero subprocess)
    let arch = get_arch();
    let hostname = get_host();
    let terminal = get_terminal();
    let de = get_de();

    let os = get_os_name(&arch);
    let uptime = get_uptime();
    let packages = get_packages();

    // Software mode: fast path — no sysinfo, no CPU cooldown
    let (cpu_info, cpu_usage, memory, disk, model, gpu, display, battery, ip) = if hardware_mode {
        // Hardware mode: full sysinfo + CPU cooldown + hardware data
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        sys.refresh_cpu_specifics(CpuRefreshKind::everything());
        sys.refresh_memory_specifics(MemoryRefreshKind::everything());
        sleep(Duration::from_millis(200));
        sys.refresh_cpu_specifics(CpuRefreshKind::everything());

        let model = if cfg!(target_os = "windows") {
            let manu = reg_val(r"HKLM\HARDWARE\DESCRIPTION\System\BIOS", "SystemManufacturer").unwrap_or_default();
            let prod = reg_val(r"HKLM\HARDWARE\DESCRIPTION\System\BIOS", "SystemProductName").unwrap_or_default();
            if manu.is_empty() && prod.is_empty() { String::new() } else { format!("{} {}", manu, prod).trim().to_string() }
        } else if cfg!(target_os = "linux") {
            std::fs::read_to_string("/sys/class/dmi/id/product_name").ok().map(|s| s.trim().to_string()).unwrap_or_default()
        } else if cfg!(target_os = "macos") {
            run_stdout("sysctl", &["-n", "hw.model"]).unwrap_or_default()
        } else { String::new() };

        let gpu = get_gpu();

        let (display, battery) = if cfg!(target_os = "windows") {
            #[cfg(windows)]
            {
                let (dw, dh, dr) = unsafe {
                    let hdc = GetDC(std::ptr::null());
                    if hdc.is_null() {
                        ("Unknown".to_string(), String::new(), String::new())
                    } else {
                        let w = GetDeviceCaps(hdc, 8);
                        let h = GetDeviceCaps(hdc, 10);
                        let r = GetDeviceCaps(hdc, 116);
                        ReleaseDC(std::ptr::null(), hdc);
                        (w.to_string(), h.to_string(), if r > 0 { r.to_string() } else { "60".to_string() })
                    }
                };
                let d = if dh.is_empty() { "Unknown".to_string() } else { format!("{}x{} @ {}Hz", dw, dh, dr) };
                let mut bat_pct = 0u8;
                let mut bat_ac = 1u8;
                unsafe {
                    let mut status: SYSTEM_POWER_STATUS = std::mem::zeroed();
                    if GetSystemPowerStatus(&mut status) != 0 {
                        bat_pct = status.battery_life_percent;
                        bat_ac = status.ac_line_status;
                    }
                }
                let b = if bat_pct > 0 || bat_ac != 1 {
                    let s = match bat_ac { 0 => "Discharging", 1 => "AC", _ => "Unknown" };
                    format!("{}% [{}]", bat_pct, s)
                } else { "AC".to_string() };
                (d, b)
            }
            #[cfg(not(windows))]
            { (get_display(), get_battery()) }
        } else {
            (get_display(), get_battery())
        };

        let cpu_info = get_cpu_info(&sys);
        let cpu_usage = format!("{:.1}%", sys.global_cpu_usage());
        let memory = get_memory(&sys);
        let disk = get_disk();
        let ip = get_ip();
        (cpu_info, cpu_usage, memory, disk, model, gpu, display, battery, ip)
    } else {
        // Software mode: skip sysinfo entirely, just get IP
        let ip = get_ip();
        (String::new(), String::new(), String::new(), String::new(),
         String::new(), String::new(), String::new(), String::new(), ip)
    };

    // Phase 5: Output
    print_logo_banner();
    println!();

    let mut entries: Vec<(&str, String)> = Vec::new();

    if !hardware_mode {
        entries.push(("OS", os));
        entries.push(("Host", hostname));
        entries.push(("Uptime", uptime));
        entries.push(("Terminal", terminal));
        entries.push(("DE", de));
        entries.push(("Packages", packages));
        entries.push(("IP", ip));
    }

    if hardware_mode {
        entries.push(("Model", model));
        entries.push(("CPU", cpu_info));
        entries.push(("CPU Usage", cpu_usage));
        entries.push(("GPU", gpu));
        entries.push(("GPU Usage", "N/A".to_string()));
        entries.push(("Memory", memory));
        entries.push(("Disk", disk));
        entries.push(("Display", display));
        entries.push(("Battery", battery));
    }

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
