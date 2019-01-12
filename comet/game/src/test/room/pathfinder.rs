//use room::pathfinder::{Pos, Tile, Pathfinder};
//
//struct TestMap {
//    tiles: Vec<Vec<RoomTile>>
//}
//
//struct RoomTile(i32, i32);
//
//impl Tile for RoomTile {
//    fn x(&self) -> i32 {
//        self.0
//    }
//
//    fn y(&self) -> i32 {
//        self.1
//    }
//}
//
//impl Pathfinder<RoomTile> for TestMap {
//    fn map(&self) -> Vec<Vec<RoomTile>> {
//        self.tiles.clone()
//    }
//}
//
//#[test]
//fn generate_path() {
//    let map = TestMap {
//        tiles: vec![vec![RoomTile(0, 0)]]
//    };
//
//    assert_eq!(map.move_points(), vec![
//        Pos(-1, -1),
//        Pos(0, -1),
//        Pos(1, 1),
//        Pos(0, 1),
//        Pos(1, -1),
//        Pos(1, 0),
//        Pos(-1, 1),
//        Pos(-1, 0)])
//}