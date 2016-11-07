use libc;
use obs_sys;
use std::ffi;

/// Wrapper methods that should be available for `obs_data_t` objects.
pub trait Data {
    fn set_default_string(&mut self, key: &str, value: &str);
    fn get_string<'a>(&'a mut self, key: &str) -> &'a str;
}

#[inline(always)]
fn as_c_str(s: &str) -> *const libc::c_char {
    use ::mem;

    mem::checked_c_str(s.as_bytes()).as_ptr()
}

impl Data for obs_sys::obs_data_t {
    fn set_default_string(&mut self,
                          key: &str,
                          value: &str) {
        unsafe {
            obs_sys::obs_data_set_default_string(
                self as *mut obs_sys::obs_data_t,
                as_c_str(key),
                as_c_str(value));
        }
    }

    fn get_string<'a>(&'a mut self,
                      key: &str) -> &'a str {
        let cstr = unsafe {
            let ptr = obs_sys::obs_data_get_string(
                self as *mut obs_sys::obs_data_t,
                as_c_str(key));
            ffi::CStr::from_ptr(ptr)
        };
        cstr.to_str().unwrap()
    }
}
