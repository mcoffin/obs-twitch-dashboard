# obs-twitch-dashboard

[OBS-Studio](https://github.com/jp9000/obs-studio) plugin for providing the functions of the twitch dashboard from within OBS.

Also an experiment in [Rust](http://rust-lang.org) FFI.

This is pre-release software and should be treated as such.

## Building

`obs-twitch-dashboard` currently uses a few unstable features of rust, and therefore must be built with the *nightly* toolchain.

`obs-twitch-dashboard` is built using [Cargo](https://crates.io).

```bash
cargo build --release
```

This will build a `libobs_twitch_dashboard.so` (or similar `.dll` or `.dylib`). Simply copy this file in to your `obs-plugins` folder (ex. `/usr/lib/obs-plugins`) to install the plugin, then re-launch OBS. 

## Licence

`obs-twitch-dashboard` is released under the Apache Licence 2.0. See LICENSE file for details.

```
Copyright 2016 Matt Coffin

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```
