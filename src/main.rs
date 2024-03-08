use notan::prelude::*;
use notan::draw::*;
use rand::Rng;

const WINDOW_X: u32 = 600; //sets the width of the game window
const WINDOW_Y: u32 = 800; //sets the height of the game window
const PLATFORM_WIDTH: f32 = 100.0;
const PLATFORM_HEIGHT: f32 = 30.0;
// const PLATFORM_SPEED: f32 = 20.0;
// const MAX_SPEED: f32 = 350.0; // the max speed the player can go
// const ACCELERATION_RATE: f32 = 700.0; // how fast the player accelerates
// const GRAVITY: f32 = 400.0; // the speed at which the player falls
// const MAX_FALL: f32 = 600.0; // the max rate the player can fall
// const TEMP_GROUND: f32 = 600.0; // dummy ground for testing bouncing
// const CROSS_ACCEL: f32 = 2.0; // acceleration boost for going in opposite direction
// const STOP_ACCEL: f32 = 3.0; // acceleration boost for coming to a stop
// const PLAYER_WIDTH: f32 = 80.0; // width of player sprite
// const PLAYER_HEIGHT: f32 = 80.0; //height of player sprite
// const BOUNCE_HEIGHT: f32 = -600.0; //player jump height, its negative because y zero is at top of screen

fn main() ->Result<(), String> {
    let win_config: WindowConfig = WindowConfig::new().set_size(WINDOW_X, WINDOW_Y).set_vsync(true);

    notan::init_with(State::new).add_config(win_config).add_config(DrawConfig).update(update).draw(draw).build()
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.is_down(KeyCode::W) {
        // state.platform_1.y = state.platform_1.y - PLATFORM_SPEED;
    } else if app.keyboard.is_down(KeyCode::S) {
        // state.platform_1.y = state.platform_1.y + PLATFORM_SPEED;
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw: Draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    spawn_platforms(&mut state.platform_list, state.score as usize);

    state.score += 1;

    for platform in state.platform_list.iter() {
        match platform {
            PlatformResult::BasicPlatform(platform) => {
                draw.rect(platform.position(), (PLATFORM_WIDTH, PLATFORM_HEIGHT));
            }
            PlatformResult::Blank => {}
        }
    } 

    // draw.rect(state.jumpy_boi.position(), (PLAYER_WIDTH, PLAYER_HEIGHT));
    gfx.render(&draw);
    
}

#[derive(AppState)]
struct State {
    platform_list: Vec<PlatformResult>,
    score: i32,
}

impl State {
    fn new(_gfx: &mut Graphics) -> Self {
        Self {
            platform_list: vec![
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
                PlatformResult::Blank,
            ],
            score: 0,
        }
    }
}

trait Platform {
    fn new(x: f32, y: f32) -> Self;
    fn position(&self) -> (f32, f32);
}

enum PlatformResult {
    BasicPlatform(BasicPlatform),
    Blank,
}

struct BasicPlatform {
    x: f32,
    y: f32,
}

impl Platform for BasicPlatform {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

fn spawn_platforms(platforms: &mut Vec<PlatformResult>, score: usize) {
    let mut bit: i8 = 0;
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    if score == 0 as usize {
        for i in 0..6 {
            for t in 0..20 {
                bit = rng.gen_range(0..=4);
                if bit == 1 {
                    platforms[(i*20)+t] = PlatformResult::BasicPlatform(BasicPlatform::new(i as f32 * 100.0, t as f32 * 30.0));
                } else {
                    platforms[(i*20)+t] = PlatformResult::Blank;
                }
            }
        }
    }
}

// struct JumpyBoi {
//     x: f32,
//     y: f32,
// }

// impl JumpyBoi {
//     fn new(x: f32, y: f32) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
//     fn position(&self) -> (f32, f32) {
//         (self.x, self.y)
//     }
// }