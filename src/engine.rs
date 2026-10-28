use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

#[repr(C)]
pub struct HamixWindowRaw {
    pub id: u32,
    pub width: c_int,
    pub height: c_int,
    pub scroll_x: c_int,
    pub scroll_y: c_int,
    pub title: *mut c_char,
    pub url: *mut c_char,
    pub status: *mut c_char,
    pub userdata: *mut c_void,
    pub bw: *mut c_void,
    pub next: *mut HamixWindowRaw,
}

extern "C" {
    fn hamix_init(cache_dir: *const c_char) -> c_int;
    fn hamix_shutdown();
    fn hamix_window_create(width: c_int, height: c_int, userdata: *mut c_void) -> *mut HamixWindowRaw;
    fn hamix_window_destroy(hw: *mut HamixWindowRaw);
    fn hamix_window_navigate(hw: *mut HamixWindowRaw, url: *const c_char);
    fn hamix_poll();
}

#[derive(Debug)]
pub struct WindowHandle {
    raw: *mut HamixWindowRaw,
}

impl WindowHandle {
    pub fn navigate(&self, url: &str) {
        if self.raw.is_null() {
            return;
        }
        if let Ok(c_url) = CString::new(url) {
            unsafe {
                hamix_window_navigate(self.raw, c_url.as_ptr());
            }
        }
    }

    pub fn title(&self) -> Option<String> {
        if self.raw.is_null() {
            return None;
        }
        unsafe {
            let ptr = (*self.raw).title;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }

    pub fn url(&self) -> Option<String> {
        if self.raw.is_null() {
            return None;
        }
        unsafe {
            let ptr = (*self.raw).url;
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }
}

impl Drop for WindowHandle {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                hamix_window_destroy(self.raw);
            }
            self.raw = std::ptr::null_mut();
        }
    }
}

pub struct Engine {
    initialized: bool,
}

impl Engine {
    pub fn new() -> Result<Self, &'static str> {
        let ret = unsafe { hamix_init(std::ptr::null()) };
        if ret == 0 {
            Ok(Self { initialized: true })
        } else {
            Err("failed to initialize NetSurf engine")
        }
    }

    pub fn create_window(&self, width: i32, height: i32) -> Result<WindowHandle, &'static str> {
        if !self.initialized {
            return Err("engine is not initialized");
        }
        let raw = unsafe { hamix_window_create(width as c_int, height as c_int, std::ptr::null_mut()) };
        if raw.is_null() {
            Err("failed to create window")
        } else {
            Ok(WindowHandle { raw })
        }
    }

    pub fn poll(&self) {
        if self.initialized {
            unsafe {
                hamix_poll();
            }
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                hamix_shutdown();
            }
            self.initialized = false;
        }
    }
}
