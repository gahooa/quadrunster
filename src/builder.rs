const MAP_SIZE: BlockPosition = (100, 53);
use macroquad::{prelude::*, ui::widgets::Button as OtherButton};

type BlockPosition = (usize, usize);

struct Global {
    state: State,
    buttons: Vec<Button>,
    map: Map,
}

impl Global {
    fn init() -> Self {
        let mut init = Self {
            state: State {statetype: StateType::Menu, tickcount: 0},
            buttons: Vec::from([Button::edit_menu(), Button::play_menu()]),
            map: Map::new(),
        };

        //init.

        init
    }

    fn tick(&mut self) {
        match &self.state.statetype {
            StateType::Menu => {
                
            },

            StateType::Edit(mut editor) => {
                self.map.draw(editor.scroll);
                let tick = editor.tick(false);
                if tick.is_some() {
                    let unwrapped_tick = tick.unwrap();
                    if unwrapped_tick.0+unwrapped_tick.1*MAP_SIZE.0 < self.map.blocks.len() {
                        self.map.set_block(unwrapped_tick.0 as u8, unwrapped_tick.1 as u8, Block::new(editor.brushtype));
                    }
                    
                }
                
                draw_line(0.0, MAP_SIZE.1 as f32*10.0+2.0, MAP_SIZE.0 as f32*10.0, MAP_SIZE.1 as f32*10.0+2.0, 1.0, WHITE);
            },

            StateType::Play(mut player) => {
                self.map.draw(
                    if player.position.x < screen_width()/2.0 {
                        0.0
                    } else if MAP_SIZE.0 as f32*10.0-player.position.x > screen_width()/2.0 {
                        MAP_SIZE.0 as f32*10.0-screen_width()
                    } else {
                        player.position.x-screen_width()/2.0
                    }
                );

                player.tick();
            },
        }

        let mut on_button = false;
        for button in 0..self.buttons.len() { 
            match self.buttons[button].tick() {
                Some(0) => {
                    self.state.statetype = StateType::Edit(Editor::new());
                    self.buttons = 
                        vec![
                            Button::menu_edit(), 
                            Button::brush_size(true), 
                            Button::brush_size(false), 
                            Button::brush_type(BlockType::Brick),
                            Button::brush_type(BlockType::Water),
                            Button::brush_type(BlockType::Lava),
                            Button::brush_type(BlockType::None),
                        ];
                    break
                }, // Menu to edit
                Some(1) => {
                    self.state.statetype = StateType::Play(Player::new(self.map.spawn.0 as f32*10.0, self.map.spawn.1 as f32*10.0));
                    self.buttons = vec![Button::edit_menu(), Button::play_menu()];
                    break
                }, // Menu to play
                Some(2) => {
                    self.state.statetype = StateType::Menu;
                    self.buttons = vec![Button::edit_menu(), Button::play_menu()];
                    break
                }, // Edit to menu
                Some(3) => {
                    self.state.statetype = StateType::Menu;
                    self.buttons = vec![Button::edit_menu(), Button::play_menu()];
                    break
                }, // Play to menu
                //Some() => {},
                Some(11) => {
                    on_button = true;
                }
                _ => {},
            };
        }
    }
}

enum ButtonFunction {

}

struct State {
    statetype: StateType,
    tickcount: u32,
}


#[derive(Clone, Copy)]
enum StateType {
    Menu,
    Edit(Editor),
    Play(Player),
}

struct Button {
    buttontype: ButtonType,
    pressed: bool, 
    tick: u32,
}

impl Button {
    fn tick(&mut self) -> Option<u8> {
        self.tick += 1;
        let mouse_position = mouse_position();
        let screen_dimensions = (screen_width(), screen_height());
        
        let (size, position, text) = 
        match self.buttontype {
            ButtonType::BrushSize(plus) => (   
                Vec2::new(30.0, 30.0),
                if plus {
                    Vec2::new(screen_dimensions.0, screen_dimensions.1)
                } else {
                    Vec2::new(screen_dimensions.0, screen_dimensions.1)
                },
                if plus {"+"} else {"-"}
            ),
            ButtonType::BrushType(blocktype) => ( 
                match blocktype {
                    BlockType::Brick => {
                        (Vec2::new(30.0, 30.0), Vec2::new(screen_dimensions.0/2.0 - 100.0 + 10.0, screen_dimensions.1 - 50.0), "B")
                    },

                    BlockType::Water => {
                        (Vec2::new(30.0, 30.0), Vec2::new(screen_dimensions.0/2.0 - 50.0 + 10.0, screen_dimensions.1 - 50.0), "W")
                    },

                    BlockType::Lava => {
                        (Vec2::new(30.0, 30.0), Vec2::new(screen_dimensions.0/2.0 + 10.0, screen_dimensions.1 - 50.0), "L")
                    },

                    BlockType::None => {
                        (Vec2::new(30.0, 30.0), Vec2::new(screen_dimensions.0/2.0 + 50.0 + 10.0, screen_dimensions.1 - 50.0), "A")
                    },
                }
            ),

            ButtonType::MenuToEdit => (Vec2::new(50.0, 30.0), Vec2::new(screen_dimensions.0/2.0 - 25.0, screen_dimensions.1/2.0 + 5.0), "Edit"),
            ButtonType::MenuToPlay => (Vec2::new(50.0, 30.0), Vec2::new(screen_dimensions.0/2.0 - 25.0, screen_dimensions.1/2.0 - 35.0), "Play"),
            ButtonType::PlayToMenu => (Vec2::new(50.0, 30.0), Vec2::new(screen_dimensions.0 - 60.0, 10.0), "Menu"),
            ButtonType::EditToMenu => (Vec2::new(50.0, 30.0), Vec2::new(screen_dimensions.0 - 60.0, 10.0), "Menu"),
            ButtonType::NewMap => (Vec2::new(50.0, 30.0), Vec2::new(screen_dimensions.0, screen_dimensions.1), "New Map"),
        };

        if mouse_position.0 > position.x && mouse_position.0 < position.x+size.x && mouse_position.1 > position.y && mouse_position.1 < position.y+size.y {
            if self.pressed {
                draw_rectangle_lines(position.x+2.0, position.y+2.0, size.x-4.0, size.y-4.0, 4.0, WHITE);
            } else {
                draw_rectangle_lines(position.x, position.y, size.x, size.y, 4.0, WHITE);
                if is_mouse_button_down(MouseButton::Left) {
                    self.pressed = true;
                    self.tick = 0;
                }
            }
        } else {
            draw_rectangle_lines(position.x, position.y, size.x, size.y, 2.0, WHITE);
        }

        let text_center = get_text_center(text, None, 20, 1.0, 0.0);
        draw_text(text, position.x+size.x/2.0-text_center.x, position.y+size.y/2.0-text_center.y, 20.0, WHITE);

        if self.tick == 10 && self.pressed {
            return Some (
                match self.buttontype {
                    ButtonType::MenuToEdit => 0,
                    ButtonType::MenuToPlay => 1,
                    ButtonType::EditToMenu => 2,
                    ButtonType::PlayToMenu => 3,
                    ButtonType::NewMap => 4,
                    ButtonType::BrushSize(true) => 5,
                    ButtonType::BrushSize(false) => 6,
                    ButtonType::BrushType(BlockType::Brick) => 7,
                    ButtonType::BrushType(BlockType::Water) => 8,
                    ButtonType::BrushType(BlockType::Lava) => 9,
                    ButtonType::BrushType(BlockType::None) => 10,
                }
            )
        }
        
        None
    }

    fn edit_menu() -> Self {
        Self {
            buttontype: ButtonType::MenuToEdit,
            pressed: false,
            tick: 0,
        }
    }

    fn play_menu() -> Self {
        Self {
            buttontype: ButtonType::MenuToPlay,
            pressed: false,
            tick: 0,
        }
    }

    fn menu_edit() -> Self {
        Self {
            buttontype: ButtonType::EditToMenu,
            pressed: false,
            tick: 0,
        }
    }

    fn menu_play() -> Self {
        Self {
            buttontype: ButtonType::PlayToMenu,
            pressed: false,
            tick: 0,
        }
    }

    fn brush_size(bool: bool) -> Self {
        Self {
            buttontype: ButtonType::BrushSize(bool),
            pressed: false,
            tick: 0,
        }
    }

    fn brush_type(blocktype: BlockType) -> Self {
        Self {
            buttontype: ButtonType::BrushType(blocktype),
            pressed: false,
            tick: 0,
        }
    } 
}


enum ButtonType {
    BrushSize(bool),
    BrushType(BlockType),
    MenuToEdit,
    MenuToPlay,
    PlayToMenu,
    EditToMenu,
    NewMap,
}

struct Text {

}

#[derive(Clone, Copy)]
struct Player {
    position: Vec2,
    velocity: Vec2,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(0.0, 0.0),
        }
    }

    fn tick(&mut self) {
        
    }
}

#[derive(Clone, Copy)]
struct Editor {
    scroll: f32,
    brushtype: BlockType,
}

impl Editor {
    fn new() -> Self {
        Self {
            scroll: 0.0,
            brushtype: BlockType::Brick,
        }
    }

    fn tick(&mut self, on_button: bool)  ->  Option<BlockPosition> {
        if is_mouse_button_down(MouseButton::Left) && !on_button {
            Some(get_mouse_block())
        } else {
            None
        }
    }
}

struct Map {
    blocks: [Block; MAP_SIZE.0*MAP_SIZE.1],
    spawn: BlockPosition,
}

impl Map {
    fn new() -> Self {
        Self {
            blocks: [Block::new(BlockType::None); MAP_SIZE.0*MAP_SIZE.1],
            spawn: (0, 0),
        }
    }

    fn draw(&self, scroll: f32) {
        for x in 0..MAP_SIZE.0 {
            for y in 0..MAP_SIZE.1 {
                let block = self.get_block(x, y);
                block.draw(x as f32*10.0+scroll, y as f32*10.0)
            }
        }
    }

    

    fn get_block(&self, x: usize, y: usize) -> Block {
        self.blocks[x+y*MAP_SIZE.0]
    }

    fn set_block(&mut self, x: u8, y: u8, block: Block) {
        self.blocks[x as usize+y as usize*MAP_SIZE.0] = block;
    }
}

fn get_mouse_block() -> BlockPosition {
    let mouse_position = mouse_position();
    ((mouse_position.0/10.0) as usize, (mouse_position.1/10.0) as usize)
}

#[derive(Clone, Copy)]
struct Block {
    blocktype: BlockType,
}

impl Block {
    fn new(blocktype: BlockType) -> Self {
        Self {
            blocktype,
        }
    }

    fn draw(self, x: f32, y: f32) {
        match self.blocktype {
            BlockType::None => {},
            _ => { draw_rectangle(x, y, 10.0, 10.0, match self.blocktype { BlockType::Brick => GRAY, BlockType::Lava => RED, BlockType::Water => BLUE, BlockType::None => GREEN}) }, 
        }
    }
}

#[derive(Clone, Copy)]
enum BlockType {
    Brick,
    Lava,
    Water,
    None,
}

fn window_conf() -> Conf {
    Conf {
        window_resizable: true,
        window_title: String::from("Quadrunster"),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut global = Global::init();

    loop {
        global.tick();
        next_frame().await
    }
}