use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum InputSource {
    VGA1 = 0x01,
    VGA2 = 0x02,
    DVI1 = 0x03,
    DVI2 = 0x04,
    CompositeVideo1 = 0x05,
    CompositeVideo2 = 0x06,
    SVideo1 = 0x07,
    SVideo2 = 0x08,
    Tuner1 = 0x09,
    Tuner2 = 0x0A,
    Tuner3 = 0x0B,
    Component1 = 0x0C,
    Component2 = 0x0D,
    Component3 = 0x0E,
    DisplayPort1 = 0x0F,
    DisplayPort2 = 0x10,
    HDMI1 = 0x11,
    HDMI2 = 0x12,
    HDMI3 = 0x13,
    HDMI4 = 0x14,
    USBC1 = 0x15,
    USBC2 = 0x16,
    USBC3 = 0x17,
    Unknown = 0xFF,
}

impl InputSource {
    pub fn from_vcp_value(value: u16) -> Self {
        match value {
            0x01 => InputSource::VGA1,
            0x02 => InputSource::VGA2,
            0x03 => InputSource::DVI1,
            0x04 => InputSource::DVI2,
            0x05 => InputSource::CompositeVideo1,
            0x06 => InputSource::CompositeVideo2,
            0x07 => InputSource::SVideo1,
            0x08 => InputSource::SVideo2,
            0x09 => InputSource::Tuner1,
            0x0A => InputSource::Tuner2,
            0x0B => InputSource::Tuner3,
            0x0C => InputSource::Component1,
            0x0D => InputSource::Component2,
            0x0E => InputSource::Component3,
            0x0F => InputSource::DisplayPort1,
            0x10 => InputSource::DisplayPort2,
            0x11 => InputSource::HDMI1,
            0x12 => InputSource::HDMI2,
            0x13 => InputSource::HDMI3,
            0x14 => InputSource::HDMI4,
            0x15 => InputSource::USBC1,
            0x16 => InputSource::USBC2,
            0x17 => InputSource::USBC3,
            _ => InputSource::Unknown,
        }
    }

    pub fn to_vcp_value(self) -> u16 {
        self as u16
    }

    pub fn name(&self) -> &'static str {
        match self {
            InputSource::VGA1 => "VGA 1",
            InputSource::VGA2 => "VGA 2",
            InputSource::DVI1 => "DVI 1",
            InputSource::DVI2 => "DVI 2",
            InputSource::CompositeVideo1 => "Composite 1",
            InputSource::CompositeVideo2 => "Composite 2",
            InputSource::SVideo1 => "S-Video 1",
            InputSource::SVideo2 => "S-Video 2",
            InputSource::Tuner1 => "Tuner 1",
            InputSource::Tuner2 => "Tuner 2",
            InputSource::Tuner3 => "Tuner 3",
            InputSource::Component1 => "Component 1",
            InputSource::Component2 => "Component 2",
            InputSource::Component3 => "Component 3",
            InputSource::DisplayPort1 => "DisplayPort 1",
            InputSource::DisplayPort2 => "DisplayPort 2",
            InputSource::HDMI1 => "HDMI 1",
            InputSource::HDMI2 => "HDMI 2",
            InputSource::HDMI3 => "HDMI 3",
            InputSource::HDMI4 => "HDMI 4",
            InputSource::USBC1 => "USB-C 1",
            InputSource::USBC2 => "USB-C 2",
            InputSource::USBC3 => "USB-C 3",
            InputSource::Unknown => "Unknown",
        }
    }
}

impl fmt::Display for InputSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

