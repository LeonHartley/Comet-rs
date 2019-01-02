#[derive(Debug)]
pub struct Player {
    pub avatar: PlayerAvatar,
    pub friends: Vec<PlayerAvatar>,
    pub rank: i16,
    pub achievement_points: i32,
    pub balance: PlayerBalance,
}


#[derive(Debug)]
pub struct PlayerAvatar {
    pub id: i64,
    pub name: String,
    pub figure: String,
    pub motto: String,
    pub gender: PlayerGender,
}

#[derive(Debug, Copy)]
pub enum PlayerGender {
    Male,
    Female,
}

impl From<String> for PlayerGender {
    fn from(str: String) -> Self {
        match str.as_ref() {
            "M" => PlayerGender::Male,
            _ => PlayerGender::Female
        }
    }
}

impl Into<String> for PlayerGender {
    fn into(self) -> String {
        match self {
            PlayerGender::Male => "M".to_string(),
            _ => "F".to_string()
        }
    }
}

#[derive(Copy, Debug)]
pub struct PlayerBalance {
    pub credits: i32,
    pub vip_points: i32,
    pub seasonal_points: i32,
    pub activity_points: i32,
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            avatar: self.avatar.clone(),
            friends: self.friends.clone(),
            achievement_points: self.achievement_points,
            balance: self.balance.clone(),
            rank: self.rank,
        }
    }
}

impl Clone for PlayerAvatar {
    fn clone(&self) -> Self {
        PlayerAvatar {
            id: self.id,
            name: self.name.clone(),
            figure: self.figure.clone(),
            motto: self.motto.clone(),
            gender: self.gender.clone(),
        }
    }
}

impl Clone for PlayerBalance {
    fn clone(&self) -> Self {
        *self
    }
}

impl Clone for PlayerGender {
    fn clone(&self) -> Self {
        *self
    }
}