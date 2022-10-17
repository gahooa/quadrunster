
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
    let x = ((x+scroll)/16.0) as usize;
    let y = ((sh-(y-16.0))/16.0) as usize;

    if x < LEVEL_WIDTH && y < LEVEL_HEIGHT{
        Some((x, y))
    }
    else{
        None
    }
}
