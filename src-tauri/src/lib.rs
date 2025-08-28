use std::sync::Mutex;
use tauri::{Emitter, Manager};
use sysinfo::{CpuExt, NetworkExt, NetworksExt, System, SystemExt};
use crate::monitor::{AppState, CpuInfo, MemoryInfo, NetworkInfo, NetworkStats, SystemInfo};

mod monitor;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    let x = "ddd";
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn amy(name: &str) -> String {
    let x = "ddd";
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run12() {
    println!("Running from run12 function");
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let sys = System::new_all();

    tauri::Builder::default()
        .manage(crate::monitor::AppState {
            sys: Mutex::new(sys),
            last_network_stats: Mutex::new(None),
        })
        .setup(|app| {
            let app_handle = app.handle();
            println!("sadffsdsfd");
            println!("App name: {}", app_handle.package_info().name);

            let handle = app_handle.clone();

            // 启动监控线程，每秒发送一次系统信息
            tauri::async_runtime::spawn(async move {
                loop {
                    let app_state = handle.state::<AppState>();
                    let mut sys = app_state.sys.lock().unwrap();
                    sys.refresh_all();

                    // 获取CPU使用率
                    let cpu_usage = sys.global_cpu_info().cpu_usage();

                    // 获取内存信息
                    let total_memory = sys.total_memory();
                    let used_memory = total_memory - sys.available_memory();
                    let used_memory_percent = (used_memory as f32 / total_memory as f32) * 100.0;

                    // 获取网络信息
                    let networks = sys.networks();
                    let mut rx_bytes = 0;
                    let mut tx_bytes = 0;

                    for (_name, data) in networks {
                        rx_bytes += data.received();
                        tx_bytes += data.transmitted();
                    }

                    // 计算网络速度
                    let mut last_stats = app_state.last_network_stats.lock().unwrap();
                    let (rx_speed, tx_speed) = match *last_stats {
                        Some(ref stats) => {
                            (rx_bytes.saturating_sub(stats.rx_bytes), tx_bytes.saturating_sub(stats.tx_bytes))
                        }
                        None => (0, 0),
                    };

                    *last_stats = Some(NetworkStats { rx_bytes, tx_bytes });

                    // 构建系统信息
                    let system_info = SystemInfo {
                        cpu: CpuInfo { usage: cpu_usage },
                        memory: MemoryInfo {
                            total: total_memory,
                            used: used_memory,
                            used_percent: used_memory_percent,
                        },
                        network: NetworkInfo {
                            rx_bytes,
                            tx_bytes,
                            rx_speed,
                            tx_speed,
                        },
                    };

                    // 发送事件到前端
                    handle.emit("system-info", system_info).unwrap();

                    // 每秒更新一次
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            });
            // setup code here
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");



}
