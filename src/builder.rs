mod common;
mod blocks;

use std::{process::exit, io::Empty};

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
        window_resizable: false, // don't know if you want this
        ..Default::default()
    }
} //window config

fn get_block(x: u8, y: u8, level: &Vec<Block>) -> &Block {
    &level[(x as usize) + (y as usize) * LEVEL_WIDTH]
}


struct Game {
    scroll: f32,
    level_blocks: Vec<Block>,
    build_block: BlockType,
    gamestate: GameState,
    menustate: Option<MenuState>,
    font: Font,
}

impl Game {
    async fn start() -> Self {
        common::foo();
        let mut scroll:f32 = 0.0;
        let mut level_blocks: Vec<Block> = Vec::new();
        let mut gamestate: GameState = GameState::Edit;
        
        let mut rng = thread_rng();

        for x in 0..LEVEL_WIDTH {
            for y in 0..LEVEL_HEIGHT {
                level_blocks.push(Block::new(x, y, BlockType::Empty));
            }
        }

        let font = load_ttf_font("./src/resources/Flamenco-Regular.ttf").await.unwrap();

        Self {
            scroll,
            level_blocks,
            build_block: BlockType::Lava { heat: 15.0 },
            gamestate,
            menustate: None,
            font,
        }
        
        
    }

    fn tick(&mut self) {
        match self.gamestate {
            GameState::Menu => {self.menutick()},
            GameState::Edit => {self.edittick()},
            GameState::Play => {self.playtick()},
        }
    }

    fn playtick(&mut self) {
        if is_key_down(KeyCode::Escape) {
            self.gamestate = GameState::Menu;
        }
    }

    fn menutick(&mut self) {
        match self.menustate {
            None => {
                self.menustate = Some(MenuState::Switcher)
            },

            Some(_) => {
                draw_smooth(screen_width()/2.0-190.0, screen_height()/2.0-300.0, 400.0, 180.0, 20.0, DARKGRAY);
                draw_smooth(screen_width()/2.0-195.0, screen_height()/2.0-310.0, 400.0, 180.0, 20.0, WHITE);
                draw_smooth(screen_width()/2.0-190.0, screen_height()/2.0-95.0, 400.0, 180.0, 20.0, DARKGRAY);
                draw_smooth(screen_width()/2.0-195.0, screen_height()/2.0-105.0, 400.0, 180.0, 20.0, WHITE);
                draw_smooth(screen_width()/2.0-190.0, screen_height()/2.0+110.0, 400.0, 180.0, 20.0, DARKGRAY);
                draw_smooth(screen_width()/2.0-195.0, screen_height()/2.0+100.0, 400.0, 180.0, 20.0, WHITE);
                draw_text_ex("edit", screen_width()/2.0-125.0, screen_height()/2.0-180.0, TextParams {font_size: 150, font: self.font, color: BLACK, ..Default::default()},);
                draw_text_ex("play", screen_width()/2.0-125.0, screen_height()/2.0+20.0, TextParams {font_size: 150, font: self.font, color: BLACK, ..Default::default()},);
                draw_text_ex("quit", screen_width()/2.0-125.0, screen_height()/2.0+220.0, TextParams {font_size: 150, font: self.font, color: BLACK, ..Default::default()},);
                if is_mouse_button_down(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    if mx > screen_width()/2.0-200.0 && mx < screen_width()/2.0+200.0 {
                        if my > screen_height()/2.0-300.0 && my < screen_height()/2.0-120.0 {
                            self.gamestate = GameState::Edit;
                        } else if my > screen_height()/2.0-95.0 && my < screen_height()/2.0+75.0 {
                            self.gamestate = GameState::Play;
                        } else if my > screen_height()/2.0+110.0 && my < screen_height()/2.0+290.0 {
                            exit(490)
                        }
                    }
                }
            },
        }
    }

    fn edittick(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        let sh = screen_height();
        let bxy = mouse_to_block_xy(mouse_x, mouse_y, self.scroll, sh);
                
        if is_key_down(KeyCode::Escape) {
            self.gamestate = GameState::Menu;
        }

        if is_key_down(KeyCode::A) {
            if is_key_down(KeyCode::LeftShift) {
                self.scroll -= 64.0;
            }
            else {
                self.scroll -= 16.0;
            }
            if self.scroll < 0.0{
                self.scroll = 0.0;
            }
        }
        if is_key_down(KeyCode::D) {
            if is_key_down(KeyCode::LeftShift) {
                self.scroll += 64.0;
            }
            else {
                self.scroll += 16.0;
            }
            if self.scroll > (LEVEL_WIDTH*16) as f32 - screen_width() {
                self.scroll = (LEVEL_WIDTH*16) as f32 - screen_width();
            }
        }
        
        if let Some((bx, by)) = bxy {
            if is_mouse_button_down(MouseButton::Left){
                self.level_blocks[(bx + by * LEVEL_WIDTH)].block_type = self.build_block;
                
            }
            if is_mouse_button_down(MouseButton::Right){
                self.level_blocks[(bx + by * LEVEL_WIDTH)].block_type = BlockType::Empty;
                
            }

        }



        
        
        clear_background(BLACK);
        for x in 0..LEVEL_WIDTH {
            for y in 0..LEVEL_HEIGHT {
                let block = get_block(x as u8, y as u8, &self.level_blocks);
                let block_color = match block.block_type{
                    BlockType::Empty => BLACK,
                    BlockType::Rock => GRAY,
                    BlockType::Water => BLUE,
                    BlockType::Lava{heat: _} => RED,
                };
                draw_rectangle(x as f32 * 16.0-self.scroll+1.0, sh - y as f32 * 16.0+1.0, 14.0, 14.0, block_color);
            }
        }

        if let Some((bx, by)) = bxy{
            draw_rectangle_lines(bx as f32 * 16.0-self.scroll, sh - by as f32 * 16.0, 16.0, 16.0, 2.0, WHITE);
            draw_line(0.0, sh - by as f32 * 16.0+8.0, screen_width(), sh - by as f32 * 16.0+8.0, 1.0, GRAY);
            draw_line(bx as f32 * 16.0-self.scroll+8.0, 0.0, bx as f32 * 16.0-self.scroll+8.0, screen_height(), 1.0, GRAY);
        }

        draw_outlined_rectangle(screen_width()/2.0-25.0, screen_height()-60.0, 50.0, 50.0, 3.0, WHITE);
        draw_rectangle(screen_width()/2.0-10.0, screen_height()-45.0, 20.0, 20.0, 
        match self.build_block {
            BlockType::Lava { heat: _ } => RED,
            BlockType::Water => BLUE,
            BlockType::Rock => GRAY,
            _ => BLACK,
        });

        if is_key_pressed(KeyCode::Key1) {
            self.build_block = BlockType::Lava { heat: 15.0 }
        } else if is_key_pressed(KeyCode::Key2) {
            self.build_block = BlockType::Water
        } else if is_key_pressed(KeyCode::Key3) {
            self.build_block = BlockType::Rock
        }

    }

}

fn draw_smooth(x: f32, y: f32, w: f32, h: f32, s: f32, color: Color) {
    draw_rectangle(x+s, y, w-2.0*s, h, color);
    draw_rectangle(x, y+s, w, h-2.0*s, color);
    draw_poly(x+s, y+s, s as u8, s, 0.0, color);
    draw_poly(x+w-s, y+h-s, s as u8, s, 0.0, color);
    draw_poly(x+s, y+h-s, s as u8, s, 0.0, color);
    draw_poly(x+w-s, y+s, s as u8, s, 0.0, color);
}

fn draw_outlined_rectangle(x: f32, y: f32, w: f32, h: f32, t: f32, color: Color) {
    draw_rectangle(x+w-t, y, t, h, color);
    draw_rectangle(x, y, w, t, color);
    draw_rectangle(x, y, t, h, color);
    draw_rectangle(x, y+w-t, w, t, color);
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::start().await;
    loop { game.tick(); next_frame().await }
}

enum GameState {
    Menu,
    Edit,
    Play,
}

#[derive(Copy, Clone)]
enum MenuState {
    Switcher,
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