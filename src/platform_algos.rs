use rand::Rng;

use crate::platforms::*;

use crate::PlatformResult;

use crate::collisions::*;

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
    } else if random == 4 {
        let platform: PlatformResult = PlatformResult::VerticalMovingPlatform(VerticalMovingPlatform::new(i, t));
        return platform;
    } else {
        return PlatformResult::Blank(BlankPlatform::new(i, t));
    }   
}

pub fn determine_section() -> i32 {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let first_section_start: i32 = rng.gen_range(0..=80);

    println!("Special platforms start here {}", &first_section_start);

    first_section_start
}

pub fn is_in_section(score: i32, section: i32) -> bool {
    if section < score && score < section + 20 {
        return true;
    }
    false
}

pub fn generate_special_platform(x: f32, y: f32) -> PlatformResult {
    // let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    // println!("triggered");
    PlatformResult::HorizontalMovingPlatform(HorizontalMovingPlatform::new(x, y))
}

pub fn check_proximity(index: &i32, platforms: &Vec<PlatformResult>) -> i32 {
    let mut check = 0;
    let mut px = 0.0;
    let mut py = 0.0;
    let w = crate::PLATFORM_WIDTH;
    let h = crate::PLATFORM_HEIGHT;
    match platforms[*index as usize] {
        PlatformResult::BasicPlatform(ref platform) => {
            px = platform.x;
            py = platform.y;
        }
        PlatformResult::Blank(ref platform) => {
            px = platform.x;
            py = platform.y;
        }
        PlatformResult::HorizontalMovingPlatform(ref platform) => {
            px = platform.x;
            py = platform.y;
        }
        PlatformResult::VerticalMovingPlatform(ref platform) => {
            px = platform.x;
            py = platform.y;
        }

    }
    for platform in platforms.iter() {
        match platform {
            PlatformResult::BasicPlatform(ref platform) => {
                if default_collision(px - w, py - h, w * 3.0, h * 3.0, platform.x, platform.y, w, h){
                    check = 3;
                }
            }
            PlatformResult::Blank(ref platform) => {
                
            }
            PlatformResult::HorizontalMovingPlatform(ref platform) => {
                if default_collision(px - w, py - h, w * 3.0, h * 3.0, platform.x, platform.y, w, h){
                    check = 3;
                }
            }
            PlatformResult::VerticalMovingPlatform(ref platform) => {
                if default_collision(px - w, py - h, w * 3.0, h * 3.0, platform.x, platform.y, w, h){
                    check = 3;
                }
            }
            _ => {}
    }
    
    
}
let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
//return (rng.gen_range(0..=5) + check);
return check;
}