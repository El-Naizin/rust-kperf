pub use kperf_sys;
use libc::c_int;
use std::error::Error;
use std::ffi::CString;
use std::io::ErrorKind::PermissionDenied;
use std::ptr::{null, null_mut};
use kperf_sys::functions::{kpep_config_create, kpep_db_event};
use kperf_sys::structs::{kpep_config, kpep_db, kpep_event};

#[derive(Debug, Copy, Clone)]
pub enum KpepError {
    UnknownError
}

#[derive(Debug)]
pub struct KProbesConfig {
    pub config: *mut kpep_config,
}

impl KProbesConfig {
    pub fn from_database(database: &mut KProbesDatabase) -> Result<Self, KpepError> {
        let mut config = null_mut();
        unsafe {
            let res = kpep_config_create(database.database, &mut config);
            if res != 0 {
                // println!(
                //     "Failed to create kpep config, error: {}",
                //     res
                // );
                return Err(KpepError::UnknownError)
            }
        }

        return Ok(Self {
            config
        })
    }
}

#[derive(Debug)]
pub struct KProbesDatabase {
    pub database: *mut kpep_db,
}

impl KProbesDatabase {
    pub fn load_database() -> Result<Self, KpepError> {
        let mut db: *mut kpep_db = null_mut();
        unsafe {
            let mut ret = kperf_sys::functions::kpep_db_create(
                null(),
                &mut db,
            );
            if ret != 0 {
                return Err(KpepError::UnknownError)
            }
        }

        Ok(Self {
            database: db
        })
    }
}

pub struct PerfCounterBuilder {
}

impl PerfCounterBuilder {
    fn build_counter() {
    }
}

pub struct PerfCounter {
}

impl PerfCounter {
    fn start(&mut self) {
    }

    fn stop(&mut self) {
    }

    fn reset(&mut self) {
    }

    fn read(&mut self) {
    }
}

#[derive(Debug)]
pub enum KperfError {
    PermissionDenied,
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Cycles,
    Instructions,
    Branches,
    BranchMisses,
}

pub fn get_event_names(event_type: Event) -> Vec<CString> {
    match event_type {
        Event::Cycles => {
            vec![
                CString::new("FIXED_CYCLES").unwrap(),             // Apple A7-A15
                CString::new("CPU_CLK_UNHALTED.THREAD").unwrap(),  // Intel Core 1th-10th
                CString::new("CPU_CLK_UNHALTED.CORE").unwrap(),    // Intel Yonah, Merom
            ]
        }
        Event::Instructions => {
            vec![
                CString::new("FIXED_INSTRUCTIONS").unwrap(),  // Apple A7-A15
                CString::new("INST_RETIRED.ANY").unwrap(),    // Intel Yonah, Merom, Core 1th-10th
            ]
        }
        Event::Branches => {
            vec![
                CString::new("INST_BRANCH").unwrap(),             // Apple A7-A15
                CString::new("BR_INST_RETIRED.ALL_BRANCHES").unwrap(),  // Intel Core 1th-10th
                CString::new("INST_RETIRED.ANY").unwrap(),    // Intel Yonah, Merom
            ]
        }
        Event::BranchMisses => {
            vec![
                CString::new("BRANCH_MISPRED_NONSPEC").unwrap(),       // Apple A7-A15, since iOS 15, macOS 12
                CString::new("BRANCH_MISPREDICT").unwrap(),            // Apple A7-A14
                CString::new("BR_MISP_RETIRED.ALL_BRANCHES").unwrap(), // Intel Core 2th-10th
                CString::new("BR_INST_RETIRED.MISPRED").unwrap(),      // Intel Yonah, Merom
            ]
        }
    }
}

pub fn get_event(event_type: Event, db: &mut kpep_db) -> Option<*mut kpep_event>{
    let names = get_event_names(event_type);
    for name in names {
        unsafe {
            let mut ev: *mut kpep_event = null_mut();
            if kpep_db_event(db, name.as_ptr(), &mut ev) == 0 {
                return Some(ev);
            }
        }
    }
    return None;
}

pub fn check_kpc_permission() -> Result<(), KperfError>{
    let mut force_ctrs: c_int = 0;
    unsafe {
        let res = kperf_sys::functions::kpc_force_all_ctrs_get(&mut force_ctrs);
        if res != 0 {
            println!("Permission denied, xnu/kpc requires root privileges (error code: {})", res);
            return Err(KperfError::PermissionDenied)
        }
    }
    return Ok(())
}
