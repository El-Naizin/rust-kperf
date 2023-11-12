/// C Style enum in rust
/// Can't use an actual rust enum, read this: (https://mdaverde.com/posts/rust-bindgen-enum/)
pub mod kpep_config_error_code {
    pub type Type = ::std::os::raw::c_uint;
    pub const KPEP_CONFIG_ERROR_NONE: Type = 0;
    pub const KPEP_CONFIG_ERROR_INVALID_ARGUMENT: Type = 1;
    pub const KPEP_CONFIG_ERROR_OUT_OF_MEMORY: Type = 2;
    pub const KPEP_CONFIG_ERROR_IO: Type = 3;
    pub const KPEP_CONFIG_ERROR_BUFFER_TOO_SMALL: Type = 4;
    pub const KPEP_CONFIG_ERROR_CUR_SYSTEM_UNKNOWN: Type = 5;
    pub const KPEP_CONFIG_ERROR_DB_PATH_INVALID: Type = 6;
    pub const KPEP_CONFIG_ERROR_DB_NOT_FOUND: Type = 7;
    pub const KPEP_CONFIG_ERROR_DB_ARCH_UNSUPPORTED: Type = 8;
    pub const KPEP_CONFIG_ERROR_DB_VERSION_UNSUPPORTED: Type = 9;
    pub const KPEP_CONFIG_ERROR_DB_CORRUPT: Type = 10;
    pub const KPEP_CONFIG_ERROR_EVENT_NOT_FOUND: Type = 11;
    pub const KPEP_CONFIG_ERROR_CONFLICTING_EVENTS: Type = 12;
    pub const KPEP_CONFIG_ERROR_COUNTERS_NOT_FORCED: Type = 13;
    pub const KPEP_CONFIG_ERROR_EVENT_UNAVAILABLE: Type = 14;
    pub const KPEP_CONFIG_ERROR_ERRNO: Type = 15;
    pub const KPEP_CONFIG_ERROR_MAX: Type = 16;
}

// Cross-platform class constants
pub const KPC_CLASS_FIXED: u32 = 0;
pub const KPC_CLASS_CONFIGURABLE: u32 = 1;
pub const KPC_CLASS_POWER: u32 = 2;
pub const KPC_CLASS_RAWPMU: u32 = 3;

// Cross-platform class mask constants
pub const KPC_CLASS_FIXED_MASK: u32         = 1 << KPC_CLASS_FIXED;
pub const KPC_CLASS_CONFIGURABLE_MASK: u32  = 1 << KPC_CLASS_CONFIGURABLE;
pub const KPC_CLASS_POWER_MASK: u32         = 1 << KPC_CLASS_POWER;
pub const KPC_CLASS_RAWPMU_MASK: u32        = 1 << KPC_CLASS_RAWPMU;

// PMU version constants
pub const KPC_PMU_ERROR: u32 = 0;     // Error
pub const KPC_PMU_INTEL_V3: u32 = 1;  // Intel
pub const KPC_PMU_ARM_APPLE: u32 = 2; // ARM64
pub const KPC_PMU_INTEL_V2: u32 = 3;  // Old intel
pub const KPC_PMU_ARM_V2: u32 = 4;    // Old ARM

// The maximum number of counters we could read from every class in one go.
// ARMV7: FIXED: 1, CONFIGURABLE: 4
// ARM32: FIXED: 2, CONFIGURABLE: 6
// ARM64: FIXED: 2, CONFIGURABLE: CORE_NCTRS - FIXED (6 or 8)
// x86: 32
pub const KPC_MAX_COUNTERS: u32 = 32;

// Bits for defining what to do on an action.
// Defined in https://github.com/apple/darwin-xnu/blob/main/osfmk/kperf/action.h
pub const KPERF_SAMPLER_TH_INFO: u32       = 1 << 0;
pub const KPERF_SAMPLER_TH_SNAPSHOT: u32   = 1 << 1;
pub const KPERF_SAMPLER_KSTACK: u32        = 1 << 2;
pub const KPERF_SAMPLER_USTACK: u32        = 1 << 3;
pub const KPERF_SAMPLER_PMC_THREAD: u32    = 1 << 4;
pub const KPERF_SAMPLER_PMC_CPU: u32       = 1 << 5;
pub const KPERF_SAMPLER_PMC_CONFIG: u32    = 1 << 6;
pub const KPERF_SAMPLER_MEMINFO: u32       = 1 << 7;
pub const KPERF_SAMPLER_TH_SCHEDULING: u32 = 1 << 8;
pub const KPERF_SAMPLER_TH_DISPATCH: u32   = 1 << 9;
pub const KPERF_SAMPLER_TK_SNAPSHOT: u32   = 1 << 10;
pub const KPERF_SAMPLER_SYS_MEM: u32       = 1 << 11;
pub const KPERF_SAMPLER_TH_INSCYC: u32     = 1 << 12;
pub const KPERF_SAMPLER_TK_INFO: u32       = 1 << 13;

// Maximum number of kperf action ids.
pub const KPERF_ACTION_MAX: u32 = 32;

// Maximum number of kperf timer ids.
pub const KPERF_TIMER_MAX: u32 = 8;

// KPEP CPU archtecture constants.
pub const KPEP_ARCH_I386: u32 = 0;
pub const KPEP_ARCH_X86_64: u32 = 1;
pub const KPEP_ARCH_ARM: u32 = 2;
pub const KPEP_ARCH_ARM64: u32 = 3;

