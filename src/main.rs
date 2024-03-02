use windows::core::w;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;

fn main() {
    unsafe {
        ShellExecuteW(
            None,
            w!("open"),
            w!("control.exe"),
            w!("ncpa.cpl"),
            None,
            SW_SHOW,
        );
    }
}
