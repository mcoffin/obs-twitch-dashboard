use obs_sys;
use std::ffi;

pub trait Service {
    fn get_name(&self) -> &str;
    fn get_key(&self) -> Option<&str>;
    fn get_url(&self) -> &str;
}

impl Service for obs_sys::obs_service_t {
    fn get_name(&self) -> &str {
        let cstr = unsafe {
            let ptr = obs_sys::obs_service_get_name(self as *const obs_sys::obs_service_t);
            ffi::CStr::from_ptr(ptr)
        };
        cstr.to_str().unwrap()
    }

    fn get_key<'a>(&'a self) -> Option<&'a str> {
        use std::ptr;

        unsafe {
            let ptr = obs_sys::obs_service_get_key(self as *const obs_sys::obs_service_t);
            let maybe_ptr = if ptr == ptr::null_mut() {
                None
            } else {
                Some(ptr)
            };
            maybe_ptr.and_then(|p| ffi::CStr::from_ptr(p).to_str().ok())
        }
    }

    fn get_url(&self) -> &str {
        let cstr = unsafe {
            let ptr = obs_sys::obs_service_get_url(self as *const obs_sys::obs_service_t);
            ffi::CStr::from_ptr(ptr)
        };
        cstr.to_str().unwrap()
    }
}
