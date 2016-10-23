extern crate winapi;
extern crate kernel32;
extern crate windivert_sys as ffi;

use std::mem::uninitialized;
use std::ffi::CString;
use kernel32::GetLastError;

pub struct Handle {
    handle: winapi::HANDLE,
}

impl Handle {
    pub fn new(filter: &str,
               layer: ffi::WINDIVERT_LAYER,
               priority: i16,
               flags: u64)
               -> Result<Handle, winapi::DWORD> {
        let c_filter = CString::new(filter).unwrap().as_ptr();
        unsafe {
            let handle = ffi::WinDivertOpen(c_filter, layer, priority, flags);
            if handle != winapi::INVALID_HANDLE_VALUE {
                Ok(Handle { handle: handle })
            } else {
                Err(GetLastError())
            }
        }
    }
    pub fn recv(&self, packet: &mut [u8]) -> Result<(ffi::WINDIVERT_ADDRESS, u32), winapi::DWORD> {
        unsafe {
            let mut read_len: u32 = uninitialized();
            let mut addr = uninitialized();
            if ffi::WinDivertRecv(self.handle,
                                  packet.as_mut_ptr() as winapi::PVOID,
                                  packet.len() as u32,
                                  &mut addr,
                                  &mut read_len) == winapi::TRUE {
                Ok((addr, read_len))
            } else {
                Err(GetLastError())
            }
        }
    }
    pub fn send(&self, packet: &[u8], addr: &ffi::WINDIVERT_ADDRESS) -> Result<u32, winapi::DWORD> {
        unsafe {
            let mut write_len: u32 = uninitialized();
            if ffi::WinDivertSend(self.handle,
                                  packet.as_ptr() as winapi::PVOID,
                                  packet.len() as u32,
                                  addr,
                                  &mut write_len) == winapi::TRUE {
                Ok(write_len)
            } else {
                Err(GetLastError())
            }

        }
    }
    pub fn set_param(&self, param: ffi::WINDIVERT_PARAM, value: u64) -> Result<(), winapi::DWORD> {
        unsafe {
            if ffi::WinDivertSetParam(self.handle, param, value) == winapi::TRUE {
                Ok(())
            } else {
                Err(GetLastError())
            }
        }
    }
    pub fn get_param(&self, param: ffi::WINDIVERT_PARAM) -> Result<u64, winapi::DWORD> {
        unsafe {
            let mut value: u64 = uninitialized();
            if ffi::WinDivertGetParam(self.handle, param, &mut value) == winapi::TRUE {
                Ok(value)
            } else {
                Err(GetLastError())
            }
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
