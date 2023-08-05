use windows::{
    Win32::Foundation::*, Win32::UI::Controls::*, Win32::UI::Input::Pointer::*,
};

use crate::monitors;

use pointer::DeviceType;
use pointer::Device;

pub mod pointerstates;
pub mod devicestates;
pub mod pointer;

#[derive(Debug)]
pub enum Error {
    PenCreationFailed,
    TouchCreationFailed,
    HimetricConversionFailed
}

pub fn create_pen() -> Result<Device, Error> {
    let dev: Result<HSYNTHETICPOINTERDEVICE, windows::core::Error>;
    unsafe {
        dev = CreateSyntheticPointerDevice(windows::Win32::UI::WindowsAndMessaging::PT_PEN, 1, POINTER_FEEDBACK_DEFAULT);
    }
    dev.map(|dev| Device::new(DeviceType::PEN, dev)).map_err(|_| Error::PenCreationFailed)
}

pub fn create_touch() -> Result<Device, Error> {
    let dev: Result<HSYNTHETICPOINTERDEVICE, windows::core::Error>;
    unsafe {
        dev = CreateSyntheticPointerDevice(windows::Win32::UI::WindowsAndMessaging::PT_TOUCH, 10, POINTER_FEEDBACK_DEFAULT);
    }
    dev.map(|dev| Device::new(DeviceType::PEN, dev)).map_err(|_| Error::TouchCreationFailed)
}

pub fn remove_touch(dev: Device) {
    unsafe { DestroySyntheticPointerDevice(*dev.get_device()) };
}

pub fn inject_pen(dev: &Device, x: i32, y: i32, pressure: u32, pen_flag: devicestates::pen, pointer_flag: u32, to_himetric: bool, tilt_x: Option<i32>, tilt_y: Option<i32>) -> Result<(), Error> {
    let pt = POINT { x, y };
    let mut pointer_info = POINTER_INFO {
        pointerType: windows::Win32::UI::WindowsAndMessaging::PT_PEN,
        pointerFlags: POINTER_FLAGS(pointer_flag),
        ptPixelLocation: pt,
        ..Default::default()
    };
    if to_himetric {
        let him = monitors::pixel_to_himetric(x, y).map_err(|_| Error::HimetricConversionFailed);
        match him {
            Ok(him) => pointer_info.ptHimetricLocation = POINT {x: him.0, y: him.1},
            Err(err) => return Err(err)
        }
    };
    let pointer_pen_info = POINTER_PEN_INFO {
        pointerInfo: pointer_info,
        pressure,
        penMask: windows::Win32::UI::WindowsAndMessaging::PEN_MASK_PRESSURE | windows::Win32::UI::WindowsAndMessaging::PEN_MASK_TILT_X | windows::Win32::UI::WindowsAndMessaging::PEN_MASK_TILT_Y,
        penFlags: pen_flag as u32,
        tiltX: tilt_x.unwrap_or(0),
        tiltY: tilt_y.unwrap_or(0),
        ..Default::default()
    };
    let dummyunion = POINTER_TYPE_INFO_0 {
        penInfo: pointer_pen_info
    };
    let pointer_type_info = POINTER_TYPE_INFO {
        r#type: windows::Win32::UI::WindowsAndMessaging::PT_PEN,
        Anonymous: dummyunion
    };
    unsafe {
        InjectSyntheticPointerInput(*dev.get_device(), &[pointer_type_info]);
    }
    Ok({})
}

//pub fn inject_touch