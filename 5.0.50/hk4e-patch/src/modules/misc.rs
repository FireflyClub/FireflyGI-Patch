use std::ffi::CStr;

use super::{MhyContext, MhyModule, ModuleType};
use anyhow::Result;
use ilhook::x64::Registers;
use windows::{core::PCSTR, Win32::System::LibraryLoader::GetModuleHandleA};

pub struct Misc;

const DYNAMIC_IMPORT: usize = 0x3F5240;
const SET_CUSTOM_PROPERTY_FLOAT: usize = 0x11E1880;

impl MhyModule for MhyContext<Misc> {
    unsafe fn init(&mut self) -> Result<()> {
        // CNCBWin5.0.50 sound fix
        self.interceptor
            .attach(self.assembly_base + DYNAMIC_IMPORT, on_dynamic_import)?;

        // Dither
        self.interceptor.replace(
            self.assembly_base + SET_CUSTOM_PROPERTY_FLOAT,
            set_custom_property_float_replacement,
        )
    }

    unsafe fn de_init(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_module_type(&self) -> super::ModuleType {
        ModuleType::Misc
    }
}

unsafe extern "win64" fn on_dynamic_import(reg: *mut Registers, _: usize) {
    let symbol_name_ptr = *((*reg).rcx.wrapping_add(16) as *const usize);
    let symbol_name = CStr::from_ptr(symbol_name_ptr as *const i8);

    // Hoyo forgot to package updated sound library and that's the missing export
    if symbol_name.to_string_lossy() == "GetMusicSyncCallbackInfoPlayingSeq" {
        let base = GetModuleHandleA(PCSTR::null()).unwrap().0 as usize;
        *((*reg).rcx.wrapping_add(16) as *mut usize) = base + 0x2F6CD04;
    }
}

unsafe extern "win64" fn set_custom_property_float_replacement(
    _: *mut Registers,
    _: usize,
    _: usize,
) -> usize {
    0
}
