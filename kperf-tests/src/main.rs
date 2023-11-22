use std::ffi::{CStr, CString};
use std::ptr::{null, null_mut};
use libc::{c_char, c_int};
use kperf_sys;
use kperf_sys::functions::{kpep_config_create, kpep_config_force_counters, kpep_db_event};
use kperf_sys::structs::{kpep_db, kpep_event};

// const EVENT_NAME_MAX: usize = 8;
// struct EventAlias {
//     alias: Event,
//     names: [CStr; EVENT_NAME_MAX],
// }
//
// /// Event names from /usr/share/kpep/<name>.plist
// const PROFILE_EVENTS: [EventAlias; 4] = [
//     EventAlias {
//         alias: Event::Cycles,
//         names: [
//             CStr::new("FIXED_CYCLES"),             // Apple A7-A15
//             CStr::new("CPU_CLK_UNHALTED.THREAD"),  // Intel Core 1th-10th
//             CStr::new("CPU_CLK_UNHALTED.CORE"),    // Intel Yonah, Merom
//             CStr::new(""),
//             CStr::new(""),
//             CStr::new(""),
//             CStr::new(""),
//             CStr::new(""),
//         ]
//     },
//     EventAlias {
//         alias: Event::Instructions,
//         names: [
//             CStr::new("FIXED_INSTRUCTIONS"),  // Apple A7-A15
//             CStr::new("INST_RETIRED.ANY"),    // Intel Yonah, Merom, Core 1th-10th
//             CStr::new(""),
//             CStr::new(""),
//             CStr::new(""),
//             CStr::new(""),
//             CStr::new(""),
//             CStr::new(""),
//         ]
//     },
//     EventAlias {
//         alias: Event::Branches,
//         names: [
//             CStr::new("INST_BRANCH"),                    // Apple A7-A15
//             CStr::new("BR_INST_RETIRED.ALL_BRANCHES"),   // Intel Core 1th-10th
//             CStr::new("INST_RETIRED.ANY"),               // Intel Yonah, Merom
//             CStr::new(""),
//             CStr::new(""),
//             CStr::new(""),
//             CStr::new(""),
//             CStr::new(""),
//         ]
//     },
//     EventAlias {
//         alias: Event::BranchMisses,
//         names: [
//             CStr::new("BRANCH_MISPRED_NONSPEC"),       // Apple A7-A15, since iOS 15, macOS 12
//             CStr::new("BRANCH_MISPREDICT"),            // Apple A7-A14
//             CStr::new("BR_MISP_RETIRED.ALL_BRANCHES"), // Intel Core 2th-10th
//             CStr::new("BR_INST_RETIRED.MISPRED"),      // Intel Yonah, Merom
//             CStr::new(""),
//             CStr::new(""),
//             CStr::new(""),
//             CStr::new(""),
//         ]
//     },
// ];

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
            if kpep_db_event(db, name.as_ptr(), &mut ev) == 1 {
                return Some(ev);
            }
        }
    }
    return None;
}

fn main() {
    let mut force_ctrs: c_int = 0;
    unsafe {
        // Check permission
        if kperf_sys::functions::kpc_force_all_ctrs_get(&mut force_ctrs) != 0 {
            println!("Permission denied, xnu/kpc requires root privileges");
            return;
        }
        println!("{}", force_ctrs);

        // Load pmc database
        let mut db: *mut kpep_db = null_mut();
        let mut ret = kperf_sys::functions::kpep_db_create(
            null(),
            &mut db,
        );
        let db_name = CStr::from_ptr((*db).name);
        let db_marketing_name = CStr::from_ptr((*db).marketing_name);
        println!("loaded db: {:?} ({:?})", db_name, db_marketing_name);

        let mut config = null_mut();
        let res = kpep_config_create(db, &mut config);
        if res != 0 {
            println!(
                "Failed to create kpep config, error: {}",
                res
            )
        }

        let mut res = kpep_config_force_counters(config);
        if res != 0 {
            println!(
                "Failed to force counters, error: {}",
                res
            )
        }
    }
    println!("Hello, world!");
}
