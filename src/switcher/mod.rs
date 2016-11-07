use hyper;
use obs;
use obs_sys;
use serde_json;

mod token_server;

use obs::{Output, Service, Data, Properties};

use std::fmt::Display;

use ::twitch::channel::{Channel, ChannelInfo, ChannelUpdate};

pub struct DashboardSwitcher {
    status: String,
    game: String,
    hyper: hyper::Client,
}

impl obs::source::Source for DashboardSwitcher {
    #[inline(always)]
    fn id() -> &'static str {
        "obs-twitch-dashboard-switcher\0"
    }

    #[inline(always)]
    fn typ() -> obs_sys::obs_source_type {
        obs_sys::obs_source_type::INPUT
    }

    #[inline(always)]
    fn output_flags() -> u32 {
        0
    }

    fn create(settings: &mut obs_sys::obs_data_t,
              _: &mut obs_sys::obs_source_t) -> DashboardSwitcher {
        let mut ret = DashboardSwitcher::new("", "");
        ret.update(settings);
        ret
    }

    fn get_name(&self) -> &str {
        "Twitch Dashboard Switcher\0"
    }

    fn get_width(&mut self) -> u32 {
        0
    }

    fn get_height(&mut self) -> u32 {
        0
    }
}

const KEY_STATUS: &'static str = "status\0";
const KEY_GAME: &'static str = "game\0";

/// Retreives the stream key of a currently active twitch stream
fn current_stream_key() -> Option<String> {
    let mut key: Option<String> = None;
    obs::enum_outputs(|output| {
        // Look for only active outputs
        if output.active() {
            let maybe_key = output.get_service().and_then(|service| service.get_key());
            match maybe_key {
                Some(k) => {
                    key = Some(String::from(k));
                    false
                },
                None => true
            }
        } else {
            true
        }
    });
    key
}

header! {
    (Authorization, "Authorization") => [String]
}
header! {
    (ContentType, "Content-Type") => [String]
}

fn err_string<S: AsRef<str>, E: Display>(maybe_prefix: Option<S>, e: E) -> String {
    match maybe_prefix {
        Some(prefix) => {
            let p: &str = prefix.as_ref();
            format!("{}: {}", p, e)
        },
        None => format!("{}", e)
    }
}

impl DashboardSwitcher {
    pub fn new<S: Into<String>, G: Into<String>>(status: S,
                                                 game: G) -> DashboardSwitcher {
        DashboardSwitcher {
            status: status.into(),
            game: game.into(),
            hyper: hyper::Client::new(),
        }
    }
    pub fn source_info() -> obs_sys::obs_source_info {
        use std::mem::transmute;

        let mut src_info = obs::source::source_info::<DashboardSwitcher>();
        src_info.get_defaults = Some(unsafe {
            transmute(DashboardSwitcher::get_defaults as *const extern fn(&mut obs_sys::obs_data_t))
        });
        src_info.get_properties = Some(unsafe {
            transmute(DashboardSwitcher::get_properties as *const extern fn(&DashboardSwitcher) -> *mut obs_sys::obs_properties_t)
        });
        src_info.update = Some(unsafe {
            transmute(DashboardSwitcher::update as *const extern fn(&mut DashboardSwitcher, &mut obs_sys::obs_data_t))
        });
        src_info.activate = Some(unsafe {
            transmute(DashboardSwitcher::activate as *const extern fn(&DashboardSwitcher))
        });
        src_info
    }

    fn channel_info(&self) -> Channel {
        Channel {
            status: Some(self.status.clone()),
            game: Some(self.game.clone()),
        }
    }

    extern "C" fn get_defaults(settings: &mut obs_sys::obs_data_t) {
        let manditory_keys = [KEY_STATUS, KEY_GAME];
        for k in manditory_keys.iter() {
            settings.set_default_string(k, "\0");
        }
    }

    extern "C" fn update(&mut self, settings: &mut obs_sys::obs_data_t) {
        self.status = settings.get_string(KEY_STATUS).to_string();
        self.game = settings.get_string(KEY_GAME).to_string();
    }

    extern "C" fn get_properties(&self) -> *mut obs_sys::obs_properties_t {
        let mut props = obs::properties::create();

        props.add_text(KEY_STATUS, "Channel Status\0", obs_sys::obs_text_type::DEFAULT);
        props.add_text(KEY_GAME, "Game\0", obs_sys::obs_text_type::DEFAULT);
        props.set_flags(obs::properties::DEFER_UPDATE);

        props.into_raw()
    }

    /// Called when this scene is `activated` for display on on the main screen
    extern "C" fn activate(&self) {
        use std::io::Read;
        use twitch::TwitchRequest;

        match current_stream_key() {
            Some(ref s) => {
                debug!("We're streaming (stream_key = {})", s);
                let body = {
                    let update_info: ChannelUpdate = self.channel_info().into();
                    serde_json::to_string(&update_info).unwrap()
                };
                let t = token_server::authenticate();
                let auth_header = || Authorization(format!("OAuth {}", t));
                let channel_name_response = {
                    let endpoint = "https://api.twitch.tv/kraken/channel";
                    self.hyper.get(endpoint)
                        .header(auth_header())
                        .twitch_headers()
                        .send()
                        .map_err(|e| err_string(Some("Error calling the twitch API"), e))
                        .and_then(|mut channel_name_res| {
                            let mut buf = String::from("");
                            channel_name_res.read_to_string(&mut buf)
                                .map(|_| buf)
                                .map_err(|e| err_string(Some("Error reading response body from twitch API"), e))
                        })
                        .and_then(|res_body| {
                            debug!("Got response for ChannelInfo: {}", &res_body);
                            serde_json::from_str::<ChannelInfo>(res_body.as_ref()).map_err(|e| err_string(Some("Error deserializing JSON from twitch API"), e))
                        })
                        .map(|channel_info| channel_info.name)
                };
                let response = channel_name_response.and_then(|channel_name| {
                    let endpoint = format!("https://api.twitch.tv/kraken/channels/{}", channel_name);
                    debug!("Calling twitch API: {}\n    body: {}", endpoint, &body);
                    self.hyper.put(&endpoint)
                        .header(auth_header())
                        .header(ContentType("application/json".to_string()))
                        .twitch_headers()
                        .body(body.as_bytes())
                        .send()
                        .map_err(|e| err_string(Some("Error calling the twitch API"), e))
                        .and_then(|res| {
                            use hyper::status::StatusCode;

                            match res.status {
                                StatusCode::Ok => Ok(res),
                                _ => Err(err_string(Some("Got bad HTTP status from twitch API"), res.status))
                            }
                        })
                });
                match response {
                    Ok(..) => debug!("Success in calling twitch API!"),
                    Err(e) => error!("{}", e),
                }
            },
            None => {
                debug!("We're not streaming");
            },
        };
    }
}
