use notan::prelude::*;
use notan::draw::*;
use rand::Rng;

const WINDOW_X: u32 = 600; //sets the width of the game window
const WINDOW_Y: u32 = 800; //sets the height of the game window
const WINDOW_X_FLOAT: f32 = 600.0; //sets the width of the game window
const WINDOW_Y_FLOAT: f32 = 800.0;
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
        for platform in state.platform_list.iter_mut() {
            match platform {
                PlatformResult::BasicPlatform(basic_platform) => {
                    basic_platform.y += 10.0;
                }
                PlatformResult::Blank(blank_platform) => {
                    blank_platform.y += 10.0;
                }
                PlatformResult::HorizontalMovingPlatform(horizontal_platform) => {
                    horizontal_platform.y += 10.0;
                }
            }
        }
    }

    for platform in state.platform_list.iter_mut() {
        match platform {
            PlatformResult::BasicPlatform(basic_platform) => {
                if basic_platform.y > WINDOW_Y_FLOAT {
                    basic_platform.y = 0.0;
                    *platform = spawn_platform(basic_platform.x, basic_platform.y, state.score);
                    state.score += 1;
                }
            }
            PlatformResult::Blank(blank_platform) => {
                if blank_platform.y > WINDOW_Y_FLOAT {
                    blank_platform.y = 0.0;
                    *platform = spawn_platform(blank_platform.x, blank_platform.y, state.score);
                }
            }
            PlatformResult::HorizontalMovingPlatform(horizontal_platform) => {
                if horizontal_platform.x <= 0.0 {
                    horizontal_platform.direction = true;
                } else if horizontal_platform.x >= WINDOW_X_FLOAT - PLATFORM_WIDTH {
                    horizontal_platform.direction = false;
                }
                horizontal_platform.shift(horizontal_platform.direction);

                if horizontal_platform.y > WINDOW_Y_FLOAT {
                    horizontal_platform.y = 0.0;
                    *platform = spawn_platform(horizontal_platform.x, horizontal_platform.y, state.score);
                }
            }
        }
    }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw: Draw = gfx.create_draw();
    draw.clear(Color::BLACK);

    if state.score < 1 {
        spawn_platforms(&mut state.platform_list);
        state.score += 1;
    }

    for platform in state.platform_list.iter() {
        match platform {
            PlatformResult::BasicPlatform(basic_platform) => {
                draw.rect(basic_platform.position(), (PLATFORM_WIDTH, PLATFORM_HEIGHT));
            }
            PlatformResult::HorizontalMovingPlatform(horizontal_platform) => {
                draw.rect(horizontal_platform.position(), (PLATFORM_WIDTH, PLATFORM_HEIGHT));
            }
            PlatformResult::Blank(_blank_platform) => {}
        }
    } 

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
                PlatformResult::Blank(BlankPlatform::new(0.0, 0.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 0.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 0.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 0.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 0.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 0.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 30.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 30.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 30.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 30.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 30.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 30.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 60.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 60.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 60.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 60.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 60.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 60.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 90.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 90.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 90.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 90.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 90.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 90.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 120.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 120.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 120.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 120.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 120.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 120.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 150.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 150.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 150.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 150.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 150.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 150.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 180.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 180.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 180.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 180.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 180.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 180.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 210.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 210.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 210.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 210.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 210.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 210.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 240.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 240.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 240.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 240.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 240.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 240.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 270.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 270.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 270.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 270.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 270.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 270.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 300.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 300.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 300.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 300.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 300.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 300.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 330.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 330.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 330.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 330.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 330.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 330.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 360.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 360.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 360.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 360.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 360.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 360.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 390.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 390.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 390.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 390.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 390.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 390.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 420.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 420.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 420.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 420.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 420.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 420.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 450.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 450.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 450.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 450.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 450.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 450.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 480.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 480.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 480.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 480.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 480.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 480.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 510.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 510.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 510.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 510.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 510.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 510.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 540.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 540.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 540.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 540.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 540.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 540.0)),
                PlatformResult::Blank(BlankPlatform::new(0.0, 570.0)),
                PlatformResult::Blank(BlankPlatform::new(100.0, 570.0)),
                PlatformResult::Blank(BlankPlatform::new(200.0, 570.0)),
                PlatformResult::Blank(BlankPlatform::new(300.0, 570.0)),
                PlatformResult::Blank(BlankPlatform::new(400.0, 570.0)),
                PlatformResult::Blank(BlankPlatform::new(500.0, 570.0)),
            ],
            score: 0,
        }
    }
}

trait Platform {
    fn new(x: f32, y: f32) -> Self;
    fn position(&self) -> (f32, f32);
}

#[derive(Debug)]
enum PlatformResult {
    BasicPlatform(BasicPlatform),
    HorizontalMovingPlatform(HorizontalMovingPlatform),
    Blank(BlankPlatform),
}

#[derive(Debug)]
struct BlankPlatform {
    x: f32,
    y: f32,
}

impl Platform for BlankPlatform {
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

#[derive(Debug)]
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

#[derive(Debug)]
struct HorizontalMovingPlatform {
    x: f32,
    y: f32,
    direction: bool,
    delta: f32,
}

impl Platform for HorizontalMovingPlatform {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            direction: true,
            delta: generate_move_delta(),
        }
    }
    fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl HorizontalMovingPlatform {
    fn shift(&mut self, direction: bool) {
        if direction {
            self.x += self.delta;
        } else {
            self.x -= self.delta;
        }
    }
}

// struct verticalMovingPlatform {

// }

fn spawn_platforms(platforms: &mut Vec<PlatformResult>) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    for i in 0..6 {
        for t in 0..20 {
            if rng.gen_range(0..=4) == 1 {
                platforms[(i*20)+t] = PlatformResult::BasicPlatform(BasicPlatform::new(i as f32 * 100.0, t as f32 * 30.0));
            } else {
                platforms[(i*20)+t] = PlatformResult::Blank(BlankPlatform::new(i as f32 * 100.0, t as f32 * 30.0));
            }   
        }
    }
}

fn spawn_platform(i: f32, t: f32, score: i32) -> PlatformResult {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    // very rudimentary formula for when score gets larger to spawn in less platforms 
    // score is increasing when a platform goes by
    let random: i32 = rng.gen_range(0..=(score));
    if random == 1 || random == 2 {
        return PlatformResult::BasicPlatform(BasicPlatform::new(i, t));
    } else if random == 3 {
        let platform: PlatformResult = PlatformResult::HorizontalMovingPlatform(HorizontalMovingPlatform::new(i, t)); 
        return platform;
    } else {
        return PlatformResult::Blank(BlankPlatform::new(i, t));
    }   
}

fn generate_move_delta() -> f32 {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let delta: f32 = rng.gen_range(1.0..=7.0);
    delta
}

// fn check_proximity(platform_index: i32, platform_list: Vec<PlatformResult>) -> bool {

//     true
// }

// maybe make everything a 2d array of platform results and grab the first x value to dictate the rest of them