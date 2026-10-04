pub struct CpuInfo {
    pub model_name: String,
    pub cpu_cores: u32,
}

pub struct MemInfo {
    pub mem_total_kb: u64,
    pub mem_free_kb: u64,
    pub cached_kb: u64,
    pub swap_total_kb: u64,
    pub swap_free_kb: u64,
}
