use crate::PlatformResult;
use crate::Poop;
use crate::Enemies;

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

pub fn player_enemy_collision( px :f32, py :f32,  enemy : &Enemies) -> bool{
    match enemy{
        Enemies::StaticEnemy(pe) => {
            return default_collision(px,py, crate::PLAYER_WIDTH, crate::PLAYER_HEIGHT, pe.x, pe.y, 20.0, 20.0);
        }
        Enemies::MovingEnemy(me) => {
            return default_collision(px,py, crate::PLAYER_WIDTH, crate::PLAYER_HEIGHT, me.x, me.y, 30.0, 30.0);
        }
        Enemies::PoopyEnemy(po) => {
            return default_collision(px,py, crate::PLAYER_WIDTH, crate::PLAYER_HEIGHT, po.x, po.y, 60.0, 60.0);
        }
    }
}

 //currently does nothing
pub fn projectile_enemy_collision( px :f32, py :f32,  enemy : &Enemies) -> bool{
    match enemy{ 
        Enemies::StaticEnemy(pe) => {
           return default_collision(px,py, 20.0, 20.0, pe.x, pe.y, 20.0, 20.0);
        }
        Enemies::MovingEnemy(me) => {
            return default_collision(px,py, 20.0, 20.0, me.x, me.y, 30.0, 30.0);
        }
        Enemies::PoopyEnemy(po) => {
            return default_collision(px,py, 20.0, 20.0, po.x, po.y, 60.0, 60.0);
        }
    }
}

pub fn player_poop_collision( px :f32, py :f32, poop: Poop) -> bool{
    //return default_collision(px,py, 20.0, 20.0, po.x, po.y, 60.0, 60.0);
    return false;
}