// Copyright 2016 Matt Coffin
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//    http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Root module for this OBS plugin

#![feature(const_fn, proc_macro, drop_types_in_const)]

extern crate libc;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate obs;
extern crate obs_sys;
extern crate open;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate url;
#[macro_use] extern crate hyper;

mod obs_ffi;
mod switcher;
mod twitch;
mod url_open;

/// Contains lifecycle callbacks for an OBS module
pub mod lifecycle {
    use env_logger;
    use libc;
    use obs;
    use obs::mem::checked_c_str;

    use ::obs_ffi;
    use ::switcher;

    /// Contains the module pointer set by OBS
    static mut MODULE_PTR: Option<usize> = None;

    #[no_mangle]
    pub unsafe extern "C" fn obs_module_set_pointer(ptr: usize) {
        MODULE_PTR = Some(ptr);
    }

    #[no_mangle]
    pub unsafe extern "C" fn obs_current_module() -> usize {
        // We will just panic here if the MODULE_PTR is unset
        MODULE_PTR.unwrap()
    }

    #[no_mangle]
    pub extern "C" fn obs_module_ver() -> u32 {
        obs_ffi::API_VERSION
    }

    #[no_mangle]
    pub extern "C" fn obs_module_load() -> bool {
        // FIXME: This will break if another rust module does the same thing
        // first
        env_logger::init().unwrap();

        let src_info = switcher::DashboardSwitcher::source_info();
        obs::source::register_source(&src_info);

        debug!("Module loaded.");
        true
    }

    #[no_mangle]
    pub extern "C" fn obs_module_unload() {
    }

    #[no_mangle]
    pub extern "C" fn obs_module_name() -> *const libc::c_char {
        checked_c_str(b"obs-twitch-dashboard\0").as_ptr()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
