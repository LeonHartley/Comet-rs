#[derive(Debug)]
pub struct RoomModel {
    pub tiles: Vec<Vec<Tile>>,
    pub size_x: usize,
    pub size_y: usize,
    pub door_pos: Pos,
}

#[derive(Debug)]
pub enum TileState {
    Open,
    Closed,
    Redirect,
    Door,
}

#[derive(Debug)]
pub struct Tile {
    pub state: TileState,
    pub height: i32,
}

#[derive(Debug)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub z: f64,
}

