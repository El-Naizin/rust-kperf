pub mod error;
pub mod event;
pub mod kperf;
pub mod safe_wrappers;

use error::KperfError;
use event::Event;
use kperf::KProbesConfig;
use kperf::KProbesDatabase;
pub use kperf_sys;
use kperf_sys::constants::KPC_CLASS_CONFIGURABLE_MASK;
use kperf_sys::functions::{kpc_force_all_ctrs_set, kpc_get_thread_counters, kpc_set_config, kpc_set_counting, kpc_set_thread_counting, kpep_config_add_event, kpep_config_force_counters, kpep_config_kpc, kpep_config_kpc_classes, kpep_config_kpc_count, kpep_config_kpc_map};
use kperf_sys::structs::kpc_config_t;
use libc::{c_uint, c_ulonglong, size_t};
use std::error::Error;
use std::mem::size_of;
use std::ptr::null_mut;

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
        unsafe {
            self.kprobes_config.force_counters()?;

            match self.tracked_event {
                Event::Cycles => {
                    self.kprobes_config.add_event(&self.kprobes_db, Event::Cycles)?;
                    self.kprobes_config.add_event(&self.kprobes_db, Event::Branches)?;
                    // self.kprobes_config.add_event(&self.kprobes_db, Event::Instructions)?;
                    // self.kprobes_config.add_event(&self.kprobes_db, Event::BranchMisses)?;
                }
                _ => {
                    return Err(KperfError::PerfCounterBuildError(format!(
                        "Event type {:?} not implemented yet!",
                        self.tracked_event
                    )))
                }
            }

            self.kprobes_config.fill_config_variables()?;

            let res = kpc_force_all_ctrs_set(1); // Set config to kernel
            if res != 0 {
                return Err(KperfError::PerfCounterBuildError(format!(
                    "Failed to force_all_ctrs_set, error: {}",
                    res
                )));
            }

            self.kprobes_config.set_kpc_config()?;

            self.kprobes_config.start_kpc_counting()?;

            self.kprobes_config.start_kpc_thread_counting()?;
            // TODO: What functions are actually usefull?
        }

        println!(
            "{}",
            self.kprobes_config
        );
        let counter_idx = self.kprobes_config.get_counter_index();

        Ok(PerfCounter {
            kprobes_db: self.kprobes_db,
            kprobes_config: self.kprobes_config,
            counters_start: [0 as c_ulonglong; KPC_MAX_COUNTERS],
            counters_end: [0 as c_ulonglong; KPC_MAX_COUNTERS],
            counter_idx,
            started: false,
        })
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
    pub fn start(&mut self) -> Result<(), ()> {
        if self.started {
            return Err(());
        }
        unsafe {
            let res = kpc_get_thread_counters(
                0,
                KPC_MAX_COUNTERS as c_uint,
                self.counters_start.as_mut_ptr(),
            );
            if res != 0 {
                println!("Failed to get thread counters : {}", res);
                return Err(());
            }
        }
        self.started = true;
        return Ok(());
    }

    pub fn stop(&mut self) -> Result<(), ()> {
        if !self.started {
            return Err(());
        }
        unsafe {
            let res = kpc_get_thread_counters(
                0,
                KPC_MAX_COUNTERS as c_uint,
                self.counters_end.as_mut_ptr(),
            );
            if res != 0 {
                println!("Failed to get thread counters : {}", res);
                return Err(());
            }
        }
        self.started = false;
        return Ok(());
    }

    pub fn reset(&mut self) {}

    pub fn read(&mut self) -> u64 {
        return dbg!(self.counters_end)[self.counter_idx] - self.counters_start[self.counter_idx];
    }
}
