#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    pub status: Option<String>,
    pub game: Option<String>,
}

impl Into<ChannelUpdate> for Channel {
    fn into(self) -> ChannelUpdate {
        ChannelUpdate::new(self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelUpdate {
    channel: Channel,
}

impl ChannelUpdate {
    fn new(channel: Channel) -> ChannelUpdate {
        ChannelUpdate {
            channel: channel,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelInfo {
    pub name: String,
}
