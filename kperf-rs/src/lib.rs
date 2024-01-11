pub mod error;
pub mod event;
pub mod kperf;

use error::KperfError;
use event::Event;
use kperf::KProbesConfig;
use kperf::KProbesDatabase;
pub use kperf_sys;
use kperf_sys::functions::{kpc_force_all_ctrs_set, kpc_get_thread_counters};
use libc::{c_int, c_uint, c_ulonglong, size_t};
use std::error::Error;
use std::io;

pub enum Track {
    Thread,
}

pub struct PerfCounterBuilder {
    kprobes_config: KProbesConfig, // TODO: this var should be created on build_counter!
    kprobes_db: KProbesDatabase,   // TODO: this var should be created on build_counter!
    tracked_event: Event,          // TODO: Make this an option
}

impl PerfCounterBuilder {
    pub fn new() -> Self {
        let mut kprobes_db =
            KProbesDatabase::load_database().expect("Couldn't load kprobes database");
        let kprobes_config =
            KProbesConfig::from_database(&mut kprobes_db).expect("Couldn't create kpc config");
        Self {
            kprobes_db,
            kprobes_config,
            tracked_event: Event::Cycles,
        }
    }

    pub fn build_counter(mut self) -> Result<PerfCounter, KperfError> {
        // let mut classes = 0;
        // let mut reg_count = 0;
        // let mut counter_map: [size_t; KPC_MAX_COUNTERS] = [0; KPC_MAX_COUNTERS];
        // let mut kpc_registers: [kpc_config_t; KPC_MAX_COUNTERS] = [0; KPC_MAX_COUNTERS];
        self.kprobes_config.force_counters()?;

        match self.tracked_event {
            Event::Cycles => {
                self.kprobes_config
                    .add_event(&self.kprobes_db, Event::Cycles)?;
                // self.kprobes_config.add_event(&self.kprobes_db, Event::Branches)?;
                // self.kprobes_config.add_event(&self.kprobes_db, Event::Instructions)?;
                // self.kprobes_config.add_event(&self.kprobes_db, Event::BranchMisses)?;
            }
            Event::Branches => {
                self.kprobes_config
                    .add_event(&self.kprobes_db, Event::Branches)?;
            }
            Event::BranchMisses => {
                self.kprobes_config
                    .add_event(&self.kprobes_db, Event::BranchMisses)?;
            }
            Event::Instructions => {
                self.kprobes_config
                    .add_event(&self.kprobes_db, Event::Instructions)?;
            }
            _ => {
                return Err(KperfError::PerfCounterBuildError(format!(
                    "Event type {:?} not implemented yet!",
                    self.tracked_event
                )))
            }
        }

        self.kprobes_config.fill_config_variables()?;

        let res = unsafe { kpc_force_all_ctrs_set(1) }; // Set config to kernel
        if res != 0 {
            return Err(KperfError::PerfCounterBuildError(format!(
                "Failed to force_all_ctrs_set, error: {}",
                res
            )));
        }

        self.kprobes_config.set_kpc_config()?;

        // self.kprobes_config.start_kpc_counting()?;
        //
        // self.kprobes_config.start_kpc_thread_counting()?;
        // TODO: What functions are actually usefull?

        let counter_idx = self.kprobes_config.get_counter_index();
        let mut counter = PerfCounter {
            kprobes_db: self.kprobes_db,
            kprobes_config: self.kprobes_config,
            counters_start: [0 as c_ulonglong; KPC_MAX_COUNTERS],
            counters_end: [0 as c_ulonglong; KPC_MAX_COUNTERS],
            counter_idx,
            started: false,
        };

        Ok(counter)
    }

    pub fn track_event(mut self, tracked_event: Event) -> Self {
        self.tracked_event = tracked_event;
        self
    }
}

const KPC_MAX_COUNTERS: size_t = 32;

pub struct PerfCounter {
    kprobes_config: KProbesConfig,
    kprobes_db: KProbesDatabase,
    counters_start: [c_ulonglong; KPC_MAX_COUNTERS],
    counters_end: [c_ulonglong; KPC_MAX_COUNTERS],
    counter_idx: usize,
    started: bool,
}

impl PerfCounter {
    fn fill_start(&mut self) -> Result<(), KperfError> {

        let res = unsafe {
            kpc_get_thread_counters(
                0,
                KPC_MAX_COUNTERS as c_uint,
                self.counters_start.as_mut_ptr(),
            )
        };
        if res != 0 {
            return Err(KperfError::UnknownError(format!(
                "Failed to get thread counters, error: {}",
                res
            )));
        }
        Ok(())
    }

    fn fill_end(&mut self) -> Result<(), KperfError> {
        // if !self.started {
        //     return Ok(());
        // }
        let res = unsafe {
            kpc_get_thread_counters(
                0,
                KPC_MAX_COUNTERS as c_uint,
                self.counters_end.as_mut_ptr(),
            )
        };
        if res != 0 {
            return Err(KperfError::UnknownError(format!(
                "Failed to get thread counters, error: {}",
                res
            )));
        }
        // self.started = false;
        return Ok(());
    }

    pub fn start(&mut self) -> Result<(), KperfError> {
        self.kprobes_config.start_kpc_counting()?;
        self.kprobes_config.start_kpc_thread_counting()?;
        if !self.started {
            self.fill_start()?;
            self.started = true;
        }
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), KperfError> {
        self.kprobes_config.stop_kpc_thread_counting()
    }

    pub fn reset(&mut self) -> Result<(), KperfError> {
        self.kprobes_config.reset_counters()?;
        self.fill_start()?;
        Ok(())
    }

    pub fn read(&mut self) -> Result<u64, KperfError> {
        self.fill_end()?;
        return Ok(self.counters_end[self.counter_idx] - self.counters_start[self.counter_idx]);
    }
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
