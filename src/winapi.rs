use std::iter::once;
use windows::{
    core::{PCWSTR, PWSTR},
    Win32::System::Services::{
        RegisterServiceCtrlHandlerW, SetServiceStatus, StartServiceCtrlDispatcherW,
        LPHANDLER_FUNCTION, LPSERVICE_MAIN_FUNCTIONW, SERVICE_ACCEPT_SHUTDOWN, SERVICE_ACCEPT_STOP,
        SERVICE_STATUS, SERVICE_STATUS_CURRENT_STATE, SERVICE_STATUS_HANDLE, SERVICE_TABLE_ENTRYW,
        SERVICE_WIN32_OWN_PROCESS,
    },
};

fn wchar_t(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(once(0)).collect()
}

pub fn start_service_ctrl_dispatcher_w(
    lpservicename: &str,
    lpserviceproc: LPSERVICE_MAIN_FUNCTIONW,
) {
    let mut lpservicename: Vec<u16> = wchar_t(lpservicename);
    let lpservicestarttable: &SERVICE_TABLE_ENTRYW = &SERVICE_TABLE_ENTRYW {
        lpServiceName: PWSTR::from_raw(lpservicename.as_mut_ptr()),
        lpServiceProc: lpserviceproc,
    };
    unsafe { StartServiceCtrlDispatcherW(lpservicestarttable) }.unwrap()
}

pub fn register_service_ctrl_handler_w(
    lpservicename: &str,
    lphandlerproc: LPHANDLER_FUNCTION,
) -> windows::core::Result<SERVICE_STATUS_HANDLE> {
    let lpservicename: Vec<u16> = wchar_t(lpservicename);
    let lpservicename: PCWSTR = PCWSTR::from_raw(lpservicename.as_ptr());
    unsafe { RegisterServiceCtrlHandlerW(lpservicename, lphandlerproc) }
}

pub fn set_service_status<P0>(
    hservicestatus: P0,
    dwcurrentstate: SERVICE_STATUS_CURRENT_STATE,
) -> windows::core::Result<()>
where
    P0: windows::core::Param<SERVICE_STATUS_HANDLE>,
{
    let mut lpservicestatus: SERVICE_STATUS = SERVICE_STATUS {
        dwServiceType: SERVICE_WIN32_OWN_PROCESS,
        dwCurrentState: dwcurrentstate,
        dwControlsAccepted: SERVICE_ACCEPT_STOP | SERVICE_ACCEPT_SHUTDOWN,
        dwWin32ExitCode: 0,
        dwServiceSpecificExitCode: 0,
        dwCheckPoint: 0,
        dwWaitHint: 0,
    };
    unsafe { SetServiceStatus(hservicestatus, &mut lpservicestatus) }
}
