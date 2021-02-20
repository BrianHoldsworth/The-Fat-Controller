// https://docs.microsoft.com/en-us/windows/win32/winprog/windows-data-types

use std::ffi::c_void;

// WinNT.h
pub type LONG = i32;
pub type LPWSTR = *mut u16;
pub type LPCWSTR = *const u16;
pub type PVOID = *mut c_void;
pub type HANDLE = PVOID;

// IntSafe.h
pub type DWORD = u32;

// WinDef.h
pub type WORD = u16;
pub type LPCVOID = *const c_void;
pub type UINT = u32;
pub type HLOCAL = HANDLE;
pub type BOOL = i32;

// BaseTsd.h
#[allow(non_camel_case_types)]
pub type ULONG_PTR = u64;

// Not sure where this is defined

#[repr(C)]
#[derive(Copy, Clone)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}

pub type LPPOINT = *mut POINT;