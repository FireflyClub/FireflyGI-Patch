use std::ffi::CStr;

use super::{MhyContext, MhyModule, ModuleType};
use anyhow::Result;
use ilhook::x64::Registers;
use windows::{
    core::s,
    Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
};

pub struct CcpBlocker;

impl MhyModule for MhyContext<CcpBlocker> {
    unsafe fn init(&mut self) -> Result<()> {
        let winsock2 = GetModuleHandleA(s!("Ws2_32.dll")).unwrap();
        let getaddrinfo = GetProcAddress(winsock2, s!("getaddrinfo")).unwrap();

        self.interceptor
            .attach(getaddrinfo as usize, on_getaddrinfo)
    }

    unsafe fn de_init(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_module_type(&self) -> super::ModuleType {
        ModuleType::CcpBlocker
    }
}

unsafe extern "win64" fn on_getaddrinfo(reg: *mut Registers, _: usize) {
    let host_ptr = (*reg).rcx as *const i8;
    let host = CStr::from_ptr(host_ptr);

    if host.to_string_lossy() == "dispatchcnglobal.yuanshen.com" {
        std::ptr::copy_nonoverlapping(c"0.0.0.0".as_ptr(), (*reg).rcx as *mut i8, 9);
    }
}
