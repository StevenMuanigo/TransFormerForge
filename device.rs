use anyhow::Result;
use candle_core::Device;

pub fn auto_detect_device(enable_gpu: bool) -> Result<Device> {
    if !enable_gpu {
        tracing::info!("GPU disabled, using CPU");
        return Ok(Device::Cpu);
    }

    #[cfg(feature = "cuda")]
    {
        match Device::new_cuda(0) {
            Ok(device) => {
                tracing::info!("CUDA device detected and enabled");
                return Ok(device);
            }
            Err(e) => {
                tracing::warn!("CUDA not available: {}, falling back to CPU", e);
            }
        }
    }

    #[cfg(feature = "metal")]
    {
        match Device::new_metal(0) {
            Ok(device) => {
                tracing::info!("Metal device detected and enabled");
                return Ok(device);
            }
            Err(e) => {
                tracing::warn!("Metal not available: {}, falling back to CPU", e);
            }
        }
    }

    tracing::info!("Using CPU for inference");
    Ok(Device::Cpu)
}

pub fn get_device_info() -> DeviceInfo {
    let device_type = if cfg!(feature = "cuda") {
        "CUDA (if available)"
    } else if cfg!(feature = "metal") {
        "Metal (if available)"
    } else {
        "CPU"
    };

    DeviceInfo {
        device_type: device_type.to_string(),
        available_memory: get_available_memory(),
    }
}

fn get_available_memory() -> Option<u64> {
    #[cfg(feature = "cuda")]
    {
        // Placeholder for actual CUDA memory query
        return Some(8_000_000_000); // 8GB dummy value
    }

    None
}

#[derive(Debug, serde::Serialize)]
pub struct DeviceInfo {
    pub device_type: String,
    pub available_memory: Option<u64>,
}
