use std::mem;

use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};

use crate::obf;

pub fn defaults() -> Vec<String> {
    obf::default_targets()
}

pub fn find_running(targets: &[&str]) -> Vec<(String, u32)> {
    let mut r = Vec::new();
    let snap = match unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) } {
        Ok(h) => h,
        Err(_) => return r,
    };
    let mut e = PROCESSENTRY32W {
        dwSize: mem::size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };
    if unsafe { Process32FirstW(snap, &mut e) }.is_err() {
        unsafe { let _ = CloseHandle(snap); }
        return r;
    }
    loop {
        let name = String::from_utf16_lossy(
            &e.szExeFile[..e.szExeFile.iter().position(|&c| c == 0).unwrap_or(e.szExeFile.len())],
        );
        for &t in targets {
            if name.eq_ignore_ascii_case(t) {
                r.push((name.clone(), e.th32ProcessID));
            }
        }
        if unsafe { Process32NextW(snap, &mut e) }.is_err() { break; }
    }
    unsafe { let _ = CloseHandle(snap); }
    r
}
