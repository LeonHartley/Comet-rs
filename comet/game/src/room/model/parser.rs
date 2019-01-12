use model::room::map::{Pos, RoomModel, TileState};
use model::room::map::Tile;

pub trait ModelParser: Sized {
    fn into_tile_map(self, door_pos: Pos) -> RoomModel {
        let tiles = self.parse_tiles(&door_pos);

        RoomModel {
            size_x: tiles[0].len(),
            size_y: tiles.len(),
            tiles,
            door_pos,
        }
    }

    fn parse_tiles(&self, door: &Pos) -> Vec<Vec<Tile>> {
        let mut map = Vec::new();
        let mut map_data = self.tile_data();

        for (y, axis) in map_data.iter().enumerate() {
            map.insert(y, Vec::new());

            for (x, t) in axis.iter().enumerate() {
                map[y].insert(x, create_tile(x as i32, y as i32, *t, door));
            }
        }

        map
    }

    fn tile_data(&self) -> Vec<Vec<u8>>;
}

impl ModelParser for String {
    fn tile_data(&self) -> Vec<Vec<u8>> {
        self.clone()
            .replace("\r", "\n").split("\n")
            .map(|a| String::from(a).into_bytes())
            .collect()
    }
}

fn create_tile(x: i32, y: i32, state: u8, door: &Pos) -> Tile {
    Tile {
        state: if x == door.x && y == door.y {
            TileState::Door
        } else {
            match state {
                b'x' => {
                    TileState::Closed
                }
                _ => {
                    TileState::Open
                }
            }
        },
        height: get_tile_height(state),
    }
}

pub fn get_tile_height(tile: u8) -> i32 {
    if tile >= b'0' && tile <= b'9' {
        (tile as i32) - 48
    } else if tile >= b'A' && tile <= b'Z' {
        (tile as i32) - 65
    } else if tile >= b'a' && tile <= b'z' {
        (tile as i32) - 87
    } else { 0 }
}