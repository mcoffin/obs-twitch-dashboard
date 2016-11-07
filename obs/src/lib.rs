#![feature(conservative_impl_trait)]

extern crate libc;
extern crate obs_sys;

pub mod data;
pub mod source;
pub mod mem;
pub mod output;
pub mod properties;
pub mod service;

pub use data::Data as Data;
pub use output::Output as Output;
pub use properties::Properties as Properties;
pub use service::Service as Service;

pub fn version() -> u32 {
    unsafe {
        obs_sys::obs_get_version()
    }
}

pub fn initialized() -> bool {
    unsafe {
        obs_sys::obs_initialized()
    }
}

pub fn set_locale(new_locale: &str) {
    unsafe {
        obs_sys::obs_set_locale(new_locale.as_ptr() as *const libc::c_char);
    }
}

pub fn log_loaded_modules() {
    unsafe {
        obs_sys::obs_log_loaded_modules();
    }
}

pub fn get_master_volume() -> f32 {
    let obs_vol = unsafe {
        obs_sys::obs_get_master_volume()
    };
    obs_vol as f32
}

struct EnumHelper<'f, T: 'f> {
    enum_fn: &'f mut FnMut(&mut T) -> bool,
}

impl<'f, T> EnumHelper<'f, T> {
    pub extern fn enum_helper(&mut self, item: *mut T) -> bool {
        let item_ref = unsafe { &mut *item };
        (self.enum_fn)(item_ref)
    }

    pub fn as_ptr(&mut self) -> *mut EnumHelper<'f, T> {
        self as *mut EnumHelper<'f, T>
    }
}

type EnumFn<T> = unsafe extern fn(extern fn(*mut libc::c_void, *mut T) -> bool, *mut libc::c_void);

// Internal template function for createing all `enum_*` functions for various
// types
fn enum_internal<T, F: FnMut(&mut T) -> bool>(mut f: F, do_enum: EnumFn<T>) {
    use std::mem::transmute;

    let enum_helper_ptr = EnumHelper::<T>::enum_helper as *const extern fn(&EnumHelper<T>, *mut T) -> bool;
    let f_ref: &mut FnMut(&mut T) -> bool = &mut f;
    let mut helper = EnumHelper {
        enum_fn: f_ref,
    };
    unsafe {
        do_enum(transmute(enum_helper_ptr),
                transmute(helper.as_ptr()));
    }
}

/// Convenience function for `obs_enum_outputs`
// Always inlined because it's just an alias for enum_internal with
// obs_enum_sources as an argument
#[inline(always)]
pub fn enum_outputs<F: FnMut(&mut obs_sys::obs_output_t) -> bool>(f: F) {
    enum_internal(f, obs_sys::obs_enum_outputs);
}

/// Convenience function for `obs_enum_sources`
// Always inlined because it's just an alias for enum_internal with
// obs_enum_sources as an argument
#[inline(always)]
pub fn enum_sources<F: FnMut(&mut obs_sys::obs_source_t) -> bool>(f: F) {
    enum_internal(f, obs_sys::obs_enum_sources);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
