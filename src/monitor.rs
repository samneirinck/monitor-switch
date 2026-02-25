use ddc_hi::{Ddc, Display};
use thiserror::Error;
use crate::InputSource;

const VCP_INPUT_SELECT: u8 = 0x60;

#[derive(Error, Debug)]
pub enum MonitorError {
    #[error("Failed to communicate with monitor: {0}")]
    DdcError(String),
    #[error("Monitor not found")]
    NotFound,
    #[error("Operation not supported by this monitor")]
    NotSupported,
}

pub struct Monitor {
    display: Display,
    index: usize,
}

impl Monitor {
    pub fn enumerate() -> Vec<Monitor> {
        Display::enumerate()
            .into_iter()
            .enumerate()
            .map(|(index, display)| Monitor { display, index })
            .collect()
    }

    pub fn id(&self) -> String {
        let base_id = &self.display.info.id;
        if let Some(serial) = &self.display.info.serial_number {
            format!("{}-{}", base_id, serial)
        } else if let Some(serial) = self.display.info.serial {
            format!("{}-{}", base_id, serial)
        } else {
            format!("{}-{}", base_id, self.index)
        }
    }

    pub fn model_name(&self) -> Option<String> {
        self.display.info.model_name.clone()
    }

    pub fn manufacturer_id(&self) -> Option<String> {
        self.display.info.manufacturer_id.clone()
    }

    pub fn get_current_input(&mut self) -> Result<InputSource, MonitorError> {
        let value = self
            .display
            .handle
            .get_vcp_feature(VCP_INPUT_SELECT)
            .map_err(|e| MonitorError::DdcError(e.to_string()))?;

        Ok(InputSource::from_vcp_value(value.value()))
    }

    pub fn set_input(&mut self, input: InputSource) -> Result<(), MonitorError> {
        self.display
            .handle
            .set_vcp_feature(VCP_INPUT_SELECT, input.to_vcp_value())
            .map_err(|e| MonitorError::DdcError(e.to_string()))
    }

    pub fn get_available_inputs(&mut self) -> Result<Vec<InputSource>, MonitorError> {
        let _ = self.display.update_capabilities();

        Ok(self.get_common_inputs())
    }

    fn get_common_inputs(&self) -> Vec<InputSource> {
        vec![
            InputSource::HDMI1,
            InputSource::HDMI2,
            InputSource::DisplayPort1,
            InputSource::DisplayPort2,
            InputSource::USBC1,
            InputSource::USBC2,
        ]
    }
}

