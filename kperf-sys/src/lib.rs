pub mod structs;
pub mod functions;
pub mod constants;


#[cfg(test)]
mod tests {
    use crate::{structs::*, constants::*, functions::* };
    use std::ptr::{null, null_mut};
    const LIBRARY_PATH: &str = "/System/Library/PrivateFrameworks/kperf.framework/kperf";


    #[test]
    fn test_running_function() {
        let pmu_version = unsafe {
            kpc_pmu_version()
        };
    }

    #[test]
    fn test_pmu_version() {
        let pmu_version = unsafe {
            kpc_pmu_version()
        };
        assert_ne!(pmu_version, 0, "A PMU version of zero means an error, try to run this program in sudo");
    }

    #[test]
    fn test_create_kpep_db() {
        unsafe {
            let mut db_ptr: *mut kpep_db = null_mut();
            // let mut name: &[c_char; 12] = b"test_rust_db";
            // let name = CStr::from_bytes_until_nul().unwrap();
            // let name_ptr: *const c_char = name.as_ptr() as *const c_char;
            let result = kpep_db_create(null(), &mut db_ptr);
            println!("db creation result: {}", result);
        }
    }
}
