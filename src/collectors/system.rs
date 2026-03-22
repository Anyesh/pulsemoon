use sysinfo::{Disks, System};

use crate::types::{CpuMetrics, DiskInfo, MemoryMetrics};

const HISTORY_LEN: usize = 60;

pub struct SystemCollector {
    sys: System,
    cpu_metrics: CpuMetrics,
    memory_metrics: MemoryMetrics,
}

impl SystemCollector {
    pub fn new() -> Self {
        let mut sys = System::new();
        // Perform an initial CPU refresh so the *next* call returns real data.
        sys.refresh_cpu_usage();

        Self {
            sys,
            cpu_metrics: CpuMetrics::default(),
            memory_metrics: MemoryMetrics::default(),
        }
    }

    /// Refresh CPU and memory information from the OS.
    pub fn refresh(&mut self) {
        self.sys.refresh_cpu_usage();
        self.sys.refresh_memory();
    }

    /// Return up-to-date CPU metrics.
    ///
    /// Reads per-core usage from `sysinfo`, computes the global average,
    /// appends it to the rolling history (capped at 60 samples), and
    /// fetches the CPU brand name from the first core.
    pub fn cpu_metrics(&mut self) -> &CpuMetrics {
        let cpus = self.sys.cpus();

        // Per-core usage
        let per_core: Vec<f32> = cpus.iter().map(|c| c.cpu_usage()).collect();

        // Global average
        let global_usage = if per_core.is_empty() {
            0.0
        } else {
            per_core.iter().sum::<f32>() / per_core.len() as f32
        };

        // History (ring-buffer style, max 60 entries)
        if self.cpu_metrics.history.len() >= HISTORY_LEN {
            self.cpu_metrics.history.pop_front();
        }
        self.cpu_metrics.history.push_back(global_usage);

        // CPU brand name from the first core
        let cpu_name = cpus
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_default();

        self.cpu_metrics.global_usage = global_usage;
        self.cpu_metrics.per_core = per_core;
        self.cpu_metrics.cpu_name = cpu_name;

        &self.cpu_metrics
    }

    /// Return up-to-date memory metrics.
    ///
    /// Reads total/used RAM and swap from `sysinfo`, computes a usage
    /// percentage, and appends it to the rolling history.
    pub fn memory_metrics(&mut self) -> &MemoryMetrics {
        let total = self.sys.total_memory();
        let used = self.sys.used_memory();
        let swap_total = self.sys.total_swap();
        let swap_used = self.sys.used_swap();

        // Usage percentage for the history sparkline
        let usage_pct = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        if self.memory_metrics.history.len() >= HISTORY_LEN {
            self.memory_metrics.history.pop_front();
        }
        self.memory_metrics.history.push_back(usage_pct);

        self.memory_metrics.total = total;
        self.memory_metrics.used = used;
        self.memory_metrics.swap_total = swap_total;
        self.memory_metrics.swap_used = swap_used;

        &self.memory_metrics
    }

    /// Return a snapshot of all mounted disks.
    pub fn disk_metrics(&self) -> Vec<DiskInfo> {
        let disks = Disks::new_with_refreshed_list();

        disks
            .iter()
            .map(|d| {
                let total = d.total_space();
                let available = d.available_space();
                let used = total.saturating_sub(available);

                DiskInfo {
                    name: d.name().to_string_lossy().to_string(),
                    mount_point: d.mount_point().to_string_lossy().to_string(),
                    total,
                    used,
                    fs_type: d.file_system().to_string_lossy().to_string(),
                }
            })
            .collect()
    }
}
