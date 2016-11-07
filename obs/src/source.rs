use libc;
use obs_sys;
use std::ffi;

/// Represents the minimal implementation of a an OBS source
pub trait Source {
    fn id() -> &'static str;
    fn typ() -> obs_sys::obs_source_type;
    fn output_flags() -> u32;

    fn create(settings: &mut obs_sys::obs_data_t,
              source: &mut obs_sys::obs_source_t) -> Self;
    fn get_name(&self) -> &str;
    fn get_width(&mut self) -> u32;
    fn get_height(&mut self) -> u32;
}

extern "C" fn real_create<S: Source>(settings: *mut obs_sys::obs_data_t,
                                     source: *mut obs_sys::obs_source_t) -> *mut S {
    let (settings_r, source_r) = unsafe {
        (&mut *settings, &mut *source)
    };
    let b: Box<S> = Box::new(Source::create(settings_r, source_r));
    Box::into_raw(b)
}

extern "C" fn real_destroy<S>(source: *mut S) {
    unsafe {
        Box::from_raw(source);
    }
}

extern "C" fn real_get_name<S: Source>(source: *mut S) -> *const libc::c_char {
    let source_ref = unsafe { &*source };
    ffi::CStr::from_bytes_with_nul(source_ref.get_name().as_bytes()).unwrap().as_ptr()
}

extern "C" fn real_get_width<S: Source>(source: &mut S) -> u32 {
    source.get_width()
}

extern "C" fn real_get_height<S: Source>(source: &mut S) -> u32 {
    source.get_height()
}

pub fn source_info<S: Source>() -> obs_sys::obs_source_info {
    use std::mem::transmute;

    obs_sys::obs_source_info {
        id: S::id().as_ptr() as *const libc::c_char,
        typ: S::typ(),
        output_flags: S::output_flags(),
        get_name: unsafe { transmute(real_get_name::<S> as *const extern "C" fn(*mut S) -> *const libc::c_char) },
        create: unsafe { transmute(real_create::<S> as *const extern "C" fn(*mut obs_sys::obs_data_t, *mut obs_sys::obs_source_t) -> *mut S) },
        destroy: unsafe { transmute(real_destroy::<S> as *const extern "C" fn(*mut S)) },
        get_width: unsafe { transmute(real_get_width::<S> as *const extern "C" fn(*mut S) -> u32) },
        get_height: unsafe { transmute(real_get_height::<S> as *const extern "C" fn (*mut S) -> u32) },
        get_defaults: None,
        get_properties: None,
        update: None,
        activate: None,
    }
}

pub fn register_source(source_info: &obs_sys::obs_source_info) {
    unsafe {
        obs_sys::obs_register_source(source_info as *const obs_sys::obs_source_info);
    }
}
