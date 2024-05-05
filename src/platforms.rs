use rand::Rng;

pub trait Platform {
    fn new(x: f32, y: f32) -> Self;
    fn position(&self) -> (f32, f32);
}

#[derive(Debug, Clone, Copy)]
pub enum PlatformResult {
    BasicPlatform(BasicPlatform),
    HorizontalMovingPlatform(HorizontalMovingPlatform),
    VerticalMovingPlatform(VerticalMovingPlatform),
    Blank(BlankPlatform),
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct VerticalMovingPlatform {
    pub x: f32,
    pub y: f32,
    pub direction: bool,
    pub delta: f32,
}

impl Platform for VerticalMovingPlatform {
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

impl VerticalMovingPlatform {
    pub fn shift(&mut self, direction: bool) {
        if direction {
            self.y += self.delta;
        } else {
            self.y -= self.delta;
        }
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