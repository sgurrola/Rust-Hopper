use std::time::Instant;
use notan::app::Texture;

pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub velocity: f32,
    pub direction: f32,
    pub proj_text: Texture,
}


impl Projectile {
    pub fn new(x: f32, y: f32, velocity: f32, direction: f32, proj_text: Texture) -> Self{
        Self {
            x,
            y,
            velocity,
            direction,
            proj_text,
        }
    }


    pub fn update(&mut self, dt: f32) {
        // Update projectile position based on velocity and direction
        self.x += self.velocity * self.direction.cos() * dt;
        self.y += self.velocity * self.direction.sin() * dt;
    }
}

pub fn shoot_projectile(state: &mut crate::State) { //need to add delay
    let time_since_last_shot = Instant::now().duration_since(state.last_shot_time);

    // Check if enough time has passed since the last shot
    if time_since_last_shot >= state.fire_delay {
        // If enough time has passed, update the last shot time
        state.last_shot_time = Instant::now();

        state.shoot = true;
    let direction_shift;
    let x = state.x;
    let y = state.y;
    let velocity = -300.0;
    if state.facing > 0.0 {direction_shift = 0.7}
    else if state.facing < 0.0 {direction_shift = -0.7}
    else {direction_shift = 0.0}
    let direction = 1.57 + direction_shift; // Use player's facing direction (not rn)

    let projectile = Projectile::new(x, y, velocity, direction, state.proj_text.clone());
    state.projectiles.push(projectile); 
    }
}

pub fn update_projectiles(state: &mut crate::State, dt: f32) {
    // Update all projectiles
    for projectile in &mut state.projectiles {
        projectile.update(dt);
        //println!("{}", dt);
    }


    // Remove projectiles that are out of bounds
    state.projectiles.retain(|projectile| {
        let x = projectile.x;
        let y = projectile.y;
        x > 0.0 && x < crate::WINDOW_X as f32 && y > 0.0 && y < crate::WINDOW_Y as f32
    });
}
