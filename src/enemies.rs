use notan::app::Texture;
use rand::Rng;

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub enemy_text: Texture,
}


impl Enemy {
    pub fn new(x: f32, y: f32, enemy_text: Texture) -> Self{
        Self {
            x,
            y,
            enemy_text,
        }
    }
}

pub fn spawn_enemy(state: &mut crate::State){ //need to add to main actual spawning
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    // very rudimentary formula for when score gets larger to spawn in less platforms 
    // score is increasing when a platform goes by
    let random = rng.gen_range(50.0..=550.0);
    let x = random;
    let y = state.y - 300.0;
    let enemies = Enemy::new(x, y, state.enemy_text.clone());
    state.enemies.push(enemies);
}

//fn update_enemies(state: &mut State, dt: f32) {}


//left to add to main
//actually spawning enemies
//collisions
    //player
    //projectiles