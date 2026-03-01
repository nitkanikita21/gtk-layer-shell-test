use once_cell::sync::Lazy;
use serde::Serialize;
use std::{
  sync::RwLock,
  time::{Duration, Instant},
};
use sysinfo::{Disks, Networks, System};
use tauri::command;
use tokio::time::sleep;

#[derive(Serialize, Clone)]
pub struct SystemInfo {
  pub cpu_used: f32,
  pub disk_used: u64,
  pub ram_used: u64,
  pub net_both: u64,
  pub net_in: u64,
  pub net_out: u64,
}

// Глобальний System (оновлюється у фоні)
static GLOBAL_SYS: Lazy<RwLock<System>> = Lazy::new(|| {
  let mut sys = System::new();
  sys.refresh_all();
  RwLock::new(sys)
});

// Глобальний Networks (оновлюється при кожному виклику)
static GLOBAL_NETWORKS: Lazy<RwLock<Networks>> = Lazy::new(|| {
  let mut nets = Networks::new();
  nets.refresh(true);
  RwLock::new(nets)
});

// Стан для обчислення швидкості мережі зі згладжуванням
struct NetState {
  prev_in: u64, // попереднє значення лічильника (байти)
  prev_out: u64,
  prev_time: Instant, // час попереднього виміру
  smoothed_in: f64,   // згладжене значення вхідної швидкості (байт/сек)
  smoothed_out: f64,
  initialized: bool, // чи є попередні дані для обчислення
}

static NET_STATE: Lazy<RwLock<NetState>> = Lazy::new(|| {
  RwLock::new(NetState {
    prev_in: 0,
    prev_out: 0,
    prev_time: Instant::now(),
    smoothed_in: 0.0,
    smoothed_out: 0.0,
    initialized: false,
  })
});

// Запуск фонових задач: тільки оновлення CPU, RAM (мережа оновлюється в get_system_info)
pub async fn start_background_tasks() {
  loop {
    {
      let mut sys = GLOBAL_SYS.write().unwrap();
      sys.refresh_all();
    }
    sleep(Duration::from_millis(500)).await;
  }
}

#[command]
pub fn get_system_info() -> anyhow::Result<SystemInfo, anyhow_serde::Error> {
  let sys = GLOBAL_SYS.read().unwrap();

  // Диск
  let disk_used = Disks::new_with_refreshed_list()
    .first()
    .map(|d| d.total_space() - d.available_space())
    .unwrap_or(0);

  // Мережа — обчислюємо згладжену швидкість
  let (net_in, net_out) = calculate_smoothed_net_speed();

  println!("both{} in{} out{}", net_in + net_out, net_in, net_out);

  Ok(SystemInfo {
    cpu_used: sys.global_cpu_usage(),
    disk_used,
    ram_used: sys.used_memory(),
    net_in,
    net_out,
    net_both: net_in + net_out,
  })
}

// Обчислення згладженої швидкості мережі (байт/сек) з експоненційним ковзним середнім
fn calculate_smoothed_net_speed() -> (u64, u64) {
  // Оновлюємо мережеві лічильники
  let mut nets = GLOBAL_NETWORKS.write().unwrap();
  nets.refresh(true);

  // Знаходимо Ethernet-інтерфейс (en* або eth*)
  let iface = nets
    .keys()
    .find(|name| name.starts_with("en") || name.starts_with("eth"))
    .cloned();

  let (current_in, current_out) = if let Some(ref name) = iface {
    if let Some(data) = nets.get(name) {
      (data.received(), data.transmitted())
    } else {
      (0, 0)
    }
  } else {
    (0, 0)
  };

  let now = Instant::now();

  // Оновлюємо стан і обчислюємо згладжену швидкість
  let mut state = NET_STATE.write().unwrap();

  let (smoothed_in, smoothed_out) = if state.initialized {
    let delta_secs = (now - state.prev_time).as_secs_f64().max(0.001);
    let delta_in = current_in.saturating_sub(state.prev_in);
    let delta_out = current_out.saturating_sub(state.prev_out);

    // Миттєва швидкість
    let raw_in = (delta_in as f64 / delta_secs) as f64;
    let raw_out = (delta_out as f64 / delta_secs) as f64;

    // Експоненційне ковзне середнє (EMA)
    // Фактор згладжування α = 0.3 (новий вимір важить 30%)
    const ALPHA: f64 = 0.3;
    let new_smoothed_in = ALPHA * raw_in + (1.0 - ALPHA) * state.smoothed_in;
    let new_smoothed_out = ALPHA * raw_out + (1.0 - ALPHA) * state.smoothed_out;

    (new_smoothed_in, new_smoothed_out)
  } else {
    // Перший вимір — ініціалізація (повертаємо 0)
    (0.0, 0.0)
  };

  // Оновлюємо стан для наступного виклику
  state.prev_in = current_in;
  state.prev_out = current_out;
  state.prev_time = now;
  state.smoothed_in = smoothed_in;
  state.smoothed_out = smoothed_out;
  state.initialized = true;

  (smoothed_in as u64, smoothed_out as u64)
}
