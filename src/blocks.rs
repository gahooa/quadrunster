
use crate::common::{
    LEVEL_HEIGHT,
    LEVEL_WIDTH,
};
#[derive(Copy, Clone)]
pub enum BlockType {
    Empty,
    Rock,
    Water,
    Lava {heat: f32},
    Spawn,
}

pub struct Block{
    pub x: usize,
    pub y: usize,
    pub block_type: BlockType,
}

impl Block{
    pub fn new(x: usize, y: usize, block_type: BlockType) -> Self{
        Self{
            x,
            y,
            block_type,
        }
    }
}

pub fn mouse_to_block_xy(x: f32, y: f32, scroll: f32, sh: f32) -> Option<(usize, usize)>{
    let bx = ((x+scroll)/16.0) as usize;
    let by = ((1024.0-(y-16.0))/16.0) as usize;

    if bx < LEVEL_WIDTH && by < LEVEL_HEIGHT && y < 1038.0 {
        Some((bx, by))
    }
    else {
        None
    }
}
