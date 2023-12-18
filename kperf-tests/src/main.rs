use std::ffi::{CStr, CString};
use std::mem::size_of;
use std::ptr::{null, null_mut};
use libc::{c_char, c_int, size_t};
use kperf_sys;
use kperf_sys::constants::KPC_CLASS_CONFIGURABLE;
use kperf_sys::functions::{kpc_force_all_ctrs_get, kpc_force_all_ctrs_set, kpc_set_config, kpc_set_counting, kpc_set_thread_counting, kpep_config_add_event, kpep_config_create, kpep_config_force_counters, kpep_config_kpc, kpep_config_kpc_classes, kpep_config_kpc_count, kpep_config_kpc_map, kpep_db_event};
use kperf_sys::structs::{kpc_config_t, kpep_db, kpep_event};

const KPC_MAX_COUNTERS: size_t = 32;

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

        let mut ev_branches = get_event(Event::Branches, &mut *db).unwrap();
        kpep_config_add_event(config, &mut ev_branches, 0, null_mut());
        let mut ev_cycles = get_event(Event::Cycles, &mut *db).unwrap();
        kpep_config_add_event(config, &mut ev_cycles, 0, null_mut());

        let mut classes = 0;
        let mut reg_count = 0;
        let res = kpep_config_kpc_classes(config, &mut classes);
        if res != 0 {
            println!(
                "Failed to get kpc classes, error: {}",
                res
            )
        }
        println!(
            "kpc classes: {}",
            classes
        );
        let res = kpep_config_kpc_count(config, &mut reg_count);
        if res != 0 {
            println!(
                "Failed to get kpc count, error: {}",
                res
            )
        }
        println!(
            "kpc count: {}",
            reg_count
        );
        let mut counter_map: [size_t; KPC_MAX_COUNTERS] = [0; KPC_MAX_COUNTERS];
        println!(
            "{}",
            size_of::<[size_t; KPC_MAX_COUNTERS]>()
        );
        let res = kpep_config_kpc_map(config, counter_map.as_mut_ptr(), size_of::<[size_t; KPC_MAX_COUNTERS]>());
        if res != 0 {
            println!(
                "Failed to initialize kpc map, error: {}",
                res
            )
        }
        println!(
            "{:?}",
            counter_map,
        );

        let mut regs: [kpc_config_t; KPC_MAX_COUNTERS] = [0; KPC_MAX_COUNTERS];
        let res = kpep_config_kpc(config, regs.as_mut_ptr(), size_of::<[kpc_config_t; KPC_MAX_COUNTERS]>());
        if res != 0 {
            println!(
                "Failed to configure kpc, error: {}",
                res
            );
        }

        kpc_force_all_ctrs_set(1);
        if (classes & KPC_CLASS_CONFIGURABLE) != 0 && reg_count != 0 {
            let res = kpc_set_config(classes, regs.as_mut_ptr());
            if res != 0 {
                println!(
                    "Failed to set kpc config, error: {}",
                    res
                );
            }
        }
        println!(
            "regs = {:?}",
            regs
        );

        let res = kpc_set_counting(classes);
        if res != 0 {
            println!(
                "Failed to start kpc counting, error: {}",
                res
            );
        }

        let res = kpc_set_thread_counting(classes);
        if res != 0 {
            println!(
                "Failed to start kpc thread counting, error: {}",
                res
            )
        }

        let res =
    }
    println!("Hello, world!");
}
