pub struct CpuInfo {
    pub model_name: String,
    pub cpu_cores: u32,
}

pub struct MemInfo {
    pub mem_total: f64,
    pub mem_free: f64,
    pub cached: f64,
    pub swap_total: f64,
    pub swap_free: f64,
}

pub struct Output {
    pub uname: String,
    pub hostname: String,
    pub os_pretty_name: String,
    pub uptime_seconds: String,
    pub current_charge: String,
    pub battery_status: String,
    pub cpu_info: CpuInfo,
    pub mem_info: MemInfo,
    pub displays: Vec<String>,
    pub shell_info: Option<(String, Option<String>)>,
    pub packages_info: Vec<(&'static str, u64)>,
}
