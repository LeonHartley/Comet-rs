use player::PlayerAvatar;

#[derive(Debug)]
pub struct PlayerFriend {
    pub avatar: PlayerAvatar,
    pub level: i16,
}

impl Clone for PlayerFriend {
    fn clone(&self) -> Self {
        PlayerFriend {
            avatar: self.avatar.clone(),
            level: self.level,
        }
    }
}