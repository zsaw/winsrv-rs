mod winapi;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    OnceLock,
};
use winapi::*;
use windows::{
    core::PWSTR,
    Win32::System::Services::{
        LPHANDLER_FUNCTION, LPSERVICE_MAIN_FUNCTIONW, SERVICE_RUNNING,
        SERVICE_STATUS_CURRENT_STATE, SERVICE_STOPPED,
    },
};

pub use windows::Win32::System::Services::SERVICE_CONTROL_CONTINUE;
pub use windows::Win32::System::Services::SERVICE_CONTROL_INTERROGATE;
pub use windows::Win32::System::Services::SERVICE_CONTROL_NETBINDADD;
pub use windows::Win32::System::Services::SERVICE_CONTROL_NETBINDDISABLE;
pub use windows::Win32::System::Services::SERVICE_CONTROL_NETBINDENABLE;
pub use windows::Win32::System::Services::SERVICE_CONTROL_NETBINDREMOVE;
pub use windows::Win32::System::Services::SERVICE_CONTROL_PARAMCHANGE;
pub use windows::Win32::System::Services::SERVICE_CONTROL_PAUSE;
pub use windows::Win32::System::Services::SERVICE_CONTROL_PRESHUTDOWN;
pub use windows::Win32::System::Services::SERVICE_CONTROL_SHUTDOWN;
pub use windows::Win32::System::Services::SERVICE_CONTROL_STOP;

static SERVICE_NAME: OnceLock<String> = OnceLock::new();
static CONTROL_SIGNAL: OnceLock<Sender<u32>> = OnceLock::new();
static SERVICE_MAIN_FUNC: OnceLock<fn(Receiver<u32>)> = OnceLock::new();

unsafe extern "system" fn service_control_handler(dwcontrol: u32) {
    CONTROL_SIGNAL.get().unwrap().send(dwcontrol).unwrap();
}

unsafe extern "system" fn service_main(_dwnumservicesargs: u32, _lpserviceargvectors: *mut PWSTR) {
    let (sender, receiver) = channel::<u32>();
    CONTROL_SIGNAL.set(sender).unwrap();

    let lpservicename: &str = SERVICE_NAME.get().unwrap();
    let lphandlerproc: LPHANDLER_FUNCTION = Some(service_control_handler);
    let hservicestatus = register_service_ctrl_handler_w(lpservicename, lphandlerproc).unwrap();

    let dwcurrentstate: SERVICE_STATUS_CURRENT_STATE = SERVICE_RUNNING;
    set_service_status(hservicestatus, dwcurrentstate).unwrap();

    let srnmain: &fn(Receiver<u32>) = SERVICE_MAIN_FUNC.get().unwrap();
    srnmain(receiver);

    let dwcurrentstate: SERVICE_STATUS_CURRENT_STATE = SERVICE_STOPPED;
    set_service_status(hservicestatus, dwcurrentstate).unwrap();
}

pub fn run_service(srvname: &str, srnmain: fn(Receiver<u32>)) {
    SERVICE_NAME.set(String::from(srvname)).unwrap();
    SERVICE_MAIN_FUNC.set(srnmain).unwrap();

    let lpservicename: &str = srvname;
    let lpserviceproc: LPSERVICE_MAIN_FUNCTIONW = Some(service_main);
    start_service_ctrl_dispatcher_w(lpservicename, lpserviceproc);
}
