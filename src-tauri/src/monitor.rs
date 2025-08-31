use serde::Serialize;
use std::sync::Mutex;
use sysinfo::{NetworkExt, NetworksExt, System, SystemExt};
use tauri::Manager;

#[derive(Debug, Serialize, Clone)]
pub struct CpuInfo {
    pub(crate) usage: f32,
}

#[derive(Debug, Serialize, Clone)]
pub struct MemoryInfo {
    pub(crate) total: u64,
    pub(crate) used: u64,
    pub(crate) used_percent: f32,
}

#[derive(Debug, Serialize, Clone)]
pub struct NetworkInfo {
    pub(crate) rx_bytes: u64,
    pub(crate) tx_bytes: u64,
    pub(crate) rx_speed: u64,
    pub(crate) tx_speed: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct SystemInfo {
    pub(crate) cpu: CpuInfo,
    pub(crate) memory: MemoryInfo,
    pub(crate) network: NetworkInfo,
}

pub struct NetworkStats {
    pub(crate) rx_bytes: u64,
    pub(crate) tx_bytes: u64,
}

pub struct AppState {
    pub(crate) sys: Mutex<System>,
    pub(crate) last_network_stats: Mutex<Option<NetworkStats>>,
}
