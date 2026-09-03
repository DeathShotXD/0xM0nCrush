use windows::core::PCWSTR;
use windows::Win32::System::Services::{
    CreateServiceW, DeleteService, OpenSCManagerW, OpenServiceW, StartServiceW, SC_HANDLE,
    SC_MANAGER_CREATE_SERVICE, SERVICE_ALL_ACCESS, SERVICE_KERNEL_DRIVER, SERVICE_DEMAND_START,
    SERVICE_ERROR_NORMAL,
};
use windows::Win32::System::Services::{
    CloseServiceHandle, ControlService, SERVICE_CONTROL_STOP, SERVICE_STATUS,
};

pub struct DriverService {
    scm: SC_HANDLE,
    svc: SC_HANDLE,
}

impl DriverService {
    pub fn install(name: &str, driver_path: &str) -> Result<Self, String> {
        let scm = unsafe { OpenSCManagerW(None, None, SC_MANAGER_CREATE_SERVICE) }
            .map_err(|e| format!("OpenSCManager: {e}"))?;

        let name_w: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
        let disp_w: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
        let path_w: Vec<u16> = driver_path.encode_utf16().chain(Some(0)).collect();

        let existing = unsafe { OpenServiceW(scm, PCWSTR(name_w.as_ptr()), SERVICE_ALL_ACCESS) };
        if existing.is_ok() {
            let svc = existing.unwrap();
            return Ok(Self { scm, svc });
        }

        let svc = unsafe {
            CreateServiceW(
                scm,
                PCWSTR(name_w.as_ptr()),
                PCWSTR(disp_w.as_ptr()),
                SERVICE_ALL_ACCESS,
                SERVICE_KERNEL_DRIVER,
                SERVICE_DEMAND_START,
                SERVICE_ERROR_NORMAL,
                PCWSTR(path_w.as_ptr()),
                None,
                None,
                None,
                None,
                None,
            )
        }
        .map_err(|e| {
            let _ = unsafe { CloseServiceHandle(scm) };
            format!("CreateService: {e}")
        })?;

        Ok(Self { scm, svc })
    }

    pub fn start(&self) -> Result<(), String> {
        unsafe { StartServiceW(self.svc, None) }.map_err(|e| format!("StartService: {e}"))
    }
}

impl Drop for DriverService {
    fn drop(&mut self) {
        // Stop + delete the service so no driver artifact survives the run.
        let _ = unsafe { ControlService(self.svc, SERVICE_CONTROL_STOP, &mut SERVICE_STATUS::default()) };
        let _ = unsafe { DeleteService(self.svc) };
        unsafe {
            let _ = CloseServiceHandle(self.svc);
            let _ = CloseServiceHandle(self.scm);
        }
    }
}
