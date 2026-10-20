use serde::Deserialize;

const EMBEDDED: &str = include_str!("../config/default.yaml");

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub lineage: LineageConfig,
    pub priority: PriorityConfig,
    pub tcc: TccConfig,
    pub fda: FdaConfig,
    pub ax: AxConfig,
    pub devtools: DevToolsConfig,
    pub cli: CliConfig,
    pub exit: ExitConfig,
    pub messages: Messages,
    pub log: LogConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LineageConfig {
    pub id: String,
    pub name: String,
    pub library: String,
    pub symbol: String,
    pub dlopen_mode: i32,
    pub shortbsdinfo_flavor: i32,
    pub path_max: usize,
    pub root_pid: i32,
    pub max_depth: usize,
    pub separator: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PriorityConfig {
    pub id: String,
    pub name: String,
    pub listthreads_flavor: i32,
    pub threadinfo_flavor: i32,
    pub max_threads: usize,
    pub assign: String,
    pub extra: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TccConfig {
    pub id: String,
    pub name: String,
    pub framework: String,
    pub symbol: String,
    pub service: String,
    pub dlopen_mode: i32,
    pub cf_encoding: u32,
    pub granted: Vec<i32>,
    pub denied: Vec<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FdaConfig {
    pub id: String,
    pub name: String,
    pub path: String,
    pub denied_errno: Vec<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AxConfig {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DevToolsConfig {
    pub id: String,
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub enabled: String,
    pub disabled: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CliConfig {
    pub version_flags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExitConfig {
    pub ok: u8,
    pub granted: u8,
    pub denied: u8,
    pub unknown: u8,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Messages {
    pub granted: String,
    pub denied: String,
    pub unknown: String,
    pub separator: String,
    pub usage: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    pub level_env: String,
    pub default_level: String,
}

pub fn parse(text: &str) -> Result<Config, serde_yaml_ng::Error> {
    serde_yaml_ng::from_str(text)
}

pub fn embedded() -> Config {
    parse(EMBEDDED).expect("embedded config")
}

#[cfg(test)]
#[path = "config_test.rs"]
mod tests;
