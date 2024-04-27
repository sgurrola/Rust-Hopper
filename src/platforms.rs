use rand::Rng;

pub trait Platform {
    fn new(x: f32, y: f32) -> Self;
    fn position(&self) -> (f32, f32);
}

#[derive(Debug)]
pub enum PlatformResult {
    BasicPlatform(BasicPlatform),
    HorizontalMovingPlatform(HorizontalMovingPlatform),
    Blank(BlankPlatform),
}

#[derive(Debug)]
pub struct BlankPlatform {
    pub x: f32,
    pub y: f32,
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
pub struct BasicPlatform {
    pub x: f32,
    pub y: f32,
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
pub struct HorizontalMovingPlatform {
    pub x: f32,
    pub y: f32,
    pub direction: bool,
    pub delta: f32,
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
    pub fn shift(&mut self, direction: bool) {
        if direction {
            self.x += self.delta;
        } else {
            self.x -= self.delta;
        }
    }
}

// struct verticalMovingPlatform {

// }

pub fn spawn_platforms(platforms: &mut Vec<PlatformResult>) {
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

pub fn spawn_platform(i: f32, t: f32, score: i32) -> PlatformResult {
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

pub fn generate_move_delta() -> f32 {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let delta: f32 = rng.gen_range(1.0..=7.0);
    delta
}

// fn check_proximity(platform_index: i32, platform_list: Vec<PlatformResult>) -> bool {

//     true
// }

// maybe make everything a 2d array of platform results and grab the first x value to dictate the rest of them