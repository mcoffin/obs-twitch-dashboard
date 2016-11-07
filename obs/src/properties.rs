use libc;
use obs_sys;

use ::mem::OBSBox;

pub const DEFER_UPDATE: u32 = 1;

/// Creates a new owned `obs_properties_t` object, wrapped (hopefully
/// transparently) in an `OBSBox`.
pub fn create() -> OBSBox<obs_sys::obs_properties_t, impl FnMut(*mut obs_sys::obs_properties_t)> {
    // TODO: might this actually be less efficient than just making a full-blown
    // fn at the top level? All we're really doing is unwrapping the externality
    // and the unsafe nature of `obs_properties_destroy`
    let destroy = |props| unsafe { obs_sys::obs_properties_destroy(props) };
    unsafe {
        let item = obs_sys::obs_properties_create();
        OBSBox::new(item, destroy)
    }
}

/// Wrapper methods that should be available for `obs_properties_t` objects.
pub trait Properties {
    fn add_text(&mut self,
                key: &str,
                value: &str,
                typ: obs_sys::obs_text_type);
    fn set_flags(&mut self,
                 flags: u32);
}

impl Properties for obs_sys::obs_properties_t {
    fn add_text(&mut self,
                key: &str,
                description: &str,
                typ: obs_sys::obs_text_type) {
        unsafe {
            obs_sys::obs_properties_add_text(
                self as *mut obs_sys::obs_properties_t,
                key.as_ptr() as *const libc::c_char,
                description.as_ptr() as *const libc::c_char,
                typ);
        }
    }

    fn set_flags(&mut self,
                 flags: u32) {
        unsafe {
            obs_sys::obs_properties_set_flags(
                self as *mut obs_sys::obs_properties_t,
                flags)
        }
    }
}

#[cfg(test)]
mod tests {
    use ::properties;

    #[test]
    fn properties_create_and_destroy() {
        properties::create();
    }
}
