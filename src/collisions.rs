use crate::PlatformResult;

pub fn default_collision( x1 :f32, y1 :f32, w1 :f32, h1 :f32, x2 :f32, y2 :f32, w2 :f32,  h2 :f32) -> bool {
    if ((x1 + w1) > x2 && x1 < x2) || ((x2 + w2) > x1 && x2 < x1)
    {
        if ((y1 + h1) > y2 && y1 < y2) || ((y2 + h2) > y1 && y2 < y1){
            return true;
        }
    }
    return false;
}

pub fn player_plat_collision( px :f32, py :f32,  platEnum : &PlatformResult) -> bool{
    match platEnum {
        PlatformResult::BasicPlatform(plat) => {
            return default_collision(px,py, crate::PLAYER_WIDTH, crate::PLAYER_HEIGHT, plat.x, plat.y, crate::PLATFORM_WIDTH, crate::PLATFORM_HEIGHT);
        }
        PlatformResult::HorizontalMovingPlatform(plat) => {
            return default_collision(px,py, crate::PLAYER_WIDTH, crate::PLAYER_HEIGHT, plat.x, plat.y, crate::PLATFORM_WIDTH, crate::PLATFORM_HEIGHT);
        }
        PlatformResult::Blank(play) => {return false;}
        _ => {return false;}
    }
    
}