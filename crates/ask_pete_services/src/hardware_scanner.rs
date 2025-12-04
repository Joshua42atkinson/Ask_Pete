use sysinfo::System;

#[derive(Debug, Clone)]
pub struct SystemSpecs {
    pub total_memory_gb: u64,
    pub cpu_cores: usize,
}

pub struct HardwareScanner;

impl HardwareScanner {
    pub fn scan() -> SystemSpecs {
        let mut sys = System::new_all();
        sys.refresh_all();

        let total_memory = sys.total_memory(); // Bytes
        let total_memory_gb = total_memory / 1024 / 1024 / 1024;
        let cpu_cores = sys.cpus().len();

        SystemSpecs {
            total_memory_gb,
            cpu_cores,
        }
    }
}
