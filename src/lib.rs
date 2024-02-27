use std::os::raw::c_int;
use std::ptr::null_mut;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use winapi::shared::minwindef::{DWORD, LPARAM, LRESULT, WORD, WPARAM};
use winapi::shared::windef::HHOOK;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winuser::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, HC_ACTION, KBDLLHOOKSTRUCT, WH_KEYBOARD_LL,
    WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP,
};

pub static mut HOOK: HHOOK = null_mut();

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub timestamp: u128,
    pub is_key_down: bool,
    pub key: &'static str,
}

pub const TRUE: i32 = 1;
pub const FALSE: i32 = 0;

/// # Safety
pub unsafe fn get_scan_code(lpdata: LPARAM) -> DWORD {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);
    kb.scanCode
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ListenError {
    /// Windows
    KeyHookError(u32),
    /// Windows
    MouseHookError(u32),
}

type RawCallback = unsafe extern "system" fn(code: c_int, param: WPARAM, lpdata: LPARAM) -> LRESULT;

pub fn listen<T>(callback: T) -> Result<(), HookError>
where
    T: FnMut(Event) + 'static,
{
    unsafe {
        GLOBAL_CALLBACK = Some(Box::new(callback));
        set_key_hook(raw_callback)?;
        GetMessageW(null_mut(), null_mut(), 0, 0);
    }
    Ok(())
}

/// # Safety
pub unsafe fn set_key_hook(callback: RawCallback) -> Result<(), HookError> {
    let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(callback), null_mut(), 0);

    if hook.is_null() {
        let error = GetLastError();
        return Err(HookError::Key(error));
    }
    HOOK = hook;
    Ok(())
}

static mut GLOBAL_CALLBACK: Option<Box<dyn FnMut(Event)>> = None;

#[derive(Debug)]
pub enum HookError {
    Mouse(DWORD),
    Key(DWORD),
}

unsafe extern "system" fn raw_callback(code: c_int, param: WPARAM, lpdata: LPARAM) -> LRESULT {
    if code == HC_ACTION {
        let (opt, is_key_down) = convert(param, lpdata);
        if let Some(key) = opt {
            let event = Event {
                is_key_down,
                key,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::new(0, 0))
                    .as_millis(),
            };
            if let Some(callback) = &mut GLOBAL_CALLBACK {
                callback(event);
            }
        }
    }
    CallNextHookEx(HOOK, code, param, lpdata)
}

macro_rules! decl_keycodes {
    ($($key:literal, $code:literal),*) => {
        //TODO: make const when rust lang issue #49146 is fixed
        pub fn code_from_key(key: &'static str) -> Option<WORD> {
            match key {
                $(
                    $key => Some($code),
                )*
                _ => None,
            }
        }

        //TODO: make const when rust lang issue #49146 is fixed
        pub fn key_from_code(code: WORD) -> &'static str {
            match code {
                $(
                    $code => $key,
                )*
                _ => "Unknown"
            }
        }
    };
}

// https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
// We redefined here for Letter and number keys which are not in winapi crate (and don't have a name either in win32)
decl_keycodes! {
    "Alt", 164,
    "AltGr", 165,
    "Backspace", 0x08,
    "CapsLock", 20,
    "L_Control", 162,
    "R_Control", 163,
    "Delete", 46,
    "DownArrow", 40,
    "End", 35,
    "Escape", 27,
    "F1", 112,
    "F10", 121,
    "F11", 122,
    "F12", 123,
    "F2", 113,
    "F3", 114,
    "F4", 115,
    "F5", 116,
    "F6", 117,
    "F7", 118,
    "F8", 119,
    "F9", 120,
    "Home", 36,
    "LeftArrow", 37,
    "L_Windows", 91,
    "PageDown", 34,
    "PageUp", 33,
    "Return", 0x0D,
    "RightArrow", 39,
    "L_Shift", 160,
    "R_Shift", 161,
    "Space", 32,
    "Tab", 0x09,
    "UpArrow", 38,
    "PrintScreen", 44,
    "ScrollLock", 145,
    "Pause", 19,
    "NumLock", 144,
    "BackQuote", 192,
    "Num1", 49,
    "Num2", 50,
    "Num3", 51,
    "Num4", 52,
    "Num5", 53,
    "Num6", 54,
    "Num7", 55,
    "Num8", 56,
    "Num9", 57,
    "Num0", 48,
    "Minus", 189,
    "Equal", 187,
    "KeyQ", 81,
    "KeyW", 87,
    "KeyEv", 69,
    "KeyR", 82,
    "KeyT", 84,
    "KeyY", 89,
    "KeyU", 85,
    "KeyI", 73,
    "KeyO", 79,
    "KeyP", 80,
    "L_Bracket", 219,
    "R_Bracket", 221,
    "KeyA", 65,
    "KeyS", 83,
    "KeyD", 68,
    "KeyF", 70,
    "KeyG", 71,
    "KeyH", 72,
    "KeyJ", 74,
    "KeyK", 75,
    "KeyL", 76,
    "SemiColon", 186,
    "Quote", 222,
    "BackSlash", 220,
    "IntlBackslash", 226,
    "KeyZ", 90,
    "KeyX", 88,
    "KeyC", 67,
    "KeyV", 86,
    "KeyB", 66,
    "KeyN", 78,
    "KeyM", 77,
    "Comma", 188,
    "Dot", 190,
    "Slash", 191,
    "Insert", 45,
    //KP_RETURN, 13,
    "KpMinus", 109,
    "KpPlus", 107,
    "KpMultiply", 106,
    "KpDivide", 111,
    "Kp0", 96,
    "Kp1", 97,
    "Kp2", 98,
    "Kp3", 99,
    "Kp4", 100,
    "Kp5", 101,
    "Kp6", 102,
    "Kp7", 103,
    "Kp8", 104,
    "Kp9", 105,
    "KpDelete", 110
}

/// # Safety
pub unsafe fn get_code(lpdata: LPARAM) -> DWORD {
    let kb = *(lpdata as *const KBDLLHOOKSTRUCT);
    kb.vkCode
}

/// # Safety
pub unsafe fn convert(param: WPARAM, lpdata: LPARAM) -> (Option<&'static str>, bool) {
    match param.try_into() {
        Ok(WM_KEYDOWN) | Ok(WM_SYSKEYDOWN) => {
            let code = get_code(lpdata);
            let key = key_from_code(code as u16);
            (Some(key), true)
        }
        Ok(WM_KEYUP) | Ok(WM_SYSKEYUP) => {
            let code = get_code(lpdata);
            let key = key_from_code(code as u16);
            (Some(key), false)
        }
        _ => (None, false),
    }
}

#[cfg(test)]
mod test {
    use super::{code_from_key, key_from_code};
    #[test]
    fn test_reversible() {
        for code in 0..65535 {
            let key = key_from_code(code);
            if let Some(code2) = code_from_key(key) {
                assert_eq!(code, code2)
            } else {
                assert_eq!(key, "Unknown");
            }
        }
    }
}
