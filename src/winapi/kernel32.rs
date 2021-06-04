#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::*;


#[link(name = "kernel32")]
extern "system" {
    pub fn OpenProcess(
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        dwProcessId: DWORD
    ) -> HANDLE;

    pub fn VirtualQueryEx(
        hProcess: HANDLE,
        lpAddress: LPCVOID,
        lpBuffer: PMEMORY_BASIC_INFORMATION,
        dwLength: SIZE_T,
    ) -> SIZE_T;

    pub fn ReadProcessMemory(
        hProcess: HANDLE,
        lpBaseAddress: LPCVOID,
        lpBuffer: LPVOID,
        nSize: SIZE_T,
        lpNumberOfBytesRead: *mut SIZE_T,
    ) -> BOOL;

    pub fn CloseHandle(
        hObject: HANDLE
    ) -> BOOL;

    pub fn DebugActiveProcess(
        dwProcessId: DWORD
    ) -> BOOL;

    pub fn DebugActiveProcessStop(
        dwProcessId: DWORD
    ) -> BOOL;
}
