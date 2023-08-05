use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::UI::HiDpi::*,
    Win32::UI::WindowsAndMessaging::*,
};

pub fn get_primary_dpi() -> Result<(u32, u32)> {
    let (mut x, mut y): (u32, u32) = (0, 0);
    unsafe {
        let hmonitor = MonitorFromPoint(POINT { x: 0, y: 0 }, MONITOR_DEFAULTTOPRIMARY);
        GetDpiForMonitor(hmonitor, MDT_EFFECTIVE_DPI, &mut x, &mut y)?;
    }
    Ok((x, y))
}

pub fn get_primary_res() -> (i32, i32) {
    unsafe { (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) }
}

pub fn pixel_to_himetric(x: i32, y: i32) -> Result<(i32, i32)> {
    let (dpi_x, dpi_y) = get_primary_dpi()?;
    Ok((x * 2540 / dpi_x as i32, y * 2540 / dpi_y as i32))
}