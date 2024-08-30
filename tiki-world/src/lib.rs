pub struct Node {
    pub id: u16,
    pub param1: u8,
    pub param2: u8,
}

pub type Pos = glam::IVec3;

pub fn pos(x: i32, y: i32, z: i32) -> Pos {
    Pos::new(x, y, z)
}

pub trait Map {
    fn get_node(&self, pos: Pos) -> Node;
}
