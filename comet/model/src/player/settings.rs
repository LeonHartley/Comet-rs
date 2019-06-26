#[derive(Debug)]
pub struct PlayerSettings {
    pub allow_trade: bool,
    pub player_invisible: bool,
    pub player_invisible_room: bool,
    pub home_room: i64,
    pub old_chat_style: bool,
    pub messenger: MessengerSettings,
    pub navigator: NavigatorSettings,
}

#[derive(Debug)]
pub struct MessengerSettings {
    pub allow_follow: bool,
    pub allow_friend_requests: bool,
}

#[derive(Debug)]
pub struct NavigatorSettings {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub searches_visible: bool,
}

impl Clone for NavigatorSettings {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            searches_visible: self.searches_visible,
        }
    }
}

impl Clone for PlayerSettings {
    fn clone(&self) -> Self {
        Self {
            allow_trade: self.allow_trade,
            player_invisible: self.player_invisible,
            player_invisible_room: self.player_invisible_room,
            home_room: self.home_room,
            old_chat_style: self.old_chat_style,
            messenger: MessengerSettings {
                allow_follow: self.messenger.allow_follow,
                allow_friend_requests: self.messenger.allow_friend_requests,
            },
            navigator: NavigatorSettings {
                x: self.navigator.x,
                y: self.navigator.y,
                width: self.navigator.width,
                height: self.navigator.height,
                searches_visible: self.navigator.searches_visible,
            },
        }
    }
}
