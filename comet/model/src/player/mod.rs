#[derive(Debug)]
pub struct Player {
    pub id: i64,
    pub name: String,
    pub figure: String,
    pub motto: String,
    pub gender: PlayerGender,
    pub balance: PlayerBalance,
}

#[derive(Debug)]
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

#[derive(Copy, Debug)]
pub struct PlayerBalance {
    pub credits: i32,
    pub vip_points: i32,
    pub seasonal_points: i32,
    pub activity_points: i32,
}

impl Clone for PlayerBalance {
    fn clone(&self) -> Self {
        *self
    }
}