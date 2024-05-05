use notan::app::Texture;
use rand::Rng;

#[derive(Debug)]
pub enum Enemies {
    StaticEnemy(StaticEnemy),
    MovingEnemy(MovingEnemy),
    PoopyEnemy(PoopyEnemy),
}

#[derive(Debug)]
pub struct StaticEnemy {
    pub x: f32,
    pub y: f32,
    pub enemy_text: Texture,
}

impl StaticEnemy {
    fn new(x: f32, y: f32, enemy_text: Texture) -> Self{
        Self {
            x,
            y,
            enemy_text,
        }
    }
}

#[derive(Debug)]
pub struct MovingEnemy {
    pub x: f32,
    pub y: f32,
    pub direction: bool,
    pub delta: f32,
    pub enemy_text: Texture,
}


impl MovingEnemy {
    fn new(x: f32, y: f32, enemy_text: Texture) -> Self{
        Self {
            x,
            y,
            direction: true,
            delta: generate_move_delta(),
            enemy_text,
        }
    }
    pub fn shift(&mut self, direction: bool) {
        if direction {
            self.x += self.delta;
        } else {
            self.x -= self.delta;
        }
    }
}

#[derive(Debug)]
pub struct PoopyEnemy {
    pub x: f32,
    pub y: f32,
    pub enemy_text: Texture,
}

impl PoopyEnemy {
    fn new(x: f32, y: f32, enemy_text: Texture) -> Self{
        Self {
            x,
            y,
            enemy_text,
        }
    }
}

pub fn spawn_enemy(state: &mut crate::State){ 
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let random: f32 = rng.gen_range(50.0..=550.0); //randomly selects an x point for character to spawn at
    let enemy_type: i32 = rng.gen_range(1..=4);
    let thing = enemy_type;
    let x: f32 = random;
    let y: f32 = state.y - 300.0;
    let mut enemies: Enemies = Enemies::StaticEnemy(StaticEnemy::new(x, y, state.haskell_text.clone()));
    //println!("{}", thing);
    if thing == 2 || (state.score > 70 && (thing == 2 || thing == 3)){
        enemies = Enemies::MovingEnemy(MovingEnemy::new(x, y, state.python_text.clone()));
        //println!("moving enemy spawned");
    } else if thing == 4 && state.score > 40{
        enemies = Enemies::PoopyEnemy(PoopyEnemy::new(x, y, state.java_text.clone()));
        //println!("poopy enemy spawned");
    }
    state.enemies.push(enemies);
    //println!("enemy spawned");
}

pub fn generate_move_delta() -> f32 {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let delta: f32 = rng.gen_range(1.0..=7.0);
    delta
}
//for enemy in state.enemies.iter_mut(){
//    match enemy {
//        Enemies::StaticEnemy(_pe) => {
//        }

//fn update_enemies(state: &mut State, dt: f32) {}


//left to add to main
//actually spawning enemies
//collisions
    //player
    //projectiles