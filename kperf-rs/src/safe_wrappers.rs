use crate::error::KperfError;
use crate::event::Event;
use kperf_sys::functions::kpep_db_event;
use kperf_sys::structs::{kpep_db, kpep_event};
use libc::c_int;
use std::ffi::CString;
use std::ptr::null_mut;
use crate::kperf::KProbesDatabase;

pub fn get_event_names(event_type: Event) -> Vec<CString> {
    match event_type {
        Event::Cycles => {
            vec![
                CString::new("FIXED_CYCLES").unwrap(), // Apple A7-A15
                CString::new("CPU_CLK_UNHALTED.THREAD").unwrap(), // Intel Core 1th-10th
                CString::new("CPU_CLK_UNHALTED.CORE").unwrap(), // Intel Yonah, Merom
            ]
        }
        Event::Instructions => {
            vec![
                CString::new("FIXED_INSTRUCTIONS").unwrap(), // Apple A7-A15
                CString::new("INST_RETIRED.ANY").unwrap(),   // Intel Yonah, Merom, Core 1th-10th
            ]
        }
        Event::Branches => {
            vec![
                CString::new("INST_BRANCH").unwrap(), // Apple A7-A15
                CString::new("BR_INST_RETIRED.ALL_BRANCHES").unwrap(), // Intel Core 1th-10th
                CString::new("INST_RETIRED.ANY").unwrap(), // Intel Yonah, Merom
            ]
        }
        Event::BranchMisses => {
            vec![
                CString::new("BRANCH_MISPRED_NONSPEC").unwrap(), // Apple A7-A15, since iOS 15, macOS 12
                CString::new("BRANCH_MISPREDICT").unwrap(),      // Apple A7-A14
                CString::new("BR_MISP_RETIRED.ALL_BRANCHES").unwrap(), // Intel Core 2th-10th
                CString::new("BR_INST_RETIRED.MISPRED").unwrap(), // Intel Yonah, Merom
            ]
        }
    }
}

pub fn get_event(event_type: Event, db: &KProbesDatabase) -> Option<*mut kpep_event> {
    let names = get_event_names(event_type);
    for name in names {
        unsafe {
            let mut ev: *mut kpep_event = null_mut();
            if kpep_db_event(db.database, name.as_ptr(), &mut ev) == 0 {
                return Some(ev);
            }
        }
    }
    return None;
}

pub fn check_kpc_permission() -> Result<(), KperfError> {
    let mut force_ctrs: c_int = 0;
    unsafe {
        let res = kperf_sys::functions::kpc_force_all_ctrs_get(&mut force_ctrs);
        if res != 0 {
            // println!("Permission denied, xnu/kpc requires root privileges (error code: {})", res);
            return Err(KperfError::PermissionDenied);
        }
    }
    return Ok(());
}
