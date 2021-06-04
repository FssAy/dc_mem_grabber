use sysinfo::{SystemExt, ProcessExt, System};
use crate::winapi::DWORD;


pub fn get_pids() -> Vec<DWORD> {
    let mut res_pids = Vec::<DWORD>::new();
    let mut sys = System::new();
    sys.refresh_processes();

    let discords: Vec<String> = vec![
        "Discord.exe".to_string(),
        "DiscordCanary.exe".to_string(),
        "DiscordPTB.exe".to_string()
    ];

    for (pid, proc) in sys.get_processes() {
        if discords.contains(&proc.name().to_string()) {
            res_pids.push(*pid as DWORD);
        }
    }

    res_pids
}
