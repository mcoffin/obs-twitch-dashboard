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

// TODO: This module should really be its own crate / project but is necessary
// because const functions are broken cross-crate

//! Contains bindings for libobs

#[derive(Clone)]
struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32
}

const OBS_API_VERSION: Version = Version { major: 0, minor: 16, patch: 4 };

/// API version of OBS as formatted in LIBOBS_API_VERSION macro in the C version
/// of libobs
pub const API_VERSION: u32 = obs_semver(OBS_API_VERSION.major,
                                        OBS_API_VERSION.minor,
                                        OBS_API_VERSION.patch);

/// Formats a major, minor, and patch version in to a single 32 bit integer
/// version of a semantic version for use with libobs
pub const fn obs_semver(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 24) | (minor << 16) | patch
}
