use nvml_wrapper_sys::bindings::{nvmlDevice_t, NvmlLib};

pub fn set_gpu_frequency_offset(
    nvml_lib: &NvmlLib,
    handle: nvmlDevice_t,
    offset: i32,
) -> Result<u32, u32> {
    trace!("Setting GpcFreq offset: {offset}");
    match unsafe { nvml_lib.nvmlDeviceSetGpcClkVfOffset(handle, offset) } {
        0 => {
            info!("Set GpcFreq offset: {offset}");
            Ok(0)
        }
        other => {
            error!("Could not set GpcFreq offset: {offset}");
            Err(other)
        }
    }
}
pub fn set_memory_frequency_offset(
    nvml_lib: &NvmlLib,
    handle: nvmlDevice_t,
    offset: i32,
) -> Result<u32, u32> {
    trace!("Setting MemFreq offset: {offset}");
    match unsafe { nvml_lib.nvmlDeviceSetMemClkVfOffset(handle, offset) } {
        0 => {
            info!("Set MemFreq offset: {offset}");
            Ok(0)
        }
        other => {
            error!("Could not set MemFreq offset: {offset}");
            Err(other)
        }
    }
}
pub fn set_gpu_power_limit(
    nvml_lib: &NvmlLib,
    handle: nvmlDevice_t,
    limit: u32,
) -> Result<u32, u32> {
    trace!("Setting power limit: {limit}");
    match limit {
        0 => {
            warn!("Can't set power limit to 0, like why??");
            Ok(0)
        }
        3 => {
            warn!("Can't set power limit on this device, maybe you're using a laptop?");
            Ok(0)
        }
        _ => match unsafe { nvml_lib.nvmlDeviceSetPowerManagementLimit(handle, limit) } {
            0 => {
                info!("Set power limit: {limit}");
                Ok(0)
            }
            other => {
                error!("Could not set power limit: {limit}");
                Err(other)
            }
        },
    }
}

