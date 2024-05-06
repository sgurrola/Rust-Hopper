use std::time::Instant;
use notan::app::Texture;

pub struct Poop {
    pub x: f32,
    pub y: f32,
    pub velocity: f32,
    pub poop_text: Texture,
}


impl Poop {
    pub fn new(x: f32, y: f32, velocity: f32, poop_text: Texture) -> Self{
        Self {
            x,
            y,
            velocity,
            poop_text,
        }
    }
    pub fn update(&mut self, dt: f32) {
        // Update projectile position based on velocity and direction
        self.x;
        self.y += self.velocity * dt; //acceleration??
    }
}

pub fn shoot_poopies(state: &mut crate::State, x: f32, y: f32) { //need to add delay
    let time_since_last_shot = Instant::now().duration_since(state.last_shot_time) / 10;

    // Check if enough time has passed since the last shot
    if time_since_last_shot >= state.fire_delay {
        // If enough time has passed, update the last shot time
        state.last_shot_time = Instant::now();
    let x = x;
    let y = y;
    let velocity = 75.0;


    let poop = Poop::new(x, y, velocity, state.poop_text.clone());
    state.poop.push(poop); 
    println!("poop for you");
    }
}

pub fn update_poopies(state: &mut crate::State, dt: f32) {
    // Update all projectiles
    for poop in &mut state.poop {
        poop.update(dt);
        //println!("poop update!");
        
    }


    // Remove projectiles that are out of bounds
    state.poop.retain(|poop| {
        let x = poop.x;
        let y = poop.y;
        x > 0.0 && x < crate::WINDOW_X as f32 && y > 0.0 && y < crate::WINDOW_Y as f32
    });
}
