use std::ffi::CStr;

use windows::{core::PCSTR, Win32::System::LibraryLoader::GetModuleHandleA};

const PTR_TO_STRING_ANSI: usize = 0xF33F640;
type MarshalPtrToStringAnsi = unsafe extern "fastcall" fn(*const u8) -> *const u8;

pub unsafe fn ptr_to_string_ansi(content: &CStr) -> *const u8 {
    let func = std::mem::transmute::<usize, MarshalPtrToStringAnsi>(base() + PTR_TO_STRING_ANSI);
    func(content.to_bytes_with_nul().as_ptr())
}

unsafe fn base() -> usize {
    GetModuleHandleA(PCSTR::null()).unwrap().0 as usize
}
