use libc::{c_int, size_t, c_uint, c_ulonglong, c_char, c_uchar};
use crate::structs::{kpep_db, kpep_config, kpep_event, kpc_config_t};

#[link(name="kperf", kind="framework")]
extern "C" {
    /// Print current CPU identification string to the buffer (same as snprintf),
    /// such as "cpu_7_8_10b282dc_46". This string can be used to locate the PMC
    /// database in /usr/share/kpep.
    /// @return string's length, or negative value if error occurs.
    /// @note This method does not requires root privileges.
    /// @details sysctl get(hw.cputype), get(hw.cpusubtype),
    ///                 get(hw.cpufamily), get(machdep.cpu.model)
    pub fn kpc_cpu_string(buf: *mut u8, buf_size: size_t) -> c_int;
    /// Get the version of KPC that's being run.
    /// @return See `PMU version constants` above.
    /// @details sysctl get(kpc.pmu_version)
    pub fn kpc_pmu_version() -> c_uint;
    /// Get running PMC classes.
    /// @return See `class mask constants` above,
    ///         0 if error occurs or no class is set.
    /// @details sysctl get(kpc.counting)
    pub fn kpc_get_counting() -> c_uint;
    /// Set PMC classes to enable counting.
    /// @param classes See `class mask constants` above, set 0 to shutdown counting.
    /// @return 0 for success.
    /// @details sysctl set(kpc.counting)
    pub fn kpc_set_counting(classes: c_uint) -> c_int;
    /// Get running PMC classes for current thread.
    /// @return See `class mask constants` above,
    ///         0 if error occurs or no class is set.
    /// @details sysctl get(kpc.thread_counting)
    pub fn kpc_get_thread_counting() -> c_uint;
    /// Set PMC classes to enable counting for current thread.
    /// @param classes See `class mask constants` above, set 0 to shutdown counting.
    /// @return 0 for success.
    /// @details sysctl set(kpc.thread_counting)
    pub fn kpc_set_thread_counting(classes: c_uint) -> c_int;
    /// Get how many config registers there are for a given mask.
    /// For example: Intel may returns 1 for `KPC_CLASS_FIXED_MASK`,
    ///                        returns 4 for `KPC_CLASS_CONFIGURABLE_MASK`.
    /// @param classes See `class mask constants` above.
    /// @return 0 if error occurs or no class is set.
    /// @note This method does not requires root privileges.
    /// @details sysctl get(kpc.config_count)
    pub fn kpc_get_config_count(classes: c_uint) -> c_uint;
    /// Get config registers.
    /// @param classes see `class mask constants` above.
    /// @param config Config buffer to receive values, should not smaller than
    ///               kpc_get_config_count(classes) * sizeof(kpc_config_t).
    /// @return 0 for success.
    /// @details sysctl get(kpc.config_count), get(kpc.config)
    pub fn kpc_get_config(classes: c_uint, config: *mut kpc_config_t) -> c_int;
    /// Set config registers.
    /// @param classes see `class mask constants` above.
    /// @param config Config buffer, should not smaller than
    ///               kpc_get_config_count(classes) * sizeof(kpc_config_t).
    /// @return 0 for success.
    /// @details sysctl get(kpc.config_count), set(kpc.config)
    pub fn kpc_set_config(classes: c_uint, config: *mut kpc_config_t) -> c_int;
    /// Get how many counters there are for a given mask.
    /// For example: Intel may returns 3 for `KPC_CLASS_FIXED_MASK`,
    ///                        returns 4 for `KPC_CLASS_CONFIGURABLE_MASK`.
    /// @param classes See `class mask constants` above.
    /// @note This method does not requires root privileges.
    /// @details sysctl get(kpc.counter_count)
    pub fn kpc_get_counter_count(classes: c_uint) -> c_uint;
    /// Get counter accumulations.
    /// If `all_cpus` is true, the buffer count should not smaller than
    /// (cpu_count * counter_count). Otherwize, the buffer count should not smaller
    /// than (counter_count).
    /// @see kpc_get_counter_count(), kpc_cpu_count().
    /// @param all_cpus true for all CPUs, false for current cpu.
    /// @param classes See `class mask constants` above.
    /// @param curcpu A pointer to receive current cpu id, can be NULL.
    /// @param buf Buffer to receive counter's value.
    /// @return 0 for success.
    /// @details sysctl get(hw.ncpu), get(kpc.counter_count), get(kpc.counters)
    pub fn kpc_get_cpu_counters(all_cpus: bool, classes: c_uint, curcpu: *mut c_int, buf: *mut c_ulonglong) -> c_int;
    /// Get counter accumulations for current thread.
    /// @param tid Thread id, should be 0.
    /// @param buf_count The number of buf's elements (not bytes),
    ///                  should not smaller than kpc_get_counter_count().
    /// @param buf Buffer to receive counter's value.
    /// @return 0 for success.
    /// @details sysctl get(kpc.thread_counters)
    pub fn kpc_get_thread_counters(tid: c_uint, buf_count: c_uint, buf: *mut c_ulonglong) -> c_int;
    /// Acquire/release the counters used by the Power Manager.
    /// @param val 1:acquire, 0:release
    /// @return 0 for success.
    /// @details sysctl set(kpc.force_all_ctrs)
    pub fn kpc_force_all_ctrs_set(val: c_int) -> c_int;
    /// Get the state of all_ctrs.
    /// @return 0 for success.
    /// @details sysctl get(kpc.force_all_ctrs)
    pub fn kpc_force_all_ctrs_get(val_out: *mut c_int) -> c_int;
    /// Set number of actions, should be `KPERF_ACTION_MAX`.
    /// @details sysctl set(kperf.action.count)
    pub fn kperf_action_count_set(count: c_uint) -> c_int;
    /// Get number of actions.
    /// @details sysctl get(kperf.action.count)
    pub fn kperf_action_count_get(count: *mut c_uint) -> c_int;
    /// Set what to sample when a trigger fires an action, e.g. `KPERF_SAMPLER_PMC_CPU`.
    /// @details sysctl set(kperf.action.samplers)
    pub fn kperf_action_samplers_set(actionid: c_uint, sample: c_uint) -> c_int;
    /// Get what to sample when a trigger fires an action.
    /// @details sysctl get(kperf.action.samplers)
    pub fn kperf_action_samplers_get(actionid: c_uint, sample: *mut c_uint) -> c_int;
    /// Apply a task filter to the action, -1 to disable filter.
    /// @details sysctl set(kperf.action.filter_by_task)
    pub fn kperf_action_filter_set_by_task(actionid: c_uint, port: c_int) -> c_int;
    /// Apply a pid filter to the action, -1 to disable filter.
    /// @details sysctl set(kperf.action.filter_by_pid)
    pub fn kperf_action_filter_set_by_pid(actionid: c_uint, pid: c_int) -> c_int;
    /// Set number of time triggers, should be `KPERF_TIMER_MAX`.
    /// @details sysctl set(kperf.timer.count)
    pub fn kperf_timer_count_set(count: c_uint) -> c_int;
    /// Get number of time triggers.
    /// @details sysctl get(kperf.timer.count)
    pub fn kperf_timer_count_get(count: *mut c_uint) -> c_int;
    /// Set timer number and period.
    /// @details sysctl set(kperf.timer.period)
    pub fn kperf_timer_period_set(actionid: c_uint, tick: c_ulonglong) -> c_int;
    /// Get timer number and period.
    /// @details sysctl get(kperf.timer.period)
    pub fn kperf_timer_period_get(actionid: c_uint, tick: *mut c_ulonglong) -> c_int;
    /// Set timer number and actionid.
    /// @details sysctl set(kperf.timer.action)
    pub fn kperf_timer_action_set(actionid: c_uint, timerid: c_uint) -> c_int;
    /// Get timer number and actionid.
    /// @details sysctl get(kperf.timer.action)
    pub fn kperf_timer_action_get(actionid: c_uint, timerid: *mut c_uint) -> c_int;
    /// Set which timer ID does PET (Profile Every Thread).
    /// @details sysctl set(kperf.timer.pet_timer)
    pub fn kperf_timer_pet_set(timerid: c_uint) -> c_int;
    /// Get which timer ID does PET (Profile Every Thread).
    /// @details sysctl get(kperf.timer.pet_timer)
    pub fn kperf_timer_pet_get(timerid: *mut c_uint) -> c_int;
    /// Enable or disable sampling.
    /// @details sysctl set(kperf.sampling)
    pub fn kperf_sample_set(enabled: c_uint) -> c_int;
    /// Get is currently sampling.
    /// @details sysctl get(kperf.sampling)
    pub fn kperf_sample_get(enabled: *mut c_uint) -> c_int;
    /// Reset kperf: stop sampling, kdebug, timers and actions.
    /// @return 0 for success.
    pub fn kperf_reset() -> c_int;
    /// Nanoseconds to CPU ticks.
    pub fn kperf_ns_to_ticks(ns: c_ulonglong) -> c_ulonglong;
    /// CPU ticks to nanoseconds.
    pub fn kperf_ticks_to_ns(ticks: c_ulonglong) -> c_ulonglong;
    /// CPU ticks frequency (mach_absolute_time).
    pub fn kperf_tick_frequency() -> c_ulonglong;
}

//TODO: include those two functions to the lib
// /// Get lightweight PET mode (not in kperf.framework).
// static int kperf_lightweight_pet_get(u32 *enabled) {
// if (!enabled) return -1;
// usize size = 4;
// return sysctlbyname("kperf.lightweight_pet", enabled, &size, NULL, 0);
// }
//
// /// Set lightweight PET mode (not in kperf.framework).
// static int kperf_lightweight_pet_set(u32 enabled) {
// return sysctlbyname("kperf.lightweight_pet", NULL, NULL, &enabled, 4);
// }


#[link(name="kperfdata", kind="framework")]
extern "C" {
    /// Create a config.
    /// @param db A kpep db, see kpep_db_create()
    /// @param cfg_ptr A pointer to receive the new config.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_config_create (db: *mut kpep_db, cfg_ptr: *mut *mut kpep_config) -> c_int;

    /// Free the config.
    pub fn kpep_config_free (cfg: *mut kpep_config);

    /// Add an event to config.
    /// @param cfg The config.
    /// @param ev_ptr A event pointer.
    /// @param flag 0: all, 1: user space only
    /// @param err Error bitmap pointer, can be NULL.
    ///            If return value is `CONFLICTING_EVENTS`, this bitmap contains
    ///            the conflicted event indices, e.g. "1 << 2" means index 2.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_config_add_event (cfg: *mut kpep_config, ev_ptr: *mut *mut kpep_event, flag: c_uint, err: *mut c_uint) -> c_int;

    /// Remove event at index.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_config_remove_event (cfg: *mut kpep_config, idx: size_t) -> c_int;

    /// Force all counters.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_config_force_counters (cfg: *mut kpep_config) -> c_int;

    /// Get events count.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_config_events_count (cfg: *mut kpep_config, count_ptr: *mut size_t) -> c_int;

    /// Get all event pointers.
    /// @param buf A buffer to receive event pointers.
    /// @param buf_size The buffer's size in bytes, should not smaller than
    ///                 kpep_config_events_count() * sizeof(void *).
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_config_events (cfg: *mut kpep_config, buf: *mut *mut kpep_event, buf_size: size_t) -> c_int;

    /// Get kpc register configs.
    /// @param buf A buffer to receive kpc register configs.
    /// @param buf_size The buffer's size in bytes, should not smaller than
    ///                 kpep_config_kpc_count() * sizeof(kpc_config_t).
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_config_kpc (cfg: *mut kpep_config, buf: *mut kpc_config_t, buf_size: size_t) -> c_int;

    /// Get kpc register config count.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_config_kpc_count (cfg: *mut kpep_config, count_ptr: *mut size_t) -> c_int;

    /// Get kpc classes.
    /// @param classes See `class mask constants` above.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_config_kpc_classes (cfg: *mut kpep_config, classes_ptr: *mut c_uint) -> c_int;

    /// Get the index mapping from event to counter.
    /// @param buf A buffer to receive indexes.
    /// @param buf_size The buffer's size in bytes, should not smaller than
    ///                 kpep_config_events_count() * sizeof(kpc_config_t).
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_config_kpc_map (cfg: *mut kpep_config, buf: *mut size_t, buf_size: size_t) -> c_int;

    /// Open a kpep database file in "/usr/share/kpep/" or "/usr/local/share/kpep/".
    /// @param name File name, for example "haswell", "cpu_100000c_1_92fb37c8".
    ///             Pass NULL for current CPU.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_db_create (name: *const c_char, db_ptr: *mut *mut kpep_db) -> c_int;

    /// Free the kpep database.
    pub fn kpep_db_free (db: *mut kpep_db);

    /// Get the database's name.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_db_name (db: *mut kpep_db, name: *const *mut c_char) -> c_int;

    /// Get the event alias count.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_db_aliases_count (db: *mut kpep_db, count: *mut size_t) -> c_int;

    /// Get all alias.
    /// @param buf A buffer to receive all alias strings.
    /// @param buf_size The buffer's size in bytes,
    ///        should not smaller than kpep_db_aliases_count() * sizeof(void *).
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_db_aliases (db: *mut kpep_db, buf: *const *mut c_char, buf_size: size_t) -> c_int;

    /// Get counters count for given classes.
    /// @param classes 1: Fixed, 2: Configurable.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_db_counters_count (db: *mut kpep_db, classes: c_uchar, count: *mut size_t) -> c_int;

    /// Get all event count.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_db_events_count (db: *mut kpep_db, count: *mut size_t) -> c_int;

    /// Get all events.
    /// @param buf A buffer to receive all event pointers.
    /// @param buf_size The buffer's size in bytes,
    ///        should not smaller than kpep_db_events_count() * sizeof(void *).
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_db_events (db: *mut kpep_db, buf: *mut *mut kpep_event, buf_size: size_t) -> c_int;

    /// Get one event by name.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_db_event (db: *mut kpep_db, name: *const c_char, ev_ptr: *mut *mut kpep_event) -> c_int;

    /// Get event's name.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_event_name (ev: *mut kpep_event, name_ptr: *const *mut c_char) -> c_int;

    /// Get event's alias.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_event_alias (ev: *mut kpep_event, alias_ptr: *const *mut c_char) -> c_int;

    /// Get event's description.
    /// @return kpep_config_error_code, 0 for success.
    pub fn kpep_event_description (ev: *mut kpep_event, str_ptr: *const *mut c_char) -> c_int;
}
