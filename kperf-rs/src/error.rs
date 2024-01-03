#[derive(Debug, Copy, Clone)]
pub enum KpepError {
    UnknownError,
}

#[derive(Debug, Clone)]
pub enum KperfError {
    UnknownError(String),
    PermissionDenied,
    PerfCounterBuildError(String),
}
