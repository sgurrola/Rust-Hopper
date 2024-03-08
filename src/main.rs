use notan::draw::*;
use notan::prelude::*;

//the state holds all our game data / stats / anything we need, passed to both the render and gameplay logic function
#[derive(AppState)]
struct State {
    img: Texture,
    x: f32, //actual x position of player on screen
    y: f32, //actual y position of player on screen
    x_vel: f32, //players velocity in x direction
    y_vel: f32, //players velocity in y direction
    offset: f32, //test for fake camera
    score: f32,
    anims: Vec<Anims>,
    anim: usize,
    shoot:bool,
    facing:f32,

    
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
const PLAYER_WIDTH: f32 = 80.0; // width of player sprite
const PLAYER_HEIGHT: f32 = 80.0; //height of player sprite
const BOUNCE_HEIGHT: f32 = -600.0; //player jump height, its negative because y zero is at top of screen


//just initializes the notan render + logic loop
#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new()
        .set_size(WINDOW_X, WINDOW_Y);

    notan::init_with(init)
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
        
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
    .from_image(include_bytes!("assets/guy_idle1.png"))
    .build()
    .unwrap();

    let idle2 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/guy_idle2.png"))
    .build()
    .unwrap();

    let idle3 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/guy_idle3.png"))
    .build()
    .unwrap();

    let fall4 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/guy_falling1.png"))
    .build()
    .unwrap();

    let fall5 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/guy_falling2.png"))
    .build()
    .unwrap();

    let fall6 = gfx
    .create_texture()
    .from_image(include_bytes!("assets/guy_falling3.png"))
    .build()
    .unwrap();

    let temp = Anims::Idle(Animation{anims:vec![idle1, idle2, idle3], timing:0.0,frame:0, speed:0.12},0);
    let temp1 = Anims::Falling(Animation{anims:vec![fall4, fall5, fall6], timing:0.0, frame:0, speed:0.12}, 1);
    State {
        img: texture,
        x: 100.0,
        y: 100.0,
        x_vel: 0.0,
        y_vel:0.0,
        offset:0.0,
        score:0.0,
        anim:0,
        anims: vec![temp, temp1],
        shoot: false,
        facing: 1.0,
    }
}

//this is the logic that runs each frame
fn update(app: &mut App, state: &mut State) {

    //for moving left
    if app.keyboard.is_down(KeyCode::A) {
        if state.facing > 0.0 {
            state.x += PLAYER_WIDTH;
        }
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
        if state.facing < 0.0 {
            state.x -= PLAYER_WIDTH;
            
        }
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
    if state.y + (state.y_vel * app.timer.delta_f32()) > TEMP_GROUND {
        println!("y {} and vel {} before bounce", state.y, state.y_vel);
        state.y = TEMP_GROUND;
        state.y_vel = BOUNCE_HEIGHT;
        println!("bounce here {}", state.y_vel);
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

    //This moves the platforms up if the player is moving up and is in the top 2/3rds of the screen
    if state.y < 500.0 && state.y_vel < 0.0 {
        state.offset -= state.y_vel * app.timer.delta_f32();
    }

    if state.y_vel < 0.0 {
        state.score -= state.y_vel * app.timer.delta_f32() * 0.1;
        println!("score is {}", state.score)
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
                anime.frame = (anime.frame + 1);
                anime.timing = 0.0;
            }
            if anime.frame >= anime.anims.len() as i32{
                state.anim = 0;
                anime.frame = 0;
                anime.timing = 0.0;
            }
        }
     }
    
}

//this is the draw function, does all of the rendering each frame
fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    let thing;
    match &state.anims[state.anim]{
        Anims::Idle(anime, _) | Anims::Falling(anime, _) | Anims::Shooting(anime, _) => thing = &anime.anims[anime.frame as usize],
        _ => thing = &state.img,
    }
    draw.image(thing).size(state.facing * PLAYER_WIDTH,PLAYER_HEIGHT).position(state.x, state.y);
    draw.image(&state.img).size(40.0,120.0).position(400.0, 200.0 + state.offset);
    draw.image(&state.img).size(40.0,120.0).position(300.0, 100.0 + state.offset);
    gfx.render(&draw);
}