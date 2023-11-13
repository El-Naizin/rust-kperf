use libc::c_int;
use kperf_sys;


fn main() {
    let mut force_ctrs: c_int = 0;
    unsafe {
        if kperf_sys::functions::kpc_force_all_ctrs_get(&mut force_ctrs) != 0{
            println!("Permission denied, xnu/kpc requires root privileges");
            return;
        }
        println!("{}", force_ctrs);
    }
    println!("Hello, world!");
}
