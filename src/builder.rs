

mod common;
mod blocks;
use common::{
    LEVEL_HEIGHT,
    LEVEL_WIDTH,
};

use blocks::{
    BlockType,
    Block,
    mouse_to_block_xy,
};

use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};






fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Quad Runster"),
        window_width: 1024,
        window_height: 1024,
        fullscreen: false,
        ..Default::default()
    }
}

fn get_block(x: u8, y: u8, level: &Vec<Block>) -> &Block{
    &level[(x as usize) + (y as usize) * LEVEL_WIDTH]
}



#[macroquad::main(window_conf)]
async fn main() {
    common::foo();

    let mut rng = thread_rng();
    let mut scroll:f32 = 0.0;
    let mut level_blocks: Vec<Block> = Vec::new();


    for x in 0..LEVEL_WIDTH{
        for y in 0..LEVEL_HEIGHT{
            let block_type = if rng.gen_range(0..100) < 2{
                BlockType::Rock
            }else {
                if rng.gen_range(0..100) < 2{
                    BlockType::Water
                }else{
                    BlockType::Empty
                }
            };
            level_blocks.push(Block::new(x, y, block_type));
        }
    }
    

    loop {


        let (mouse_x, mouse_y) = mouse_position();
        let sh = screen_height();
        let bxy = mouse_to_block_xy(mouse_x, mouse_y, scroll, sh);
                
        
        if is_key_down(KeyCode::A){
            if is_key_down(KeyCode::LeftShift){
                scroll -= 64.0;
            }
            else {
                scroll -= 16.0;
            }
            if scroll < 0.0{
                scroll = 0.0;
            }
        }
        if is_key_down(KeyCode::D){
            if is_key_down(KeyCode::LeftShift){
                scroll += 64.0;
            }
            else {
                scroll += 16.0;
            }
            if scroll > (LEVEL_WIDTH*16) as f32 - screen_width(){
                scroll = (LEVEL_WIDTH*16) as f32 - screen_width();
            }
        }
        
        if let Some((bx, by)) = bxy {
            if is_mouse_button_down(MouseButton::Left){
                level_blocks[(bx + by * LEVEL_WIDTH)].block_type = BlockType::Lava{heat: 0.0};
                println!("Block: {:?} {:?}", bx, by);
            }
            if is_mouse_button_down(MouseButton::Right){
                level_blocks[(bx + by * LEVEL_WIDTH)].block_type = BlockType::Empty;
                println!("Block: {:?} {:?}", bx, by);
            }

        }



        
        
        clear_background(BLACK);
        for x in 0..LEVEL_WIDTH{
            for y in 0..LEVEL_HEIGHT{
                let block = get_block(x as u8, y as u8, &level_blocks);
                let block_color = match block.block_type{
                    BlockType::Empty => BLACK,
                    BlockType::Rock => GRAY,
                    BlockType::Water => BLUE,
                    BlockType::Lava{heat: _} => RED,
                };
                draw_rectangle(x as f32 * 16.0-scroll+1.0, sh - y as f32 * 16.0+1.0, 14.0, 14.0, block_color);
            }
        }

        if let Some((bx, by)) = bxy{
            draw_rectangle_lines(bx as f32 * 16.0-scroll, sh - by as f32 * 16.0, 16.0, 16.0, 2.0, WHITE);
            draw_line(0.0, sh - by as f32 * 16.0+8.0, screen_width(), sh - by as f32 * 16.0+8.0, 1.0, GRAY);
            draw_line(bx as f32 * 16.0-scroll+8.0, 0.0, bx as f32 * 16.0-scroll+8.0, screen_height(), 1.0, GRAY);
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