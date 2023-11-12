use libc::{c_char, c_uint, c_uchar, c_void, size_t, c_ulonglong};

#[allow(non_camel_case_types)]
pub type kpc_config_t = u64;

/// KPEP event (size: 48/28 bytes on 64/32 bit OS)
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct kpep_event {
    name: *const c_char, ///< Unique name of a event, such as "INST_RETIRED.ANY".
    description: *const c_char, ///< Description for this event.
    errata: *const c_char,      ///< Errata, currently NULL.
    alias: *const c_char,       ///< Alias name, such as "Instructions", "Cycles".
    fallback: *const c_char,    ///< Fallback event name for fixed counter.
    mask: c_uint,
    number: c_uchar,
    umask: c_uchar,
    reserved: c_uchar,
    is_fixed: c_uchar,
}

/// KPEP database (size: 144/80 bytes on 64/32 bit OS)
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct kpep_db {
    name: *const c_char,           ///< Database name, such as "haswell".
    cpu_id: *const c_char,         ///< Plist name, such as "cpu_7_8_10b282dc".
    marketing_name: *const c_char, ///< Marketing name, such as "Intel Haswell".
    plist_data: *mut c_void,           ///< Plist data (CFDataRef), currently NULL.
    event_map: *mut c_void, ///< All events (CFDict<CFSTR(event_name), kpep_event *>).
    event_arr: *mut kpep_event, ///< Event struct buffer (sizeof(kpep_event) * events_count).
    fixed_event_arr: *mut *mut kpep_event, ///< Fixed counter events (sizeof(kpep_event *)
    ///< * fixed_counter_count)
    alias_map: *mut c_void, ///< All aliases (CFDict<CFSTR(event_name), kpep_event *>).
    reserved_1: size_t,
    reserved_2: size_t,
    reserved_3: size_t,
    event_count: size_t, ///< All events count.
    alias_count: size_t,
    fixed_counter_count: size_t,
    config_counter_count: size_t,
    power_counter_count: size_t,
    archtecture: c_uint, ///< see `KPEP CPU archtecture constants` above.
    fixed_counter_bits: c_uint,
    config_counter_bits: c_uint,
    power_counter_bits: c_uint,
}

/// KPEP config (size: 80/44 bytes on 64/32 bit OS)
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct kpep_config {
    db: *mut kpep_db,
    ev_arr: *mut *mut kpep_event, ///< (sizeof(kpep_event *) * counter_count), init NULL
    ev_map: *mut size_t,       ///< (sizeof(size_t *) * counter_count), init 0
    ev_idx: *mut size_t,       ///< (sizeof(size_t *) * counter_count), init -1
    flags: *mut c_uint,          ///< (sizeof(c_uint *) * counter_count), init 0
    kpc_periods: *mut c_ulonglong,    ///< (sizeof(c_ulonglong *) * counter_count), init 0
    event_count: size_t,   /// kpep_config_events_count()
    counter_count: size_t,
    classes: c_uint, ///< See `class mask constants` above.
    config_counter: c_uint,
    power_counter: c_uint,
    reserved: c_uint,
}
