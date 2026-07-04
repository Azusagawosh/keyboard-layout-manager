use x11::xlib::*;
use std::ffi::CString;

pub fn get_active_window_info() -> Option<(String, u32, u32)> {
    unsafe {
        let display = XOpenDisplay(std::ptr::null());
        if display.is_null() {
            return None;
        }

        let window = XGetInputFocus(display, std::ptr::null_mut(), std::ptr::null_mut());
        let mut pid: u32 = 0;
        let mut pid_len: u32 = 0;
        
        let atom = XInternAtom(display, CString::new("_NET_WM_PID").unwrap().as_ptr(), 0);
        if atom != 0 {
            XGetWindowProperty(
                display,
                window,
                atom,
                0,
                1,
                0,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut pid_len,
                std::ptr::null_mut(),
                &mut pid as *mut u32 as *mut _,
            );
        }

        // Для получения раскладки в X11 нужно больше работы,
        // но для MVP используем заглушку
        let process_name = "unknown".to_string();
        let layout = 0x0409; // English by default
        
        XCloseDisplay(display);
        Some((process_name, layout, 0))
    }
}

pub fn set_layout(_layout: u32) -> bool {
    // В X11 это делается через XkbLockGroup
    // Пока заглушка
    true
}