// https://doc.rust-lang.org/cargo/reference/build-script-examples.html
#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::ffi::c_void;
use std::mem::{size_of, zeroed};
use std::thread::sleep;
use std::time::Duration;

#[repr(C)]
struct Memorystatusex {
    dwLength: u32,
    dwMemoryLoad: u32,
    ullTotalPhys: u64,
    ullAvailPhys: u64,
    ullTotalPageFile: u64,
    ullAvailPageFile: u64,
    ullTotalVirtual: u64,
    ullAvailVirtual: u64,
    ullAvailExtendedVirtual: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Filetime {
    dwLowDateTime: u32,
    dwHighDateTime: u32,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GlobalMemoryStatusEx(lpBuffer: *mut Memorystatusex) -> i32;
    fn GetSystemTimes(
        lpIdleTime: *mut Filetime,
        lpKernelTime: *mut Filetime,
        lpUserTime: *mut Filetime,
    ) -> i32;
}

fn memory_usage_percent() -> u32 {
    unsafe {
        let mut mem_info = Memorystatusex {
            dwLength: size_of::<Memorystatusex>() as u32,
            dwMemoryLoad: 0,
            ullTotalPhys: 0,
            ullAvailPhys: 0,
            ullTotalPageFile: 0,
            ullAvailPageFile: 0,
            ullTotalVirtual: 0,
            ullAvailVirtual: 0,
            ullAvailExtendedVirtual: 0,
        };
        if GlobalMemoryStatusEx(&mut mem_info) != 0 {
            mem_info.dwMemoryLoad
        } else {
            0
        }
    }
}

fn filetime_to_u64(ft: Filetime) -> u64 {
    ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64)
}

fn cpu_usage_percent() -> f64 {
    unsafe {
        let (mut idle1, mut kernel1, mut user1) = (
            zeroed::<Filetime>(),
            zeroed::<Filetime>(),
            zeroed::<Filetime>(),
        );
        GetSystemTimes(&mut idle1, &mut kernel1, &mut user1);

        sleep(Duration::from_millis(500));

        let (mut idle2, mut kernel2, mut user2) = (
            zeroed::<Filetime>(),
            zeroed::<Filetime>(),
            zeroed::<Filetime>(),
        );
        GetSystemTimes(&mut idle2, &mut kernel2, &mut user2);

        let idle_diff = filetime_to_u64(idle2) - filetime_to_u64(idle1);
        let kernel_diff = filetime_to_u64(kernel2) - filetime_to_u64(kernel1);
        let user_diff = filetime_to_u64(user2) - filetime_to_u64(user1);

        let total = kernel_diff + user_diff;
        if total == 0 {
            return 0.0;
        }
        (1.0 - idle_diff as f64 / total as f64) * 100.0
    }
}

fn main() {
    println!(
        "cargo:warning=Memory usage: {:}%",
        memory_usage_percent()
    );
    println!(
        "cargo:warning=CPU usage: {:.2}%",
        cpu_usage_percent()
    );
}
