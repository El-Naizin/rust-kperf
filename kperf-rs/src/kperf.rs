use crate::error::KpepError::UnknownError;
use crate::error::{KpepError, KperfError};
use crate::event::get_event;
use crate::event::Event;
use crate::KPC_MAX_COUNTERS;
use kperf_sys::constants::KPC_CLASS_CONFIGURABLE_MASK;
use kperf_sys::functions::{
    kpc_set_config, kpc_set_counting, kpc_set_thread_counting, kpep_config_add_event,
    kpep_config_create, kpep_config_force_counters, kpep_config_kpc, kpep_config_kpc_classes,
    kpep_config_kpc_count, kpep_config_kpc_map, kperf_ns_to_ticks, kperf_tick_frequency,
    kperf_ticks_to_ns,
};
use kperf_sys::structs::{kpc_config_t, kpep_config, kpep_db};
use libc::{c_uint, c_ulonglong, size_t};
use std::ffi::{CStr, CString};
use std::fmt;
use std::fmt::Formatter;
use std::mem::size_of;
use std::ptr::{null, null_mut};

#[derive(Debug)]
pub struct KProbesConfig {
    pub config: *mut kpep_config,
    classes: c_uint,
    reg_count: size_t,
    counter_map: [size_t; KPC_MAX_COUNTERS],
    kpc_registers: [kpc_config_t; KPC_MAX_COUNTERS],
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
                return Err(KpepError::UnknownError);
            }
        }

        return Ok(Self {
            config,
            classes: 0,
            reg_count: 0,
            counter_map: [0; KPC_MAX_COUNTERS],
            kpc_registers: [0; KPC_MAX_COUNTERS],
        });
    }

    pub fn add_event(&mut self, db: &KProbesDatabase, event_type: Event) -> Result<(), KperfError> {
        let mut event = get_event(event_type, db).ok_or(KperfError::UnknownError(format!(
            "Couldn't find matching event for event type: {}",
            event_type
        )))?;
        unsafe {
            let res = kpep_config_add_event(self.config, &mut event, 0, null_mut());
            if res != 0 {
                return Err(KperfError::UnknownError(format!(
                    "Error when adding event: {}, error code: {}",
                    event_type, res
                )));
            }
        }
        Ok(())
    }

    pub fn force_counters(&mut self) -> Result<(), KperfError> {
        let res = unsafe { kpep_config_force_counters(self.config) };
        if res != 0 {
            return Err(KperfError::UnknownError(format!(
                "Failed to force counters, error: {}",
                res
            )));
        }
        Ok(())
    }

    pub fn fill_config_variables(&mut self) -> Result<(), KperfError> {
        /// Fill config variables classes, reg_count, counter_map, and kpc_registers
        /// from kperf api.
        let res = unsafe { kpep_config_kpc_classes(self.config, &mut self.classes) };
        if res != 0 {
            return Err(KperfError::PerfCounterBuildError(format!(
                "Failed to get Kernel Performance Counters classes, error: {}",
                res
            )));
        }

        let res = unsafe { kpep_config_kpc_count(self.config, &mut self.reg_count) };
        if res != 0 {
            return Err(KperfError::PerfCounterBuildError(format!(
                "Failed to get Kernel Performance Counters register count, error: {}",
                res
            )));
        }

        let res = unsafe {
            kpep_config_kpc_map(
                self.config,
                self.counter_map.as_mut_ptr(),
                size_of::<[size_t; KPC_MAX_COUNTERS]>(),
            )
        };
        if res != 0 {
            return Err(KperfError::PerfCounterBuildError(format!(
                "Failed to get Kernel Performance Counters register mapping, error: {}",
                res
            )));
        }

        let res = unsafe {
            kpep_config_kpc(
                self.config,
                self.kpc_registers.as_mut_ptr(),
                size_of::<[kpc_config_t; KPC_MAX_COUNTERS]>(),
            )
        };
        if res != 0 {
            return Err(KperfError::PerfCounterBuildError(format!(
                "Failed to configure kpc, error: {}",
                res
            )));
        }

        Ok(())
    }

    pub fn start_kpc_counting(&mut self) -> Result<(), KperfError> {
        let res = unsafe { kpc_set_counting(self.classes) };
        if res != 0 {
            return Err(KperfError::PerfCounterBuildError(format!(
                "Failed to start kpc counting, error: {}",
                res
            )));
        }
        Ok(())
    }
    pub fn start_kpc_thread_counting(&mut self) -> Result<(), KperfError> {
        let res = unsafe { kpc_set_thread_counting(self.classes) };
        if res != 0 {
            return Err(KperfError::PerfCounterBuildError(format!(
                "Failed to start kpc thread counting, error: {}",
                res
            )));
        }
        Ok(())
    }

    pub fn set_kpc_config(&mut self) -> Result<(), KperfError> {
        if (self.classes & KPC_CLASS_CONFIGURABLE_MASK) != 0 && self.reg_count != 0 {
            let res = unsafe { kpc_set_config(self.classes, self.kpc_registers.as_mut_ptr()) };
            if res != 0 {
                return Err(KperfError::PerfCounterBuildError(format!(
                    "Failed to set kpc config, error: {}",
                    res
                )));
            }
            return Ok(());
        } else if (self.classes & KPC_CLASS_CONFIGURABLE_MASK) == 0 {
            println!("Warn: classes weren't configurable and register count was 0, should be OK");
            return Ok(());
            // return Err(KperfError::PerfCounterBuildError(format!("KProbes Register count is 0")));
        } else if self.reg_count == 0 {
            return Err(KperfError::PerfCounterBuildError(format!(
                "KPC is configurable but reg_count is 0, probably shouldn't happen"
            )));
        } else {
            return Err(KperfError::UnknownError(format!(
                "Should never get to here"
            )));
        }
    }

    pub fn get_counter_index(&mut self) -> usize {
        //TODO: see if this is OK for every event, should be
        self.counter_map[0]
    }
}

impl fmt::Display for KProbesConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
               "KProbesConfig:\nclasses: {}\nreg_count: {}\ncounter_map: {:?}\nkpc_registers: {:?}\nkpep_config: {:?}\n",
               self.classes,
               self.reg_count,
               self.counter_map,
               self.kpc_registers,
               self.config
        )
    }
}

#[derive(Debug)]
pub struct KProbesDatabase {
    pub database: *mut kpep_db, // TODO: make this non public
}

impl KProbesDatabase {
    pub fn load_database() -> Result<Self, KpepError> {
        let mut db: *mut kpep_db = null_mut();
        unsafe {
            let mut ret = kperf_sys::functions::kpep_db_create(null(), &mut db);
            if ret != 0 {
                return Err(KpepError::UnknownError);
            }
        }

        Ok(Self { database: db })
    }

    pub fn get_fixed_counter_count(&self) -> usize {
        unsafe { (*self.database).fixed_counter_count as usize }
    }

    pub fn get_configurable_counter_count(&self) -> usize {
        unsafe { (*self.database).config_counter_count as usize }
    }

    pub fn get_db_name(&self) -> Option<String> {
        unsafe {
            if (*self.database).name.is_null() {
                return None;
            }
            let name = CString::from(CStr::from_ptr((*self.database).name)).into_string();
            match name {
                Ok(x) => return Some(x),
                Err(x) => return None,
            }
        }
    }
}

pub fn get_tick_frequency() -> u64 {
    unsafe { kperf_tick_frequency() as u64 }
}

const TICKS_TO_NANOSECONDS_MAGIC_NUMBER: u64 = 100; // Magic number, don't ask
pub fn ticks_to_nanoseconds(cpu_ticks: u64) -> u64 {
    unsafe {
        kperf_ticks_to_ns(cpu_ticks as c_ulonglong) as u64 / TICKS_TO_NANOSECONDS_MAGIC_NUMBER
    }
}

pub fn nanaseconds_to_ticks(nanoseconds: u64) -> u64 {
    unsafe {
        kperf_ns_to_ticks(nanoseconds * TICKS_TO_NANOSECONDS_MAGIC_NUMBER as c_ulonglong) as u64
    }
}
