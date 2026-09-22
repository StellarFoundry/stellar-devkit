//! Measure peak memory while decoding large base64 XDR `ScVal` inputs.
//!
//! Run with:
//!
//! ```text
//! cargo run --release -p devkit-xdr --example mem_profile
//! ```
//!
//! Optionally pass comma-separated input sizes in bytes:
//!
//! ```text
//! cargo run --release -p devkit-xdr --example mem_profile -- 1024,65536,1048576
//! ```
//!
//! The example reports memory per input size as CSV. Peak/current memory is
//! read from platform APIs (Linux `/proc/self/status`, Windows
//! `GetProcessMemoryInfo`) with a documented fallback to `n/a` on other
//! platforms. See `docs/MEMORY.md` for methodology and limits.

use devkit_xdr::inspect_scval_base64;
use stellar_xdr::{BytesM, Limits, ScBytes, ScVal, WriteXdr};

const DEFAULT_SIZES: &[usize] = &[1024, 64 * 1024, 1024 * 1024];

#[derive(Clone, Copy)]
struct Memory {
    current: Option<u64>,
    peak: Option<u64>,
}

fn main() {
    println!(
        "size_bytes,base64_bytes,decoded_json_bytes,current_bytes,peak_bytes,delta_current_bytes"
    );
    let mut measured = false;
    for size in sizes_from_args() {
        let bytes = BytesM::try_from(vec![0xAB; size]).expect("sample within MAX");
        let input = ScVal::Bytes(ScBytes::from(bytes))
            .to_xdr_base64(Limits::none())
            .expect("encode sample scval");
        let before = platform::sample();
        let json = inspect_scval_base64(&input).expect("decode sample scval");
        let after = platform::sample();
        let decoded_bytes = serde_json::to_string(&json).map(|s| s.len()).unwrap_or(0);
        measured |= after.peak.is_some();
        println!(
            "{size},{},{decoded_bytes},{},{},{}",
            input.len(),
            opt(after.current),
            opt(after.peak),
            delta(after.current, before.current),
        );
    }
    if !measured {
        eprintln!("note: memory sampling is unsupported on this platform; see docs/MEMORY.md");
    }
}

fn sizes_from_args() -> Vec<usize> {
    let Some(spec) = std::env::args().nth(1) else {
        return DEFAULT_SIZES.to_vec();
    };
    let sizes: Vec<usize> = spec
        .split(',')
        .filter_map(|part| part.trim().parse().ok())
        .collect();
    if sizes.is_empty() {
        DEFAULT_SIZES.to_vec()
    } else {
        sizes
    }
}

fn opt(value: Option<u64>) -> String {
    value
        .map(|v| v.to_string())
        .unwrap_or_else(|| "n/a".to_string())
}

fn delta(after: Option<u64>, before: Option<u64>) -> String {
    match (after, before) {
        (Some(a), Some(b)) => a.saturating_sub(b).to_string(),
        _ => "n/a".to_string(),
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use super::Memory;

    fn field(status: &str, key: &str) -> Option<u64> {
        status
            .lines()
            .find(|line| line.starts_with(key))
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|value| value.parse::<u64>().ok())
            .map(|kib| kib * 1024)
    }

    pub fn sample() -> Memory {
        let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
        Memory {
            current: field(&status, "VmRSS:"),
            peak: field(&status, "VmHWM:"),
        }
    }
}

#[cfg(windows)]
mod platform {
    use super::Memory;
    use std::ffi::c_void;
    use std::mem;

    #[repr(C)]
    struct ProcessMemoryCounters {
        cb: u32,
        page_fault_count: u32,
        peak_working_set_size: usize,
        working_set_size: usize,
        quota_peak_paged_pool_usage: usize,
        quota_paged_pool_usage: usize,
        quota_peak_non_paged_pool_usage: usize,
        quota_non_paged_pool_usage: usize,
        pagefile_usage: usize,
        peak_pagefile_usage: usize,
    }

    #[link(name = "psapi")]
    extern "system" {
        fn GetCurrentProcess() -> *mut c_void;
        fn GetProcessMemoryInfo(
            process: *mut c_void,
            counters: *mut ProcessMemoryCounters,
            cb: u32,
        ) -> i32;
    }

    pub fn sample() -> Memory {
        unsafe {
            let mut counters: ProcessMemoryCounters = mem::zeroed();
            counters.cb = mem::size_of::<ProcessMemoryCounters>() as u32;
            if GetProcessMemoryInfo(GetCurrentProcess(), &mut counters, counters.cb) != 0 {
                Memory {
                    current: Some(counters.working_set_size as u64),
                    peak: Some(counters.peak_working_set_size as u64),
                }
            } else {
                Memory {
                    current: None,
                    peak: None,
                }
            }
        }
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
mod platform {
    use super::Memory;

    pub fn sample() -> Memory {
        Memory {
            current: None,
            peak: None,
        }
    }
}
