use std::convert::Into;
use std::ffi;
use std::ops::{Deref, DerefMut};

pub unsafe fn opt_ref_from_ptr_mut<'a, T>(p: *mut T) -> Option<&'a mut T> {
    use std::ptr;

    if p == ptr::null_mut() {
        None
    } else {
        Some(&mut *p)
    }
}

/// Performs a cast to a C string from a slice of bytes while checking for
/// null-termination.
pub fn checked_c_str(b: &[u8]) -> &ffi::CStr {
    ffi::CStr::from_bytes_with_nul(b).unwrap()
}

/// Represents an item from OBS that must be deallocated by OBS
pub struct OBSBox<T, F: FnMut(*mut T)> {
    item: *mut T,
    destroy_fn: Option<F>,
}

impl<T, F: FnMut(*mut T)> OBSBox<T, F> {
    /// Creates a new OBS box from an already instantiated item, and a
    /// destructor function, taking ownership of the item
    pub unsafe fn new(item: *mut T,
                      destroy_fn: F) -> OBSBox<T, F> {
        OBSBox {
            item: item,
            destroy_fn: Some(destroy_fn),
        }
    }

    /// Converts this `OBSBox` into a raw reference, consuming the box
    /// and makin the caller responsible for deallocation
    pub fn into_raw(self) -> *mut T {
        self.into()
    }
}

impl<T, F: FnMut(*mut T)> Into<*mut T> for OBSBox<T, F> {
    fn into(mut self) -> *mut T {
        // Here, we set destroy_fn to None, indicating to
        // `drop` that the contents of this box have already
        // been consumed
        self.destroy_fn = None;
        self.item
    }
}

impl<T, F: FnMut(*mut T)> Drop for OBSBox<T, F> {
    fn drop(&mut self) {
        use std::ptr;

        // If destroy_fn has been unset, then there's no need for internal
        // free'ing because the result has been consumed outside of Rust.
        match self.destroy_fn {
            Some(ref mut df) => df(self.item),
            None => {}
        }
        self.item = ptr::null_mut();
    }
}

impl<T, F: FnMut(*mut T)> Deref for OBSBox<T, F> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.item }
    }
}

impl<T, F: FnMut(*mut T)> DerefMut for OBSBox<T, F> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.item }
    }
}

#[cfg(test)]
mod tests {
    use ::mem;

    #[test]
    fn obsbox_calls_destructor() {
        let mut dummy_value: usize = 0;
        let mut destructor_call_count: usize = 0;
        {
            let destructor = |_| destructor_call_count = destructor_call_count + 1;
            unsafe { mem::OBSBox::new(&mut dummy_value as *mut usize, destructor) };
        }
        assert_eq!(destructor_call_count, 1);
    }

    #[test]
    fn obsbox_doesnt_call_destructor_if_moved() {
        let mut dummy_value: usize = 0;
        let mut destructor_call_count: usize = 0;
        {
            let destructor = |_| destructor_call_count = destructor_call_count + 1;
            unsafe { mem::OBSBox::new(&mut dummy_value as *mut usize, destructor).into_raw() };
        }
        assert_eq!(destructor_call_count, 0);
    }
}
