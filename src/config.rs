mod operations;

use nvml_wrapper::Nvml;
use nvml_wrapper_sys::bindings::NvmlLib;
use serde::{Deserialize, Serialize};
use std::ffi::{c_int, c_uint};
use std::io::prelude::*;
use std::mem;
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

use crate::CONFIG;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Set {
    pub device_index: u32,
    pub gpu_freq_offset: i32,
    pub mem_freq_offset: i32,
    pub power_limit: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    sets: Vec<Set>,
}

impl Config {
    pub fn new() -> Self {
        trace!("Initializing NVML");
        let nvml = Nvml::init().expect("Failed to initialize NVML");
        let nvml_lib =
            unsafe { NvmlLib::new("libnvidia-ml.so").expect("Failed to load NVML library") };
        let device_count = nvml
            .device_count()
            .expect("Failed to get number of devices");
        let mut sets: Vec<Set> = Vec::new();
        for i in 0..device_count {
            trace!("Getting device with index {i}");
            let device = nvml.device_by_index(i).expect("Failed to get device");
            let raw_device = unsafe { device.handle() };
            sets.push(Set {
                device_index: i,
                gpu_freq_offset: unsafe {
                    trace!("Querying GpcFreq offset for device index: {i}");
                    let mut offset: c_int = mem::zeroed();
                    match nvml_lib.nvmlDeviceGetGpcClkVfOffset(raw_device, &mut offset) {
                        0 => offset,
                        other => {
                            error!("Error while querying Gpc Offset: {other}");
                            unreachable!()
                        }
                    }
                },
                mem_freq_offset: unsafe {
                    trace!("Querying MemFreq offset for device index: {i}");
                    let mut offset: c_int = mem::zeroed();
                    match nvml_lib.nvmlDeviceGetMemClkVfOffset(raw_device, &mut offset) {
                        0 => offset,
                        other => {
                            error!("Error while querying Mem Offset: {other}");
                            unreachable!()
                        }
                    }
                },
                power_limit: unsafe {
                    trace!("Querying power limit for device index: {i}");
                    let mut limit: c_uint = mem::zeroed();
                    match nvml_lib.nvmlDeviceGetPowerManagementLimit(raw_device, &mut limit) {
                        0 => limit,
                        3 => {
                            warn!(
                                "Can't query power limit on this device. Are you using a laptop?"
                            );
                            3
                        }
                        other => {
                            error!("Error while querying Power Limit: {other}");
                            unreachable!()
                        }
                    }
                },
            })
        }
        Self { sets }
    }
    pub fn init() -> Self {
        trace!("Initializing config");
        match Path::new(CONFIG).exists() {
            true => Config::from_config(),
            false => {
                trace!("Config file doesn't exist: {}", CONFIG);
                let config = Config::new();
                config.write_config().unwrap();
                config
            }
        }
    }
    pub fn from_config() -> Self {
        trace!("Reading config from file: {}", CONFIG);
        let mut attempts = 0;
        let max_attempts = 5;
        let wait_time = Duration::from_millis(100);

        while attempts < max_attempts {
            match fs::read_to_string(Path::new(CONFIG)) {
                Ok(contents) => {
                    match toml::from_str(&contents) {
                        Ok(config) => return config,
                        Err(e) => {
                            error!("Failed to parse config file: {}", e);
                            // If parsing fails, we don't retry - it's likely a format issue
                            break;
                        }
                    }
                }
                Err(e) => {
                    warn!(
                        "Failed to read config file (attempt {}): {}",
                        attempts + 1,
                        e
                    );
                    attempts += 1;
                    if attempts < max_attempts {
                        thread::sleep(wait_time);
                    }
                }
            }
        }

        error!(
            "Failed to read config file after {} attempts. Falling back to default config.",
            max_attempts
        );
        Config::new()
    }
    pub fn write_config(&self) -> std::io::Result<()> {
        trace!("Writing config to file: {}", CONFIG);
        let prefix = Path::new(CONFIG).parent().unwrap();
        fs::create_dir_all(prefix).unwrap();
        let mut config_file = fs::File::create(CONFIG).expect("Failed to create config file");
        config_file.write_all(
            toml::to_string(self)
                .expect("Failed to serialize")
                .as_bytes(),
        )
    }
    pub fn apply(self) {
        trace!("Applying config");
        let nvml = Nvml::init().expect("Failed to initialize NVML");
        let nvml_lib =
            unsafe { NvmlLib::new("libnvidia-ml.so").expect("Failed to load NVML library") };
        for s in self.sets {
            trace!("Applying set {}", s.device_index);
            let device = nvml
                .device_by_index(s.device_index)
                .expect("Failed to get device");
            let raw_device = unsafe { device.handle() };
            operations::set_gpu_frequency_offset(&nvml_lib, raw_device, s.gpu_freq_offset).unwrap();
            operations::set_memory_frequency_offset(&nvml_lib, raw_device, s.mem_freq_offset)
                .unwrap();
            operations::set_gpu_power_limit(&nvml_lib, raw_device, s.power_limit).unwrap();
        }
    }
}

