#![allow(dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case)]
extern crate winapi;

use winapi::*;

/// *************************************************************************
// WINDIVERT API
/// *************************************************************************
// Divert address.
//
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct WINDIVERT_ADDRESS {
    pub IfIdx: UINT32, // Packet's interface index.
    pub SubIfIdx: UINT32, // Packet's sub-interface index.
    pub Direction: UINT8, // Packet's direction.
}

pub type PWINDIVERT_ADDRESS = *mut WINDIVERT_ADDRESS;

pub const WINDIVERT_DIRECTION_OUTBOUND: UINT8 = 0;
pub const WINDIVERT_DIRECTION_INBOUND: UINT8 = 1;

// Divert layers.
//
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum WINDIVERT_LAYER {
    WINDIVERT_LAYER_NETWORK = 0, // Network layer.
    WINDIVERT_LAYER_NETWORK_FORWARD = 1, // Network layer (forwarded packets)
}

pub type PWINDIVERT_LAYER = *mut WINDIVERT_LAYER;

// Divert flags.
//
pub const WINDIVERT_FLAG_SNIFF: UINT64 = 1;
pub const WINDIVERT_FLAG_DROP: UINT64 = 2;

// Divert parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum WINDIVERT_PARAM {
    WINDIVERT_PARAM_QUEUE_LEN = 0, // Packet queue length.
    WINDIVERT_PARAM_QUEUE_TIME = 1, // Packet queue time.
}

pub type PWINDIVERT_PARAM = *mut WINDIVERT_PARAM;

pub const WINDIVERT_PARAM_MAX: WINDIVERT_PARAM = WINDIVERT_PARAM::WINDIVERT_PARAM_QUEUE_TIME;

extern "C" {
    // Open a WinDivert handle.
    //
    pub fn WinDivertOpen(filter: *const c_char,
                         layer: WINDIVERT_LAYER,
                         priority: INT16,
                         flags: UINT64)
                         -> HANDLE;

    // Receive (read) a packet from a WinDivert handle.
    //
    pub fn WinDivertRecv(handle: HANDLE,
                         pPacket: PVOID,
                         packetLen: UINT,
                         pAddr: PWINDIVERT_ADDRESS,
                         readLen: *mut UINT)
                         -> BOOL;

    // Receive (read) a packet from a WinDivert handle.
    //
    pub fn WinDivertRecvEx(handle: HANDLE,
                           pPacket: PVOID,
                           packetLen: UINT,
                           flags: UINT64,
                           pAddr: PWINDIVERT_ADDRESS,
                           readLen: *mut UINT,
                           lpOverlapped: LPOVERLAPPED)
                           -> BOOL;

    // Send (write/inject) a packet to a WinDivert handle.
    //
    pub fn WinDivertSend(handle: HANDLE,
                         pPacket: PVOID,
                         packetLen: UINT,
                         pAddr: *const WINDIVERT_ADDRESS,
                         writeLen: *mut UINT)
                         -> BOOL;

    // Send (write/inject) a packet to a WinDivert handle.
    //
    pub fn WinDivertSendEx(handle: HANDLE,
                           pPacket: PVOID,
                           packetLen: UINT,
                           flags: UINT64,
                           pAddr: *const WINDIVERT_ADDRESS,
                           writeLen: *mut UINT,
                           lpOverlapped: LPOVERLAPPED)
                           -> BOOL;

    // Close a WinDivert handle.
    //
    pub fn WinDivertClose(handle: HANDLE) -> BOOL;

    // Set a WinDivert handle parameter.
    //
    pub fn WinDivertSetParam(handle: HANDLE, param: WINDIVERT_PARAM, value: UINT64) -> BOOL;

    // Get a WinDivert handle parameter.
    //
    pub fn WinDivertGetParam(handle: HANDLE, param: WINDIVERT_PARAM, pValue: *mut UINT64) -> BOOL;
}

/// *************************************************************************
// WINDIVERT LEGACY API
/// *************************************************************************

// Deprecated API:
//
const WINDIVERT_FLAG_NO_CHECKSUM: UINT64 = 0;
