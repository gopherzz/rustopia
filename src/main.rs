use cgmath;
use ggez::{Context, ContextBuilder, GameError, GameResult, conf, event, graphics::{self, Canvas, Color, DrawParam, FillOptions}, timer};

const WINDOW_WIDTH: f32 = 1024.;
const WINDOW_HEIGHT: f32 = 768.;
const BLOCK_WIDTH: f32 = 80.;
const BLOCK_HEIGHT: f32 = 100.;
const WORLD_WIDTH: usize = 60;
const WORLD_HEIGHT: usize = 30;

#[derive(Copy, Clone)]
enum Plan {
    Foreground,
    Background,
}

#[derive(Copy, Clone)]
enum BlockType {
    Air,
    Dirt,
}

#[derive(Copy, Clone)]
struct Block {
    id: BlockType,
    is_solid: bool,
    z: Plan,
}

struct World {
    name: String,
    map: [[Block; WORLD_WIDTH]; WORLD_HEIGHT],
    blend_mode: Option<graphics::BlendMode>,
}
struct State {
    dt: std::time::Duration,
    fps: ggez::graphics::Text,
    origin: cgmath::Point2<f32>,
    world: World,
    keysdown: Vec<event::KeyCode>,
    zoom: f32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Block {
    fn draw(&self, ctx: &mut Context, x: f32, y: f32, zoom: f32) {
        let block_rect = graphics::Rect {
            x: x, 
            y: y, 
            w: BLOCK_WIDTH, 
            h: BLOCK_HEIGHT,
        };
        let block_mesh: graphics::Mesh;

        match self.id {
            BlockType::Air => block_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(1.), block_rect, Color::BLUE).unwrap(),
            BlockType::Dirt => block_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(1.), block_rect, Color::WHITE).unwrap(),
        }

        match graphics::draw(ctx, &block_mesh, DrawParam::new().scale(cgmath::Vector2::new(zoom, zoom))) {
            Ok(_) => (),
            Err(_) => println!("Error While draw block!"),
        }
    }
}

impl World {
    fn new(name: String) -> World {
        let air_block = Block{
            id: BlockType::Air,
            is_solid: false,
            z: Plan::Foreground,
        };
        let dirt_block = Block {
            id: BlockType::Dirt,
            is_solid: true,
            z: Plan::Background,
        };
        let mut map: [[Block; WORLD_WIDTH]; WORLD_HEIGHT] = [[air_block; WORLD_WIDTH]; WORLD_HEIGHT];
        for i in 0..20 {
            map[i] = [air_block; WORLD_WIDTH];
        }
        for i in 20..map.len() {
            map[i] = [dirt_block; WORLD_WIDTH];
        }
        World {
            name: name,
            map: map,
            blend_mode: Some(graphics::BlendMode::Lighten),
        }
    }
    fn draw(&self, ctx: &mut Context, offset: &cgmath::Point2<f32>, zoom: f32) {
        for (row_idx, _row) in self.map.to_vec().iter().enumerate() {
            for (col_idx, block) in self.map[row_idx].iter().enumerate() {
                let x = col_idx as f32 * BLOCK_WIDTH - offset.x;
                let y = row_idx as f32 * BLOCK_HEIGHT - offset.y;
                if x < -BLOCK_WIDTH || x > WINDOW_WIDTH as f32
                || y < -BLOCK_HEIGHT || y > WINDOW_HEIGHT as f32 {
                    continue;
                }
                block.draw(ctx, x, y, zoom);
            }
        }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: event::MouseButton, _x: f32, _y: f32) {
        println!("Mouse X: {}, Mouse Y {}", _x, _y);
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: event::KeyCode, _keymods: event::KeyMods, _repeat: bool) {
        self.keysdown.push(keycode);
        self.keysdown.dedup_by_key(|x| *x);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: event::KeyCode, _keymods: event::KeyMods) {
        self.keysdown.retain(|&x| x != keycode);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        if y > 0.0 {
            self.zoom = self.zoom - 0.1;
        } else if y < 0.0 {
            self.zoom = self.zoom + 0.1;
        }
        println!("{}", self.zoom);
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const FPS: u32 = 60;

        while timer::check_update_time(ctx, FPS) {
            for keycode in &self.keysdown {
                if keycode == &event::KeyCode::W {
                    self.origin.y = self.origin.y - 2.0;
                }
                if keycode == &event::KeyCode::S {
                    self.origin.y = self.origin.y + 2.0;
                }
                if keycode == &event::KeyCode::A {
                    self.origin.x = self.origin.x - 2.0;
                }
                if keycode == &event::KeyCode::D {
                    self.origin.x = self.origin.x + 2.0;
                }

                if self.origin.x < 0.0 {
                    self.origin.x = 0.0;
                } else if self.origin.x > WORLD_WIDTH as f32 * BLOCK_WIDTH {
                    self.origin.x = WORLD_WIDTH as f32 * BLOCK_WIDTH
                }
                if self.origin.y < 0.0 {
                    self.origin.y = 0.0;
                } else if self.origin.y > WORLD_HEIGHT as f32 * BLOCK_HEIGHT {
                    self.origin.y = WORLD_HEIGHT as f32 * BLOCK_HEIGHT;
                }
            }
        }
        self.fps = graphics::Text::new(format!("FPS: {:.1}, X: {}, Y: {}", timer::fps(ctx), self.origin.x, self.origin.y));
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // let (width, height) = graphics::drawable_size(ctx);

        graphics::queue_text(ctx, &self.fps, cgmath::Point2::new(0.0, 0.0), None);

        graphics::draw_queued_text(
            ctx, 
            graphics::DrawParam::default(),
            None,
            graphics::FilterMode::Linear
        )?;
        self.world.draw(ctx, &self.origin, self.zoom);
        
        let player_image = graphics::Image::new(ctx, "/player.png").expect("Image Not Found!");
        graphics::draw(ctx, &player_image, DrawParam::new().dest(cgmath::Point2::new(WINDOW_WIDTH / 2. - player_image.width() as f32 / 2., WINDOW_HEIGHT / 2. - player_image.height() as f32 / 2.)))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let state = State {
        dt: std::time::Duration::new(0, 0),
        fps: graphics::Text::new(""),
        world: World::new("Some World".to_string()),
        origin: cgmath::Point2::new(BLOCK_WIDTH * 20., BLOCK_WIDTH * 20.),
        keysdown: Vec::new(),
        zoom: 1.0,
    };
    
    let window_setup = conf::WindowSetup {
        title: "Rustopia".to_string(),
        samples: conf::NumSamples::One,
        vsync: false,
        icon: "".to_owned(),
        srgb: true,
    };

    let mut window_mode = ggez::conf::WindowMode::default();
    window_mode.width = WINDOW_WIDTH;
    window_mode.height = WINDOW_HEIGHT;

    let c = conf::Conf {
        window_mode: window_mode,
        window_setup: window_setup,
        backend: conf::Backend::default(),
        modules: conf::ModuleConf::default()
    };
    let (ctx, event_loop) = ContextBuilder::new("rustopia", "gopherz")
        .default_conf(c)
        .build()
        .unwrap();
    
    event::run(ctx, event_loop, state);
}