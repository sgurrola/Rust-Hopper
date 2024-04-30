use rand::Rng;

use crate::platforms::*;

use crate::PlatformResult;

pub fn determine_sections() -> (i32, i32) {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    let first_section_start: i32 = rng.gen_range(0..=50);
    let second_section_start: i32 = rng.gen_range(50..=100);

    (first_section_start, second_section_start)
}

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

// pub fn check_and_regenerate_platforms(platform_list: Vec<PlatformResult>, ) {
//     for platform in state.platform_list.iter_mut() {
//         match platform {
//             PlatformResult::BasicPlatform(basic_platform) => {
//                 if basic_platform.y > WINDOW_Y_FLOAT {
//                     basic_platform.y = 0.0;
//                     *platform = spawn_platform(basic_platform.x, basic_platform.y, state.score);
//                     state.score += 1;
//                 }
//             }
//             PlatformResult::Blank(blank_platform) => {
//                 if blank_platform.y > WINDOW_Y_FLOAT {
//                     blank_platform.y = 0.0;
//                     *platform = spawn_platform(blank_platform.x, blank_platform.y, state.score);
//                 }
//             }
//             PlatformResult::HorizontalMovingPlatform(horizontal_platform) => {
//                 if horizontal_platform.x <= 0.0 {
//                     horizontal_platform.direction = true;
//                 } else if horizontal_platform.x >= WINDOW_X_FLOAT - PLATFORM_WIDTH {
//                     horizontal_platform.direction = false;
//                 }
//                 horizontal_platform.shift(horizontal_platform.direction);

//                 if horizontal_platform.y > WINDOW_Y_FLOAT {
//                     horizontal_platform.y = 0.0;
//                     *platform = spawn_platform(horizontal_platform.x, horizontal_platform.y, state.score);
//                 }
//             }
//             PlatformResult::VerticalMovingPlatform(vertical_platform) => {
                
//             }
//         }
//     }
// }