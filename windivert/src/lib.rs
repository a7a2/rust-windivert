extern crate winapi;
extern crate kernel32;
extern crate windivert_sys as ffi;

use std::io::Result;
use std::mem::uninitialized;
use std::ffi::CString;

macro_rules! try_win {
    ($expr:expr) => (if $expr == winapi::FALSE { return Err(std::io::Error::last_os_error()) })
}

pub struct Handle {
    handle: winapi::HANDLE,
}

impl Handle {
    pub fn new(filter: &str,
               layer: ffi::WINDIVERT_LAYER,
               priority: i16,
               flags: u64)
               -> Result<Handle> {
        let c_filter = CString::new(filter).unwrap().as_ptr();
        unsafe {
            let handle = ffi::WinDivertOpen(c_filter, layer, priority, flags);
            if handle != winapi::INVALID_HANDLE_VALUE {
                Ok(Handle { handle: handle })
            } else {
                Err(std::io::Error::last_os_error())
            }
        }
    }
    pub fn recv(&self, packet: &mut [u8]) -> Result<(ffi::WINDIVERT_ADDRESS, u32)> {
        unsafe {
            let mut read_len: u32 = uninitialized();
            let mut addr = uninitialized();
            try_win!(ffi::WinDivertRecv(self.handle,
                                        packet.as_mut_ptr() as winapi::PVOID,
                                        packet.len() as u32,
                                        &mut addr,
                                        &mut read_len));
            Ok((addr, read_len))
        }
    }
    pub fn send(&self, packet: &[u8], addr: &ffi::WINDIVERT_ADDRESS) -> Result<u32> {
        unsafe {
            let mut write_len: u32 = uninitialized();
            try_win!(ffi::WinDivertSend(self.handle,
                                        packet.as_ptr() as winapi::PVOID,
                                        packet.len() as u32,
                                        addr,
                                        &mut write_len));
            Ok(write_len)
        }
    }
    pub fn set_param(&self, param: ffi::WINDIVERT_PARAM, value: u64) -> Result<()> {
        unsafe {
            try_win!(ffi::WinDivertSetParam(self.handle, param, value));
            Ok(())
        }
    }
    pub fn get_param(&self, param: ffi::WINDIVERT_PARAM) -> Result<u64> {
        unsafe {
            let mut value: u64 = uninitialized();
            try_win!(ffi::WinDivertGetParam(self.handle, param, &mut value));
            Ok(value)
        }
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            ffi::WinDivertClose(self.handle);
        }
    }
}
