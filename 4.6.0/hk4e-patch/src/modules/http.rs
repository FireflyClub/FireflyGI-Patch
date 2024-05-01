use std::ffi::CString;

use super::{MhyContext, MhyModule, ModuleType};
use crate::marshal;
use anyhow::Result;
use ilhook::x64::Registers;

use std::fs;

fn read_server_txt() -> Result<String, std::io::Error> {
    fs::read_to_string("server.txt")
}

fn write_default_url_to_server_txt() -> Result<(), std::io::Error> {
    fs::write("server.txt", "http://127.0.0.1:443")
}

const UNITY_WEB_REQUEST_SET_URL: usize = 0x0D9442D0;

pub struct Http;

impl MhyModule for MhyContext<Http> {
    unsafe fn init(&mut self) -> Result<()> {
        self.interceptor.attach(
            self.assembly_base + UNITY_WEB_REQUEST_SET_URL,
            on_uwr_set_url,
        )
    }

    unsafe fn de_init(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_module_type(&self) -> super::ModuleType {
        ModuleType::Http
    }
}

unsafe extern "win64" fn on_uwr_set_url(reg: *mut Registers, _: usize) {
    let str_length = *((*reg).rdx.wrapping_add(16) as *const u32);
    let str_ptr = (*reg).rdx.wrapping_add(20) as *const u8;

    let slice = std::slice::from_raw_parts(str_ptr, (str_length * 2) as usize);
    let url = String::from_utf16le(slice).unwrap();

    let mut new_url = match read_server_txt() {
        Ok(content) => content,
        Err(_) => {
            if let Err(err) = write_default_url_to_server_txt() {
                println!("Failed to create server.txt: {}", err);
            }
            println!("Cound not find server.txt, created it automatically!");
            String::from("http://127.0.0.1:443")
        },
    };

    url.split('/').skip(3).for_each(|s| {
        new_url.push_str("/");
        new_url.push_str(s);
    });

    println!("Redirect: {url} -> {new_url}");
    (*reg).rdx =
        marshal::ptr_to_string_ansi(CString::new(new_url.as_str()).unwrap().as_c_str()) as u64;
}
