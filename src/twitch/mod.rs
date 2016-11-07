use hyper;
use url;

pub mod channel;

header! {
    (Accept, "Accept") => [String]
}
header! {
    (ClientID, "Client-ID") => [String]
}

const CLIENT_ID: &'static str = include_str!("client_id.txt");

pub trait TwitchRequest<'a> {
    fn twitch_headers(self) -> hyper::client::RequestBuilder<'a>;
}

impl<'a> TwitchRequest<'a> for hyper::client::RequestBuilder<'a> {
    fn twitch_headers(self) -> hyper::client::RequestBuilder<'a> {
        debug!("Twitch client id: {}", CLIENT_ID);
        self.header(Accept("application/vnd.twitchtv.v3+json".to_string()))
            .header(ClientID(CLIENT_ID.to_string()))
    }
}

pub fn authorize_url(scope: &str) -> url::Url {
    use url::Url;

    let mut auth_url = Url::parse("https://api.twitch.tv/kraken/oauth2/authorize").unwrap();
    auth_url.query_pairs_mut().append_pair("client_id", CLIENT_ID);
    auth_url.query_pairs_mut().append_pair("response_type", "token");
    auth_url.query_pairs_mut().append_pair("redirect_uri", "http://localhost:7684");
    auth_url.query_pairs_mut().append_pair("scope", scope);
    auth_url
}

#[cfg(test)]
mod tests {
    use hyper;
    use url;

    use super::*;

    #[test]
    fn authorize_url_passes_validation() {
        let auth_url = authorize_url("channel_read channel_editor");
        url::Url::parse(auth_url.as_str()).unwrap();
    }
}
