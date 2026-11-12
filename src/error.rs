#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeError {
    InvalidCString,
    LoadFailed,
    SymbolMissing,
    SpawnFailed,
    ResponsibleFailed,
    ProcInfoFailed,
    PathFailed,
    ThreadsTruncated,
}
