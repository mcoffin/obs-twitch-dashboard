#![allow(improper_ctypes)]

extern crate libc;

#[repr(C)]
pub struct obs_data_t;

#[repr(C)]
pub struct obs_output_t;

#[repr(C)]
pub struct obs_property_t;

#[repr(C)]
pub struct obs_properties_t;

#[repr(C)]
pub struct obs_service_t;

#[repr(C)]
pub struct obs_source_t;

#[repr(C)]
#[derive(Clone)]
pub enum obs_source_type {
    INPUT,
    FILTER,
    TRANSITION,
    SCENE
}

#[repr(C)]
pub enum obs_text_type {
    DEFAULT,
    PASSWORD,
    MULTILINE,
}

#[repr(C)]
#[derive(Clone)]
pub struct obs_source_info {
   pub id: *const libc::c_char,
   pub typ: obs_source_type,
   pub output_flags: u32,
   pub get_name: extern fn(type_data: *mut libc::c_void) -> *const libc::c_char,
   pub create: extern fn(settings: *mut obs_data_t, source: *mut obs_source_t) -> *mut libc::c_void,
   pub destroy: extern fn(data: *mut libc::c_void),
   pub get_width: extern fn(data: *mut libc::c_void) -> u32,
   pub get_height: extern fn(data: *mut libc::c_void) -> u32,
   pub get_defaults: Option<extern fn(settings: *mut obs_data_t)>,
   pub get_properties: Option<extern fn(data: *mut libc::c_void) -> *const obs_properties_t>,
   pub update: Option<extern fn(data: *mut libc::c_void, settings: *mut obs_data_t)>,
   pub activate: Option<extern fn(data: *mut libc::c_void)>,
}

extern "C" {
    pub fn obs_get_version() -> u32;
    pub fn obs_shutdown();
    pub fn obs_initialized() -> bool;
    pub fn obs_set_locale(new_locale: *const libc::c_char);
    pub fn obs_log_loaded_modules();
    pub fn obs_render_main_view();
    pub fn obs_get_master_volume() -> libc::c_float;
    pub fn obs_register_source_s(source_info: *const obs_source_info, count: libc::size_t);

    pub fn obs_data_set_default_string(data: *mut obs_data_t, key: *const libc::c_char, value: *const libc::c_char);
    pub fn obs_data_set_default_int(data: *mut obs_data_t, key: *const libc::c_char, value: libc::c_int);
    pub fn obs_data_set_default_double(data: *mut obs_data_t, key: *const libc::c_char, value: libc::c_double);
    pub fn obs_data_set_default_bool(data: *mut obs_data_t, key: *const libc::c_char, value: bool);
    pub fn obs_data_get_string(data: *mut obs_data_t, key: *const libc::c_char) -> *const libc::c_char;

    pub fn obs_properties_create() -> *mut obs_properties_t;
    pub fn obs_properties_destroy(props: *mut obs_properties_t);
    pub fn obs_properties_add_text(props: *mut obs_properties_t, key: *const libc::c_char, value: *const libc::c_char, typ: obs_text_type) -> *mut obs_property_t;
    pub fn obs_properties_set_flags(props: *mut obs_properties_t, flags: u32);

    pub fn obs_enum_sources(f: extern fn(*mut libc::c_void, *mut obs_source_t) -> bool, param: *mut libc::c_void);
    pub fn obs_enum_outputs(f: extern fn(*mut libc::c_void, *mut obs_output_t) -> bool, param: *mut libc::c_void);

    pub fn obs_output_get_name(output: *const obs_output_t) -> *const libc::c_char;
    pub fn obs_output_get_service(output: *const obs_output_t) -> *mut obs_service_t;
    pub fn obs_output_active(output: *const obs_output_t) -> bool;

    pub fn obs_service_get_name(service: *const obs_service_t) -> *const libc::c_char;
    pub fn obs_service_get_key(service: *const obs_service_t) -> *const libc::c_char;
    pub fn obs_service_get_url(service: *const obs_service_t) -> *const libc::c_char;
}


// This is usually a macro from libobs so I'll just include it here even though
// it technically kind of breaks *-sys crate standards
pub unsafe fn obs_register_source(source_info: *const obs_source_info) {
    use std::mem;

    obs_register_source_s(source_info, mem::size_of::<obs_source_info>() as libc::size_t);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
