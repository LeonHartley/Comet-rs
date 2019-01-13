use std::collections::VecDeque;

pub trait Tile {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn can_walk(&self) -> bool { true }
}

pub trait Pathfinder<Node> where Node: Tile {
    fn map(&self) -> Vec<Vec<Node>>;

    fn find_path(&self, to: Node) -> VecDeque<Node> {
        self
            .find_path_reversed(to)
            .into_iter()
            .rev()
            .collect()
    }

    fn find_path_reversed(&self, to: Node) -> VecDeque<Node> {
        VecDeque::new()
    }

    fn move_points(&self) -> Vec<Pos> {
        vec![
            Pos(-1, -1),
            Pos(0, -1),
            Pos(1, 1),
            Pos(0, 1),
            Pos(1, -1),
            Pos(1, 0),
            Pos(-1, 1),
            Pos(-1, 0)]
    }
}

#[derive(Eq, PartialEq, Debug, Copy)]
pub struct Pos(pub i32, pub i32);

impl Clone for Pos {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl Tile for Pos {
    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
        self.1
    }
}