use std::ffi::CString;

use crate::marshal;

use super::{MhyContext, MhyModule, ModuleType};
use anyhow::Result;
use ilhook::x64::Registers;

const MHYRSA_PERFORM_CRYPTO_ACTION: usize = 0x9DD5C8;
const KEY_SIGN_CHECK: usize = 0x9DF4BC;
const SDK_UTIL_RSA_ENCRYPT: usize = 0xF7A73C0;

const KEY_SIZE: usize = 268;
static SERVER_PUBLIC_KEY: &[u8] = include_bytes!("../../server_public_key.bin");
static SDK_PUBLIC_KEY: &str = include_str!("../../sdk_public_key.xml");

pub struct Security;

impl MhyModule for MhyContext<Security> {
    unsafe fn init(&mut self) -> Result<()> {
        self.interceptor.attach(
            self.assembly_base + MHYRSA_PERFORM_CRYPTO_ACTION,
            on_mhy_rsa,
        )?;

        self.interceptor
            .attach(self.assembly_base + KEY_SIGN_CHECK, after_key_sign_check)?;

        self.interceptor.attach(
            self.assembly_base + SDK_UTIL_RSA_ENCRYPT,
            on_sdk_util_rsa_encrypt,
        )
    }

    unsafe fn de_init(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_module_type(&self) -> super::ModuleType {
        ModuleType::Security
    }
}

unsafe extern "win64" fn after_key_sign_check(reg: *mut Registers, _: usize) {
    println!("key sign check!");
    (*reg).rax = 1
}

unsafe extern "win64" fn on_mhy_rsa(reg: *mut Registers, _: usize) {
    println!("key: {:X}", *((*reg).rdx as *const u64));
    println!("len: {:X}", (*reg).r8);

    if (*reg).r8 as usize == KEY_SIZE {
        println!("[*] key replaced");

        std::ptr::copy_nonoverlapping(
            SERVER_PUBLIC_KEY.as_ptr(),
            (*reg).rdx as *mut u8,
            SERVER_PUBLIC_KEY.len(),
        );
    }
}

unsafe extern "win64" fn on_sdk_util_rsa_encrypt(reg: *mut Registers, _: usize) {
    println!("[*] SDK RSA: key replaced");
    (*reg).rcx =
        marshal::ptr_to_string_ansi(CString::new(SDK_PUBLIC_KEY).unwrap().as_c_str()) as u64;
}
