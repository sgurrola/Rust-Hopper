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
}

const MAX_SPEED: f32 = 350.0; // the max speed the player can go
const ACCELERATION_RATE: f32 = 700.0; // how fast the player accelerates
const GRAVITY: f32 = 160.0; // the speed at which the player falls
const MAX_FALL: f32 = 250.0; // the max rate the player can fall
const TEMP_GROUND: f32 = 600.0; // dummy ground for testing bouncing
const CROSS_ACCEL: f32 = 2.0; // acceleration boost for going in opposite direction
const STOP_ACCEL: f32 = 3.0; // acceleration boost for coming to a stop
const WINDOW_X: u32 = 600; //sets the width of the game window
const WINDOW_Y: u32 = 800; //sets the height of the game window
const PLAYER_WIDTH: f32 = 80.0; // width of player sprite
const PLAYER_HEIGHT: f32 = 80.0; //height of player sprite
const BOUNCE_HEIGHT: f32 = -250.0; //player jump height, its negative because y zero is at top of screen


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
        .from_image(include_bytes!("assets/guy.jpg"))
        .build()
        .unwrap();
    State { 
        img: texture,
        x: 100.0,
        y: 100.0,
        x_vel: 0.0,
        y_vel:0.0,
    }
}

//this is the logic that runs each frame
fn update(app: &mut App, state: &mut State) {

    //for moving left
    if app.keyboard.is_down(KeyCode::A) {
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
    else{
        state.y += state.y_vel * app.timer.delta_f32();
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
}

//this is the draw function, does all of the rendering each frame
fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.image(&state.img).size(PLAYER_WIDTH,PLAYER_HEIGHT).position(state.x, state.y);
    draw.image(&state.img).size(40.0,120.0).position(400.0, 200.0);
    draw.image(&state.img).size(40.0,120.0).position(300.0, 100.0);
    gfx.render(&draw);
}