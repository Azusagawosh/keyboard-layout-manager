use x11::xlib::*;
use std::ffi::CString;

pub fn get_active_window_info() -> Option<(String, u32, u32)> {
    unsafe {
        let display = XOpenDisplay(std::ptr::null());
        if display.is_null() {
            return None;
        }

        let mut window: u64 = 0;
        // Для получения активного окна в X11 можно использовать XGetInputFocus
        let status = XGetInputFocus(display, &mut window, std::ptr::null_mut());
        if status != 0 || window == 0 {
            XCloseDisplay(display);
            return None;
        }

        let mut pid: u64 = 0;
        let mut pid_len: u64 = 0;
        
        let atom = XInternAtom(display, CString::new("_NET_WM_PID").unwrap().as_ptr(), 0);
        if atom != 0 {
            let mut actual_type: u64 = 0;
            let mut actual_format: i32 = 0;
            let mut nitems: u64 = 0;
            let mut bytes_after: u64 = 0;
            let mut prop: *mut u8 = std::ptr::null_mut();

            XGetWindowProperty(
                display,
                window as u64,
                atom,
                0,
                1,
                0,
                0,
                &mut actual_type,
                &mut actual_format,
                &mut nitems,
                &mut bytes_after,
                &mut prop,
            );

            if !prop.is_null() {
                pid = *(prop as *mut u64);
                XFree(prop as *mut _);
            }
        }

        // Получаем имя процесса через /proc (более надёжно, чем X11)
        let process_name = if pid > 0 {
            let path = format!("/proc/{}/comm", pid);
            if let Ok(content) = std::fs::read_to_string(&path) {
                content.trim().to_string()
            } else {
                "unknown".to_string()
            }
        } else {
            "unknown".to_string()
        };

        // В X11 получение текущей раскладки — отдельная история через Xkb
        // Пока возвращаем заглушку, но позже можно реализовать через XkbGetState
        let layout = 0x0409; // English by default
        
        XCloseDisplay(display);
        Some((process_name, layout, 0))
    }
}

pub fn set_layout(_layout: u32) -> bool {
    // В X11 переключение раскладки делается через XkbLockGroup
    // Пока заглушка — всегда успешно
    true
}