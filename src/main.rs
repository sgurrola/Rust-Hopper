use notan::prelude::*;
use notan::draw::*;
use rand::Rng;
use std::iter::Scan;
use std::time::{Duration, Instant};

mod projectiles;
use projectiles::*;

mod enemies;
use enemies::*;

mod platforms;
use platforms::*;

mod platform_algos;
use platform_algos::*;

mod collisions;
use collisions::*;

use crate::enemies::spawn_enemy;
mod poop;
use poop::*;

//the state holds all our game data / stats / anything we need, passed to both the render and gameplay logic function
#[derive(AppState)]
struct State {
    img: Texture,
    x: f32, //actual x position of player on screen
    y: f32, //actual y position of player on screen
    x_vel: f32, //players velocity in x direction
    y_vel: f32, //players velocity in y direction
    offset: f32, //test for fake camera
    score: i32,
    anims: Vec<Anims>,
    anim: usize,
    shoot:bool,
    facing:f32,
    platform_list: Vec<PlatformResult>,
    proj_text: Texture,
    projectiles: Vec<Projectile>,
    last_shot_time: Instant, // Track the time of the last shot
    fire_delay: Duration, // Define the firing delay duration
    python_text: Texture,
    haskell_text: Texture,
    section_start: i32,
    java_text: Texture,
    enemies: Vec<Enemies>,
    poop: Vec<Poop>,
    poop_text: Texture,


    
    //anim: Option<Box<dyn AnimState>>

}

enum Anims{
    Idle(Animation, usize),
    Falling(Animation, usize),
    Shooting(Animation, usize),
}
/*
trait AnimState {
    fn update(wow: &Self, num: f32, state: &mut State) where Self:Sized{}
    fn shoot( wow: &Self, state: &mut State) where Self:Sized{
        return state.shooting;
    }
    fn image(wow: &Self) -> Texture where Self:Sized{}
}
*/
struct Animation
{
    anims: Vec<Texture>,
    timing: f32,
    frame: i32,
    speed: f32,
}


/*
impl AnimState for Idle {
    fn image(wow: &Self) -> Texture{
        return wow.anims[wow.frame];
    }
    fn update(wow: &Self, num:f32, state: &mut State){
        wow.timing += num;
        if wow.timing > wow.speed {
            wow.timing = 0.0;
            wow.frame = wow.frame + 1 % wow.anims.len();
        }
        if state.y_vel > 0.0{
            return state.falling;
        }
        else {return state.rising;}
    }
}

impl AnimState for Falling {
    fn image(wow: &Self) -> Texture{
        return wow.anims[wow.frame];
    }
    fn update(wow: &Self, num:f32, state: &mut State) -> Box<dyn State>{
        wow.timing += num;
        if wow.timing > wow.speed {
            wow.timing = 0.0;
            wow.frame = wow.frame + 1 % wow.anims.len();
        }

        if state.y_vel < 0.0{
            return state.rising;
        }
        else {return state.falling;}
    }
}

impl AnimState for Shooting {
    fn image(wow: &Self) -> Texture{
        return wow.anims[wow.frame];
    }
    fn update(wow: &Self, num:f32, state: &mut State){
        wow.timing += num;
        if wow.timing > wow.speed {
            wow.timing = 0.0;
            wow.frame = wow.frame + 1;
            if wow.frame >= wow.anims.len() as i32{
                return state.rising;
            }
        }
        else {return state.shooting;}
    }
}
*/
const MAX_SPEED: f32 = 350.0; // the max speed the player can go
const ACCELERATION_RATE: f32 = 700.0; // how fast the player accelerates
const GRAVITY: f32 = 400.0; // the speed at which the player falls
const MAX_FALL: f32 = 600.0; // the max rate the player can fall
const TEMP_GROUND: f32 = 600.0; // dummy ground for testing bouncing
const CROSS_ACCEL: f32 = 2.0; // acceleration boost for going in opposite direction
const STOP_ACCEL: f32 = 3.0; // acceleration boost for coming to a stop
const WINDOW_X: u32 = 600; //sets the width of the game window
const WINDOW_Y: u32 = 800; //sets the height of the game window
const WINDOW_X_FLOAT: f32 = 600.0; //sets the width of the game window
const _WINDOW_Y_FLOAT: f32 = 800.0;
const PLATFORM_WIDTH: f32 = 100.0;
const PLATFORM_HEIGHT: f32 = 30.0;
const PLAYER_WIDTH: f32 = 70.0; // width of player sprite
const PLAYER_HEIGHT: f32 = 120.0; //height of player sprite
const BOUNCE_HEIGHT: f32 = -600.0; //player jump height, its negative because y zero is at top of screen

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

    notan::init_with(init).add_config(win_config).add_config(DrawConfig).update(update).draw(draw).build()
}

//this just initializes the loops, used in main
fn init(gfx: &mut Graphics) -> State {

    let texture = gfx
        .create_texture()
        .from_image(include_bytes!("assets/andrewzoom.png"))
        .build()
        .unwrap();

    let idle1 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/spin1.PNG"))
    .build()
    .unwrap();

    let idle2 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/spin2.PNG"))
    .build()
    .unwrap();

    let idle3 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/spin3.PNG"))
    .build()
    .unwrap();

    let idle4 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/spin4.PNG"))
    .build()
    .unwrap();

    let fall4 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/groove1.PNG"))
    .build()
    .unwrap();

    let fall5 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/groove2.PNG"))
    .build()
    .unwrap();

    let fall6 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/groove3.PNG"))
    .build()
    .unwrap();

    let shoot1 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/juggle1.PNG"))
    .build()
    .unwrap();

    let shoot2 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/juggle2.PNG"))
    .build()
    .unwrap();

    let shoot3 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/juggle3.PNG"))
    .build()
    .unwrap();

    let shoot4 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/juggle4.PNG"))
    .build()
    .unwrap();

    let proj_text = gfx
        .create_texture()
        .from_image(include_bytes!("assets/scala_icon.png"))
        .build()
        .unwrap();

    let python_text = gfx
        .create_texture()
        .from_image(include_bytes!("assets/python_icon.png"))
        .build()
        .unwrap();
    let haskell_text = gfx
        .create_texture()
        .from_image(include_bytes!("assets/haskell_icon.png"))
        .build()
        .unwrap();

    let java_text = gfx
        .create_texture()
        .from_image(include_bytes!("assets/java_icon.png"))
        .build()
        .unwrap();
    let poop_text = gfx
        .create_texture()
        .from_image(include_bytes!("assets/null_icon.png"))
        .build()
        .unwrap();

    let temp = Anims::Idle(Animation{anims:vec![idle1, idle2, idle3,idle4], timing:0.0,frame:0, speed:0.12},0);
    let temp1 = Anims::Falling(Animation{anims:vec![fall4, fall5, fall6], timing:0.0, frame:0, speed:0.12}, 1);
    let temp2 = Anims::Shooting(Animation{anims:vec![shoot1, shoot2, shoot3, shoot4], timing:0.0, frame:0, speed:0.12}, 2);
    let section_start = determine_section();
    State {
        img: texture,
        x: 100.0,
        y: 100.0,
        x_vel: 0.0,
        y_vel:0.0,
        offset:0.0,
        score:0,
        anim:0,
        anims: vec![temp, temp1, temp2],
        shoot: false,
        facing: 1.0,
        projectiles: vec![],
        proj_text,
        last_shot_time: Instant::now(), // Initialize last shot time to the current time
        fire_delay: Duration::from_millis(200), // Set the firing delay
        enemies: vec![],
        python_text,
        haskell_text,
        java_text,
        poop: vec![],
        section_start,
        poop_text,
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
} 
}

//this is the logic that runs each frame
fn update(app: &mut App, state: &mut State) {
    //for moving left
    if app.keyboard.is_down(KeyCode::A) {
        //if state.facing > 0.0 {
            //state.x += PLAYER_WIDTH;
        //}
        state.facing = -1.0;
        //checks if player is moving with or against key, and adds to the velocity acordingly
        if state.x_vel < 1.0 {
        state.x_vel -= ACCELERATION_RATE * app.timer.delta_f32();
        }
        else
        {
            state.x_vel -= ACCELERATION_RATE * app.timer.delta_f32() * CROSS_ACCEL;
        }

    }
    //for moving right
    if app.keyboard.is_down(KeyCode::D) {
        //if state.facing < 0.0 {
            //state.x -= PLAYER_WIDTH;
            
        //}
        state.facing = 1.0;
        //checks if player is moving with or against key, and adds to the velocity acordingly
        if state.x_vel > 1.0 {
            state.x_vel += ACCELERATION_RATE * app.timer.delta_f32();
            }
            else
            {
                state.x_vel += ACCELERATION_RATE * app.timer.delta_f32() * CROSS_ACCEL;
            }
    }
    //checks for no left / right input, not an if/else so holding left and right doesn't bias to a direction
    if !app.keyboard.is_down(KeyCode::D) && !app.keyboard.is_down(KeyCode::A){
        if state.x_vel.abs() < 4.0{
            state.x_vel = 0.0;
        }
        else if state.x_vel > 1.0 {
            state.x_vel -= ACCELERATION_RATE * app.timer.delta_f32() * STOP_ACCEL;
        }
        else{
            state.x_vel += ACCELERATION_RATE * app.timer.delta_f32() * STOP_ACCEL;
        }
    }

    //this caps the max speed of the player
    if state.x_vel.abs() > MAX_SPEED{
        if state.x_vel < 1.0{
            state.x_vel = MAX_SPEED * -1.0;
        }
        else{
            state.x_vel = MAX_SPEED;
        }
    }
    //this adds the velocity to the players current position
    state.x += state.x_vel * app.timer.delta_f32();

    //adds gravity to y velocity
    state.y_vel += GRAVITY * app.timer.delta_f32();

    //caps the max fall speed of player
    if state.y_vel > MAX_FALL {
        state.y_vel = MAX_FALL;
    }
    //checks if the players position + the velocity that will be added that frame would be lower than the ground, and if so jump
    /*if state.y + (state.y_vel * app.timer.delta_f32()) > TEMP_GROUND {
        println!("y {} and vel {} before bounce", state.y, state.y_vel);
        state.y = TEMP_GROUND;
        state.y_vel = BOUNCE_HEIGHT;
        println!("bounce here {}", state.y_vel);
    }*/
    for index in 0..120 {
        let platform: PlatformResult = state.platform_list[index as usize];
        if is_in_section(state.score, state.section_start) && check_proximity(&index, &state.platform_list) < 3 {
            match platform {
                PlatformResult::BasicPlatform(mut basic_platform) => {
                    if basic_platform.y > WINDOW_Y_FLOAT {
                        basic_platform.y = 0.0;
                        state.platform_list[index as usize] = generate_special_platform(basic_platform.x, basic_platform.y);
                        println!("{:?}", &state.platform_list[index as usize]);
                        state.score += 1;
                    }
                }
                PlatformResult::Blank(mut blank_platform) => {
                    if blank_platform.y > WINDOW_Y_FLOAT {
                        blank_platform.y = 0.0;
                        state.platform_list[index as usize] = generate_special_platform(blank_platform.x, blank_platform.y);
                    }
                }
                PlatformResult::HorizontalMovingPlatform(mut horizontal_platform) => {
                    if horizontal_platform.x <= 0.0 {
                        horizontal_platform.direction = true;
                    } else if horizontal_platform.x >= WINDOW_X_FLOAT - PLATFORM_WIDTH {
                        horizontal_platform.direction = false;
                    }
                    horizontal_platform.shift(horizontal_platform.direction);
    
                    if horizontal_platform.y > WINDOW_Y_FLOAT {
                        horizontal_platform.y = 0.0;
                    }
                }
                PlatformResult::VerticalMovingPlatform(vertical_platform) => {
                    
                }
            }   
        } else {
            match &mut state.platform_list[index as usize] {
                PlatformResult::BasicPlatform(basic_platform) => {
                    if basic_platform.y > WINDOW_Y_FLOAT {
                        basic_platform.y = 0.0;
                        state.platform_list[index as usize] = spawn_platform(basic_platform.x, basic_platform.y, state.score);
                        state.score += 1;
                    }
                }
                PlatformResult::Blank(blank_platform) => {
                    if blank_platform.y > WINDOW_Y_FLOAT {
                        blank_platform.y = 0.0;
                        state.platform_list[index as usize] = spawn_platform(blank_platform.x, blank_platform.y, state.score);
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
                        state.platform_list[index as usize] = spawn_platform(horizontal_platform.x, horizontal_platform.y, state.score);
                    }
                }
                PlatformResult::VerticalMovingPlatform(vertical_platform) => {
                    
                }
            }   
        }
    }

    let mut poops: Vec<(f32,f32)> = Vec::new();
    for enemy in state.enemies.iter_mut(){
        match enemy {
            Enemies::StaticEnemy(_pe) => {
            }
            Enemies::MovingEnemy(me) => {
                if me.x <= 0.0 {
                    me.direction = true;
                } else if me.x >= WINDOW_X_FLOAT - PLATFORM_WIDTH {
                    me.direction = false;
                }
                me.shift(me.direction);
            }
            Enemies::PoopyEnemy(po) => {
                //println!("poopy");
                poops.push((po.x,po.y));
            }
        }
    }

    //poop::shoot_poopies(state, 500.0, 500.0);
    for (x,y) in poops{
        //println!("poop loop");
        poop::shoot_poopies(state, x, y);
    }
    //my issue is tying this specifically to Poopy Enemy, otherwise I think it is fine

    poop::update_poopies(state, app.timer.delta_f32());

    if app.keyboard.is_down(KeyCode::Space) {
        projectiles::shoot_projectile(state);
    }

    projectiles::update_projectiles(state, app.timer.delta_f32());
    //shoot enemies pls currently does nothing 
    /*
    for enemy in state.enemies.iter_mut(){
        for projectile in state.projectiles.iter() {
            if projectile_enemy_collision(projectile.x, projectile.y, &enemy) {

            }
        }
    } */

    if state.y_vel >0.0 {
        let mut thing: f32 = 0.0;
        //if(state.facing > 0.0){
            //thing = PLAYER_WIDTH * -1.0;
        //}
        for platform in state.platform_list.iter() {
            if player_plat_collision(state.x + thing, state.y, platform){
                state.y_vel = BOUNCE_HEIGHT;
            }
            
        }

        let mut reset_check = false;
        for projectile in state.projectiles.iter_mut() {
            for enemy in state.enemies.iter_mut() {
                if player_enemy_collision(state.x + thing, state.y, enemy){
                    state.score = 0;
                    state.x = 300.0;
                    state.y = 300.0;
                    state.y_vel = 0.0;
                    state.x_vel = 0.0;
                    reset_check = true;
                    state.poop = vec![];
                } else if projectile_enemy_collision(projectile.x, projectile.y, enemy){
                        match enemy {
                            Enemies::StaticEnemy(ref mut pe) => {
                                pe.y = 4000.0;
                            }
                            Enemies::MovingEnemy(ref mut me) => {
                                me.y = 4000.0;
                            }
                            Enemies::PoopyEnemy(ref mut po) => {
                                po.y = 4000.0;
                            }
                        }
                }
            }
        }
        if reset_check == true{
            state.enemies = vec![];
        }
        

    }
    
    //this is the screen wrap code from left to right
    if state.x + (PLAYER_WIDTH / 2.0) < 0.0{
        state.x = (WINDOW_X as f32) + (state.x);
        println!("wrap left {}", state.x);
    }
    //the screen wrap code from right to left
    else if state.x + (PLAYER_WIDTH / 2.0) > (WINDOW_X as f32){
        state.x = state.x - (WINDOW_X as f32);
        println!("wrap right {}", state.x);
    }

    //didn't realize I had this code here twice, will delete later but physics are currently tuned with this
    state.y += state.y_vel * app.timer.delta_f32();

    if state.y < 300.0 {
        let dist = 290.0 *app.timer.delta_f32();
        state.y += dist;
        for platform in state.platform_list.iter_mut() {
            match platform {
                PlatformResult::BasicPlatform(ref mut platform) => {
                    platform.y += dist;
                }
                PlatformResult::Blank(ref mut platform) => {
                    platform.y += dist;
                }
                _ => {}
            }
        } 
    }

    //This moves the platforms up if the player is moving up and is in the top 2/3rds of the screen
    if state.y < 500.0 && state.y_vel < 0.0 {
        for platform in state.platform_list.iter_mut() {
            match platform {
                PlatformResult::BasicPlatform(ref mut platform) => {
                    platform.y -= state.y_vel * app.timer.delta_f32();
                }
                PlatformResult::Blank(ref mut platform) => {
                    platform.y -= state.y_vel * app.timer.delta_f32();
                }
                _ => {}
            }
        } 
        for enemy in state.enemies.iter_mut(){
            match enemy {
                Enemies::StaticEnemy(ref mut pe) => {
                    pe.y -= state.y_vel * app.timer.delta_f32();
                }
                Enemies::MovingEnemy(ref mut me) => {
                    me.y -= state.y_vel * app.timer.delta_f32();
                }
                Enemies::PoopyEnemy(ref mut po) => {
                    po.y -= state.y_vel * app.timer.delta_f32();
                }
            }
        }

        for poopy in state.poop.iter_mut() {
            poopy.y -= state.y_vel * app.timer.delta_f32();
        }
        state.offset -= state.y_vel * app.timer.delta_f32();

    }

    if state.y > WINDOW_Y as f32 + 20.0{
        state.score = 0;
        state.x = 300.0;
        state.y = 300.0;
        state.y_vel = 0.0;
        state.x_vel = 0.0;
        state.enemies = vec![];
        state.poop = vec![];
    }
    

     match state.anims[state.anim]{
        Anims::Idle(ref mut anime, i) => {
            anime.timing += app.timer.delta_f32();
            if anime.timing > anime.speed {
                anime.frame = (anime.frame + 1) % (anime.anims.len() as i32);
                anime.timing = 0.0;
            }
            if state.y_vel > 0.0{
                state.anim = 1;
                anime.frame = 0;
                anime.timing = 0.0;
            }
            if state.shoot {
                state.anim = 2;
            }
        }
        Anims::Falling(ref mut anime, i) =>  {
            anime.timing += app.timer.delta_f32();
            if anime.timing > anime.speed {
                anime.frame = (anime.frame + 1) % (anime.anims.len() as i32);
                anime.timing = 0.0;
            }
            if state.y_vel < 0.0{
                state.anim = 0;
            }
            if state.shoot {
                state.anim = 2;
            }
        }
        Anims::Shooting(ref mut anime, i) => {
            anime.timing += app.timer.delta_f32();
            if anime.timing > anime.speed {
                anime.frame =anime.frame + 1;
                anime.timing = 0.0;
            }
            if anime.frame >= anime.anims.len() as i32{
                state.shoot = false;
                state.anim = 0;
                anime.frame = 0;
                anime.timing = 0.0;
            }
        }
     }

     if app.keyboard.is_down(KeyCode::Space) {
        
        projectiles::shoot_projectile(state);
    }

    projectiles::update_projectiles(state, app.timer.delta_f32());

    //println!("x_vel: {}, score: {}", state.x_vel, state.score); // Debugging

    if (state.score % 5 == 0) && state.score > 125{
        //println!("enemy should be spawned");
        state.score = state.score + 1;
        enemies::spawn_enemy(state);
    } else if (state.score % 6 == 0) && state.score > 100{
        //println!("enemy should be spawned");
        state.score = state.score + 1;
        enemies::spawn_enemy(state);
    } else if (state.score % 7 == 0) && state.score > 75{
        //println!("enemy should be spawned");
        state.score = state.score + 1;
        enemies::spawn_enemy(state);
    } else if (state.score % 8 == 0) && state.score > 50{
        //println!("enemy should be spawned");
        state.score = state.score + 1;
        enemies::spawn_enemy(state);
    } else if (state.score % 9 == 0) && state.score > 25{
        //println!("enemy should be spawned");
        state.score = state.score + 1;
        enemies::spawn_enemy(state);
    } else if (state.score % 10 == 0) && state.score > 0{
        //println!("enemy should be spawned");
        state.score = state.score + 1;
        enemies::spawn_enemy(state);
    }
    
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw: Draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    let thing;
    match &state.anims[state.anim]{
        Anims::Idle(anime, _) | Anims::Falling(anime, _) | Anims::Shooting(anime, _) => thing = &anime.anims[anime.frame as usize],
        _ => thing = &state.img,
    }
    draw.rect((state.x, state.y), (PLATFORM_WIDTH, PLATFORM_HEIGHT));
    draw.image(thing).size(/*state.facing * */PLAYER_WIDTH,PLAYER_HEIGHT).position(state.x, state.y);
    draw.image(&state.img).size(40.0,120.0).position(400.0, 200.0 + state.offset);
    draw.image(&state.img).size(40.0,120.0).position(300.0, 100.0 + state.offset);

    for projectile in &state.projectiles {
        draw.image(&projectile.proj_text)
            .size(20.0, 20.0)
            .position(projectile.x, projectile.y);
    }
    for enemy in &state.enemies {
        match enemy {
            Enemies::StaticEnemy(pe) => {
                draw.image(&pe.enemy_text)
                .size(30.0, 30.0)
                .position(pe.x, pe.y);
            }
            Enemies::MovingEnemy(me) => {
                draw.image(&me.enemy_text)
                .size(30.0, 30.0)
                .position(me.x, me.y);
            }
            Enemies::PoopyEnemy(po) => {
                draw.image(&po.enemy_text)
                .size(30.0, 30.0)
                .position(po.x, po.y);
            }
        }
    }

    for poopies in &state.poop {
        draw.image(&poopies.poop_text)
            .size(20.0, 20.0)
            .position(poopies.x, poopies.y);
            //.position(x_poop, y_poop);
    }

    //draw.image(&state.proj_text).size(20.0,20.0).position(state.x, state.y);

    if state.score == 0 
    {
        spawn_platforms(&mut state.platform_list);
        state.score = 1;
    }
    for platform in state.platform_list.iter() {
        match platform {
            PlatformResult::BasicPlatform(basic_platform) => {
                draw.rect(basic_platform.position(), (PLATFORM_WIDTH, PLATFORM_HEIGHT));
            }
            PlatformResult::HorizontalMovingPlatform(horizontal_platform) => {
                draw.rect(horizontal_platform.position(), (PLATFORM_WIDTH, PLATFORM_HEIGHT));
            }
            PlatformResult::VerticalMovingPlatform(vertical_platform) => {
                draw.rect(vertical_platform.position(), (PLATFORM_WIDTH, PLATFORM_HEIGHT));
            }
            PlatformResult::Blank(_blank_platform) => {}
        }
    }  

    // draw.rect(state.jumpy_boi.position(), (PLAYER_WIDTH, PLAYER_HEIGHT));
    gfx.render(&draw);
    
}

// fn default_collision( x1 :f32, y1 :f32, w1 :f32, h1 :f32, x2 :f32, y2 :f32, w2 :f32,  h2 :f32) -> bool {
//     if ((x1 + w1) > x2 && x1 < x2) || ((x2 + w2) > x1 && x2 < x1)
//     {
//         if ((y1 + h1) > y2 && y1 < y2) || ((y2 + h2) > y1 && y2 < y1){
//             return true;
//         }
//     }
//     return false;
// }

// fn player_plat_collision( px :f32, py :f32,  platEnum : &PlatformResult) -> bool{
//     match platEnum {
//         PlatformResult::BasicPlatform(plat) => {
//             return default_collision(px,py, PLAYER_WIDTH, PLAYER_HEIGHT, plat.x, plat.y, PLATFORM_WIDTH, PLATFORM_HEIGHT);
//         }
//         PlatformResult::HorizontalMovingPlatform(plat) => {
//             return default_collision(px,py, PLAYER_WIDTH, PLAYER_HEIGHT, plat.x, plat.y, PLATFORM_WIDTH, PLATFORM_HEIGHT);
//         }
//         PlatformResult::Blank(play) => {return false;}
//         _ => {return false;}
//     }
    
// }
