use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows_sys::Win32::System::ProcessStatus::K32GetModuleBaseNameW;
use windows_sys::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowThreadProcessId};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetKeyboardLayout, ActivateKeyboardLayout};

pub fn get_active_window_info() -> Option<(String, u32, u32)> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return None;
        }

        let mut pid = 0;
        let thread_id = GetWindowThreadProcessId(hwnd, &mut pid);

        let process = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if process.is_null() {
            return None;
        }

        let mut name_buffer = [0u16; 260];
        let len = K32GetModuleBaseNameW(process, std::ptr::null_mut(), name_buffer.as_mut_ptr(), name_buffer.len() as u32);
        let process_name = if len > 0 {
            String::from_utf16_lossy(&name_buffer[..len as usize])
        } else {
            return None;
        };

        let layout = GetKeyboardLayout(thread_id) as u32;

        Some((process_name, layout, thread_id))
    }
}

pub fn set_layout(layout: u32) -> bool {
    unsafe {
        let result = ActivateKeyboardLayout(layout as _, 0);
        !result.is_null()
    }
}