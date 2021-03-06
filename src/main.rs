#![windows_subsystem = "windows"]
use std::ptr::null_mut;
use winapi::{
    shared::windef::HHOOK,
    um::winuser::{
        keybd_event, CallNextHookEx, GetMessageA, SetWindowsHookExA, UnhookWindowsHookEx,
        KBDLLHOOKSTRUCT, KEYEVENTF_KEYUP, VK_CAPITAL, VK_ESCAPE, WH_KEYBOARD_LL, WM_KEYDOWN,
        WM_KEYUP,
    },
};

fn main() {
    unsafe {
        let hook_id = SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_callback), null_mut(), 0);
        while GetMessageA(null_mut(), null_mut(), 0, 0) == 0 {}
        UnhookWindowsHookEx(hook_id);
    }
}

unsafe extern "system" fn hook_callback(code: i32, w_param: usize, l_param: isize) -> isize {
    let info = *(l_param as *mut KBDLLHOOKSTRUCT);
    if info.vkCode == VK_CAPITAL as u32 {
        match w_param as u32 {
            WM_KEYDOWN => keybd_event(VK_ESCAPE as u8, 0x1, 0, 0),
            WM_KEYUP => keybd_event(VK_ESCAPE as u8, 0x1, KEYEVENTF_KEYUP, 0),
            _ => (),
        };
        return 1;
    }

    CallNextHookEx(0 as HHOOK, code, w_param, l_param)
}
