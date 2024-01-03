use libc::{c_char, c_uchar, c_uint, c_ulonglong, c_void, size_t};

#[allow(non_camel_case_types)]
pub type kpc_config_t = u64;

/// KPEP event (size: 48/28 bytes on 64/32 bit OS)
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct kpep_event {
    /// Unique name of a event, such as "INST_RETIRED.ANY".
    pub name: *const c_char,
    /// Description for this event.
    pub description: *const c_char,
    /// Errata, currently NULL.
    pub errata: *const c_char,
    /// Alias name, such as "Instructions", "Cycles".
    pub alias: *const c_char,
    /// Fallback event name for fixed counter.
    pub fallback: *const c_char,
    pub mask: c_uint,
    pub number: c_uchar,
    pub umask: c_uchar,
    pub reserved: c_uchar,
    pub is_fixed: c_uchar,
}

/// KPEP database (size: 144/80 bytes on 64/32 bit OS)
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct kpep_db {
    /// Database name, such as "haswell".
    pub name: *const c_char,
    /// Plist name, such as "cpu_7_8_10b282dc".
    pub cpu_id: *const c_char,
    /// Marketing name, such as "Intel Haswell".
    pub marketing_name: *const c_char,
    /// Plist data (CFDataRef), currently NULL.
    pub plist_data: *mut c_void,
    /// All events (CFDict<CFSTR(event_name), kpep_event *>).
    pub event_map: *mut c_void,
    /// Event struct buffer (sizeof(kpep_event) * events_count).
    pub event_arr: *mut kpep_event,
    /// Fixed counter events (sizeof(kpep_event *)
    pub fixed_event_arr: *mut *mut kpep_event,
    /// All aliases (CFDict<CFSTR(event_name), kpep_event *>).
    pub alias_map: *mut c_void,
    pub reserved_1: size_t,
    pub reserved_2: size_t,
    pub reserved_3: size_t,
    /// All events count.
    pub event_count: size_t,
    pub alias_count: size_t,
    pub fixed_counter_count: size_t,
    pub config_counter_count: size_t,
    pub power_counter_count: size_t,
    /// see `KPEP CPU archtecture constants` above.
    pub archtecture: c_uint,
    pub fixed_counter_bits: c_uint,
    pub config_counter_bits: c_uint,
    pub power_counter_bits: c_uint,
}

/// KPEP config (size: 80/44 bytes on 64/32 bit OS)
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct kpep_config {
    pub db: *mut kpep_db,
    /// (sizeof(kpep_event *) * counter_count), init NULL
    pub ev_arr: *mut *mut kpep_event,
    /// (sizeof(size_t *) * counter_count), init 0
    pub ev_map: *mut size_t,
    /// (sizeof(size_t *) * counter_count), init -1
    pub ev_idx: *mut size_t,
    /// (sizeof(c_uint *) * counter_count), init 0
    pub flags: *mut c_uint,
    /// (sizeof(c_ulonglong *) * counter_count), init 0
    pub kpc_periods: *mut c_ulonglong,
    /// kpep_config_events_count()
    pub event_count: size_t,
    pub counter_count: size_t,
    /// See `class mask constants` above.
    pub classes: c_uint,
    pub config_counter: c_uint,
    pub power_counter: c_uint,
    pub reserved: c_uint,
}
