#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeError {
    InvalidCString,
    LoadFailed,
    SymbolMissing,
    CfStringFailed,
    SpawnFailed,
    ExitFailed,
}
