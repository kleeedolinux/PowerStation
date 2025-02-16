use std::sync::Arc;

use tokio::sync::Mutex;

use crate::performance::gpu::{
    intel,
    interface::{GPUDevice, GPUResult},
    tdp::{TDPDevice, TDPResult},
};

pub enum TDPDevices {
    Intel(intel::tdp::Tdp),
}

impl TDPDevices {
    pub async fn tdp(&self) -> TDPResult<f64> {
        match self {
            Self::Intel(dev) => dev.tdp().await,
        }
    }

    pub async fn set_tdp(&mut self, value: f64) -> TDPResult<()> {
        match self {
            Self::Intel(dev) => dev.set_tdp(value).await,
        }
    }

    pub async fn boost(&self) -> TDPResult<f64> {
        match self {
            Self::Intel(dev) => dev.boost().await,
        }
    }

    pub async fn set_boost(&mut self, value: f64) -> TDPResult<()> {
        match self {
            Self::Intel(dev) => dev.set_boost(value).await,
        }
    }

    pub async fn thermal_throttle_limit_c(&self) -> TDPResult<f64> {
        match self {
            Self::Intel(dev) => dev.thermal_throttle_limit_c().await,
        }
    }

    pub async fn set_thermal_throttle_limit_c(&mut self, limit: f64) -> TDPResult<()> {
        match self {
            Self::Intel(dev) => dev.set_thermal_throttle_limit_c(limit).await,
        }
    }

    //TODO: Deprecate the power_profile functions and set them automatically with TDP.
    pub async fn power_profile(&self) -> TDPResult<String> {
        match self {
            Self::Intel(dev) => dev.power_profile().await,
        }
    }

    pub async fn set_power_profile(&mut self, profile: String) -> TDPResult<()> {
        match self {
            Self::Intel(dev) => dev.set_power_profile(profile).await,
        }
    }

    pub async fn power_profiles_available(&self) -> TDPResult<Vec<String>> {
        match self {
            Self::Intel(dev) => dev.power_profiles_available().await,
        }
    }
}

pub enum GPUDevices {
    IntelGpu(intel::intelgpu::IntelGPU),
}

impl GPUDevices {
    pub async fn get_tdp_interface(&self) -> Option<Arc<Mutex<TDPDevices>>> {
        match self {
            Self::IntelGpu(dev) => dev.get_tdp_interface().await,
        }
    }

    pub async fn get_gpu_path(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.get_gpu_path().await,
        }
    }

    pub async fn name(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.name().await,
        }
    }

    pub async fn path(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.path().await,
        }
    }

    pub async fn class(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.class().await,
        }
    }

    pub async fn class_id(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.class_id().await,
        }
    }

    pub async fn vendor(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.vendor().await,
        }
    }

    pub async fn vendor_id(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.vendor_id().await,
        }
    }

    pub async fn device(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.device().await,
        }
    }

    pub async fn device_id(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.device_id().await,
        }
    }

    pub async fn subdevice(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.subdevice().await,
        }
    }

    pub async fn subdevice_id(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.subdevice_id().await,
        }
    }

    pub async fn subvendor_id(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.subvendor_id().await,
        }
    }

    pub async fn revision_id(&self) -> String {
        match self {
            Self::IntelGpu(dev) => dev.revision_id().await,
        }
    }

    pub async fn clock_limit_mhz_min(&self) -> GPUResult<f64> {
        match self {
            Self::IntelGpu(dev) => dev.clock_limit_mhz_min().await,
        }
    }

    pub async fn clock_limit_mhz_max(&self) -> GPUResult<f64> {
        match self {
            Self::IntelGpu(dev) => dev.clock_limit_mhz_max().await,
        }
    }

    pub async fn clock_value_mhz_min(&self) -> GPUResult<f64> {
        match self {
            Self::IntelGpu(dev) => dev.clock_value_mhz_min().await,
        }
    }

    pub async fn set_clock_value_mhz_min(&mut self, value: f64) -> GPUResult<()> {
        match self {
            Self::IntelGpu(dev) => dev.set_clock_value_mhz_min(value).await,
        }
    }

    pub async fn clock_value_mhz_max(&self) -> GPUResult<f64> {
        match self {
            Self::IntelGpu(dev) => dev.clock_value_mhz_max().await,
        }
    }

    pub async fn set_clock_value_mhz_max(&mut self, value: f64) -> GPUResult<()> {
        match self {
            Self::IntelGpu(dev) => dev.set_clock_value_mhz_max(value).await,
        }
    }

    pub async fn manual_clock(&self) -> GPUResult<bool> {
        match self {
            Self::IntelGpu(dev) => dev.manual_clock().await,
        }
    }

    pub async fn set_manual_clock(&mut self, enabled: bool) -> GPUResult<()> {
        match self {
            Self::IntelGpu(dev) => dev.set_manual_clock(enabled).await,
        }
    }
}
