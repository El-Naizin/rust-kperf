use libc::{c_char, c_uint, c_uchar, c_void, size_t, c_ulonglong};

#[allow(non_camel_case_types)]
pub type kpc_config_t = u64;

/// KPEP event (size: 48/28 bytes on 64/32 bit OS)
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct kpep_event {
    pub name: *const c_char, ///< Unique name of a event, such as "INST_RETIRED.ANY".
    pub description: *const c_char, ///< Description for this event.
    pub errata: *const c_char,      ///< Errata, currently NULL.
    pub alias: *const c_char,       ///< Alias name, such as "Instructions", "Cycles".
    pub fallback: *const c_char,    ///< Fallback event name for fixed counter.
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
    pub name: *const c_char,           ///< Database name, such as "haswell".
    pub cpu_id: *const c_char,         ///< Plist name, such as "cpu_7_8_10b282dc".
    pub marketing_name: *const c_char, ///< Marketing name, such as "Intel Haswell".
    pub plist_data: *mut c_void,           ///< Plist data (CFDataRef), currently NULL.
    pub event_map: *mut c_void, ///< All events (CFDict<CFSTR(event_name), kpep_event *>).
    pub event_arr: *mut kpep_event, ///< Event struct buffer (sizeof(kpep_event) * events_count).
    pub fixed_event_arr: *mut *mut kpep_event, ///< Fixed counter events (sizeof(kpep_event *)
    ///< * fixed_counter_count)
    pub alias_map: *mut c_void, ///< All aliases (CFDict<CFSTR(event_name), kpep_event *>).
    pub reserved_1: size_t,
    pub reserved_2: size_t,
    pub reserved_3: size_t,
    pub event_count: size_t, ///< All events count.
    pub alias_count: size_t,
    pub fixed_counter_count: size_t,
    pub config_counter_count: size_t,
    pub power_counter_count: size_t,
    pub archtecture: c_uint, ///< see `KPEP CPU archtecture constants` above.
    pub fixed_counter_bits: c_uint,
    pub config_counter_bits: c_uint,
    pub power_counter_bits: c_uint,
}

/// KPEP config (size: 80/44 bytes on 64/32 bit OS)
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct kpep_config {
    pub db: *mut kpep_db,
    pub ev_arr: *mut *mut kpep_event, ///< (sizeof(kpep_event *) * counter_count), init NULL
    pub ev_map: *mut size_t,       ///< (sizeof(size_t *) * counter_count), init 0
    pub ev_idx: *mut size_t,       ///< (sizeof(size_t *) * counter_count), init -1
    pub flags: *mut c_uint,          ///< (sizeof(c_uint *) * counter_count), init 0
    pub kpc_periods: *mut c_ulonglong,    ///< (sizeof(c_ulonglong *) * counter_count), init 0
    pub event_count: size_t,   /// kpep_config_events_count()
    pub counter_count: size_t,
    pub classes: c_uint, ///< See `class mask constants` above.
    pub config_counter: c_uint,
    pub power_counter: c_uint,
    pub reserved: c_uint,
}
