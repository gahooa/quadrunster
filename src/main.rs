
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

enum BlockType {
    Empty,
    Rock,
}

const LEVEL_WIDTH: usize = 256;
const LEVEL_HEIGHT: usize = 32;

struct Block{
    x: u8,
    y: u8,
    block_type: BlockType,
}

impl Block{
    fn new(x: u8, y: u8, block_type: BlockType) -> Self{
        Self{
            x,
            y,
            block_type,
        }
    }

}


fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Quad Runster"),
        window_width: 2048,
        window_height: 1024,
        fullscreen: false,
        ..Default::default()
    }
}

fn get_block(x: u8, y: u8, level: &Vec<Block>) -> &Block{
    &level[(x as usize) + (y as usize) * LEVEL_WIDTH]
}

fn mouse_to_block(x: f32, y: f32, scroll: f32, sh: f32) -> (usize, usize){
    let x = ((x+scroll)/32.0) as usize;
    let y = ((sh-y)/32.0) as usize;
    (x, y)
}

#[macroquad::main(window_conf)]
async fn main() {
    
    let mut rng = thread_rng();
    let mut scroll:f32 = 0.0;
    let mut level_blocks: Vec<Block> = Vec::new();

    for x in 0..LEVEL_WIDTH{
        for y in 0..LEVEL_HEIGHT{
            let block_type = if rng.gen_range(0..100) < 10{
                BlockType::Rock
            }else{
                BlockType::Empty
            };
            level_blocks.push(Block::new(x as u8, y as u8, block_type));
        }
    }
    

    loop {

        scroll += 1.0;

        let (mouse_x, mouse_y) = mouse_position();
        let sh = screen_height();
        let (bx,by) = mouse_to_block(mouse_x, mouse_y, scroll, sh);
                
        if is_mouse_button_pressed(MouseButton::Left){
            let block = get_block(bx as u8, by as u8, &level_blocks);
            println!("Block: {:?} {:?}", bx, by);
        }
        
        
        clear_background(BLACK);
        for x in 0..LEVEL_WIDTH{
            for y in 0..LEVEL_HEIGHT{
                let block = get_block(x as u8, y as u8, &level_blocks);
                let block_color = match block.block_type{
                    BlockType::Empty => BLACK,
                    BlockType::Rock => GRAY,
                };
                draw_rectangle(x as f32 * 32.0-scroll, sh - y as f32 * 32.0, 32.0, 32.0, block_color);
            }
        }

        next_frame().await
    }


}



/*

    let mut points:Vec<Point> = vec![];
    
    let mut fps = 0;
    let mut increment:i64 = 0;

    loop {
        increment += 1;
        if increment % 30 == 0 {
            fps = get_fps();
        }

        let (mouse_x, mouse_y) = mouse_position();
        
        let gravity = is_mouse_button_down(MouseButton::Right);
        let g = vec2(mouse_x, mouse_y);
        
        if is_mouse_button_down(MouseButton::Left) {
            for _ in 0..10 {
                let vx:f32 = rng.gen_range(-1.0..1.0);
                let vy:f32 = rng.gen_range(-1.0..1.0);
                let vr:f32 = rng.gen_range(-4.0..4.0);
                points.push(Point::new(mouse_x, mouse_y, 0.0, vx, vy, vr, YELLOW ));
            }
        }

        let sw = screen_width();
        let sh = screen_height();

        points.retain(|p| p.x > 0.0 && p.x < sw && p.y > 0.0 && p.y < sh);

        for point in points.iter_mut(){
            if gravity {
                let p = vec2(point.x, point.y);
                let v = g - p;
                let q = (1.0 / (p.distance(g).powf(1.5).max(-1.0))) * 10.0;

                point.vx += v.x * q;
                point.vy += v.y * q;
            }

            point.vx *= 0.99;
            point.vy *= 0.99;
            point.update();
        }
        



        //clear_background(BLACK);

        if gravity {
            draw_circle(mouse_x, mouse_y, 10.0, BLUE);
        }

        for point in points.iter(){
            draw_poly_lines(point.x, point.y, 3, 7.0, point.r, 1.0, point.color);
        }

        draw_rectangle(0.0,0.0, screen_width(), 60.0, GRAY);
        draw_text(format!("There are {} objects, running at {} fps", points.len(), fps).as_str(), 20.0, 50.0, 50.0, WHITE);

        

        next_frame().await;
    }


*/