#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
use std::ptr::null_mut;
use winapi::{
    shared::windef::HHOOK,
    um::winuser::{
        CallNextHookEx, GetAsyncKeyState, GetMessageA, SendInput, SetWindowsHookExA,
        UnhookWindowsHookEx, INPUT, INPUT_KEYBOARD, KBDLLHOOKSTRUCT, KEYEVENTF_KEYUP, VK_CAPITAL,
        VK_CONTROL, VK_ESCAPE, VK_F12, VK_LCONTROL, VK_OEM_2, VK_RSHIFT, VK_SHIFT, VK_TAB,
        WH_KEYBOARD_LL, WM_KEYUP,
    },
};

pub struct Remap {
    pub from: i32,
    pub to: i32,
    pub modifiers: Vec<i32>,
}

static mut REMAPS: Vec<Remap> = Vec::new();

fn main() {
    unsafe {
        REMAPS.push(Remap {
            from: VK_CAPITAL,
            to: VK_ESCAPE,
            modifiers: Vec::new(),
        });

        let hook_id = SetWindowsHookExA(WH_KEYBOARD_LL, Some(hook_callback), null_mut(), 0);
        //Handle event's and block. This will not spinlock.
        while GetMessageA(null_mut(), null_mut(), 0, 0) == 0 {}
        UnhookWindowsHookEx(hook_id);
    }
}

unsafe fn send_key_event(key_code: u16, flags: u32) {
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: std::mem::MaybeUninit::zeroed().assume_init(),
    };

    input.u.ki_mut().wVk = key_code;
    input.u.ki_mut().dwFlags = flags;

    //If the function returns zero, the input was already blocked by another thread.
    let _ = SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
}

unsafe extern "system" fn hook_callback(code: i32, w_param: usize, l_param: isize) -> isize {
    let info = *(l_param as *mut KBDLLHOOKSTRUCT);

    // if info.vkCode == VK_F12 as u32 {
    //     let flags = if w_param as u32 == WM_KEYUP {
    //         KEYEVENTF_KEYUP
    //     } else {
    //         0
    //     };

    //     if GetAsyncKeyState(VK_CONTROL) & 0x8000u16 as i16 != 0
    //         && GetAsyncKeyState(VK_SHIFT) & 0x8000u16 as i16 != 0
    //     {
    //         send_key_event(VK_SHIFT as u16, KEYEVENTF_KEYUP);
    //         send_key_event(VK_CONTROL as u16, KEYEVENTF_KEYUP);
    //         send_key_event(VK_CAPITAL as u16, flags);
    //         send_key_event(VK_CAPITAL as u16, KEYEVENTF_KEYUP);
    //         return 1;
    //     }
    // }

    if let Some(remap) = REMAPS.iter().find(|remap| remap.from == info.vkCode as i32) {
        let flags = if w_param as u32 == WM_KEYUP {
            KEYEVENTF_KEYUP
        } else {
            0
        };

        for modifier in &remap.modifiers {
            send_key_event(*modifier as u16, 0);
        }

        send_key_event(remap.to as u16, flags);

        for modifier in &remap.modifiers {
            send_key_event(*modifier as u16, KEYEVENTF_KEYUP);
        }

        return 1;
    }

    CallNextHookEx(0 as HHOOK, code, w_param, l_param)
}
