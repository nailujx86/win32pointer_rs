use windows::Win32::UI::Controls::HSYNTHETICPOINTERDEVICE;

pub enum DeviceType {
    TOUCH,
    PEN
}

pub struct Device {
    devicetype: DeviceType,
    device: HSYNTHETICPOINTERDEVICE
}

impl Device {
    pub fn new(devicetype: DeviceType, device: HSYNTHETICPOINTERDEVICE) -> Device {
        Device {
            devicetype,
            device
        }
    }

    pub fn get_device(&self) -> &HSYNTHETICPOINTERDEVICE {
        &self.device
    }

    pub fn get_device_type(&self) -> &DeviceType {
        &self.devicetype
    }
}