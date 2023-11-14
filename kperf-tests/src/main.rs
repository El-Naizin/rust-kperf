use std::ffi::CStr;
use std::ptr::{null, null_mut};
use libc::c_int;
use kperf_sys;
use kperf_sys::functions::{kpep_config_create, kpep_config_force_counters};
use kperf_sys::structs::kpep_db;


fn main() {
    let mut force_ctrs: c_int = 0;
    unsafe {
        // Check permission
        if kperf_sys::functions::kpc_force_all_ctrs_get(&mut force_ctrs) != 0{
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
