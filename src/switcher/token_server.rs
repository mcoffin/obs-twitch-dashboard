use hyper;
use open;
use url;

use hyper::server::{Request, Response};
use std::sync;
use std::thread;
use std::time;
use std::io;
use std::io::Write;

use ::twitch;

/// Convenience function for writing the landing javascript page
fn write_index<'a>(res: &mut Response<'a, hyper::net::Streaming>) -> Result<usize, io::Error> {
    try!(res.write(b"<html><head><script>"));
    try!(res.write(include_str!("index.js").as_bytes()));
    res.write(b"</script><body></body></html>")
}

const TWITCH_PERMISSIONS_SCOPE: &'static str = "channel_read channel_editor";

static mut TOKEN: Option<String> = None;
static INIT_TOKEN: sync::Once = sync::ONCE_INIT;

/// Gets an OAuth2 token
pub fn authenticate() -> String {
    unsafe {
        INIT_TOKEN.call_once(|| {
            TOKEN = Some(get_token("0.0.0.0:7684", TWITCH_PERMISSIONS_SCOPE));
        });
        TOKEN.clone().unwrap()
    }
}

trait ResponseStatus {
    fn set_status(&mut self, status: hyper::status::StatusCode);
}

impl<'a> ResponseStatus for Response<'a, hyper::net::Fresh> {
    fn set_status(&mut self, status: hyper::status::StatusCode) {
        let mut status_ref = self.status_mut();
        *status_ref = status;
    }
}

#[allow(unused_must_use)]
fn get_token(bind_address: &'static str, scope: &'static str) -> String {
    use std::borrow::Borrow;

    let initial_value: Option<String> = None;
    let result = sync::Arc::new(sync::Mutex::new(initial_value));
    let cvar = sync::Arc::new(sync::Condvar::new());

    let (h_result, h_cvar) = (result.clone(), cvar.clone());
    let handler = move |req: Request, mut res: Response| {
        match req.uri {
            hyper::uri::RequestUri::AbsolutePath(s) => match s.borrow() {
                "/" => {
                    res.set_status(hyper::status::StatusCode::Ok);
                    let mut res = res.start().unwrap();
                    write_index(&mut res).unwrap();
                    res.end().unwrap();
                }
                request_uri => {
                    let url_string = format!("http://localhost:7684{}", request_uri);
                    let url = url::Url::parse(url_string.as_str()).unwrap();

                    match url.query_pairs().find(|&(ref key, _)| key.as_ref().eq("access_token")) {
                        Some((_, token)) => {
                            let mut result = h_result.lock().unwrap();
                            *result = Some(token.clone().into_owned());
                            h_cvar.notify_one();

                            res.set_status(hyper::status::StatusCode::Ok);
                            res.send(b"You can close this page now.").unwrap();
                        },
                        _ => {
                            res.set_status(hyper::status::StatusCode::NotFound);
                            res.send(b"").unwrap();
                        }
                    }
                }
            },
            _ => {
                res.set_status(hyper::status::StatusCode::NotFound);
                res.send(b"").unwrap();
            },
        }
    };

    // Create the HTTP server
    thread::spawn(move || {
        let server = hyper::Server::http(bind_address).unwrap();
        server.handle(handler).unwrap();
    });
    // Direct the user's browser at the authorization URL
    thread::spawn(move || {
        // FIXME: This is a shitty way to wait for the HTTP server to start
        thread::sleep(time::Duration::from_millis(2000));

        let auth_url = twitch::authorize_url(scope);
        open::that(auth_url.as_str());
    });

    let mut result = result.lock().unwrap();
    while (*result).is_none() {
        result = cvar.wait(result).unwrap();
    }
    let ret = (*result).clone().unwrap();
    ret
}
