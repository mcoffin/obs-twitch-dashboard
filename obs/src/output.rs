use obs_sys;
use ::mem;

pub trait Output {
    fn get_name<'a>(&'a self) -> &'a str;
    fn get_service<'a>(&'a self) -> Option<&'a mut obs_sys::obs_service_t>;
    fn active(&self) -> bool;
}

impl Output for obs_sys::obs_output_t {
    fn get_name<'a>(&'a self) -> &'a str {
        use std::ffi;

        let cstr = unsafe {
            let ptr = obs_sys::obs_output_get_name(self as *const obs_sys::obs_output_t);
            ffi::CStr::from_ptr(ptr)
        };
        cstr.to_str().unwrap()
    }

    fn get_service<'a>(&'a self) -> Option<&'a mut obs_sys::obs_service_t> {
        unsafe {
            let ptr = obs_sys::obs_output_get_service(self as *const obs_sys::obs_output_t);
            mem::opt_ref_from_ptr_mut(ptr)
        }
    }

    fn active(&self) -> bool {
        unsafe {
            obs_sys::obs_output_active(self as *const obs_sys::obs_output_t)
        }
    }
}
