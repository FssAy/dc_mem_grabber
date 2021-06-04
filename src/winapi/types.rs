#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use libc::*;


pub const PROCESS_ALL_ACCESS: DWORD = 2_097_151u32;
pub const FALSE: BOOL = 0;
pub const MEM_COMMIT: DWORD = 0x1000;
pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
pub const PAGE_GUARD: DWORD = 0x100;

pub type PVOID = *mut c_void;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;
pub type HANDLE = LPVOID;
pub type DWORD = u32;
pub type BOOL = i32;
pub type SIZE_T = usize;

pub type PMEMORY_BASIC_INFORMATION = *mut MEMORY_BASIC_INFORMATION;

#[derive(Clone)]
#[repr(C)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: PVOID,
    pub AllocationBase: PVOID,
    pub AllocationProtect: DWORD,
    // pub PartitionId: WORD,
    pub RegionSize: SIZE_T,
    pub State: DWORD,
    pub Protect: DWORD,
    pub Type: DWORD,
}
