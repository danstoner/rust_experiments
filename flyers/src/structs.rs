#[allow(unused_variables,dead_code)]

use macroquad::prelude::*;

pub struct Flyer {
    pub speed: f32,
    pub size: u8,
    pub direction_radians: f32,
    pub location_x: f32,
    pub location_y: f32,
    pub destroyed: bool,
    pub color: Color,
}


pub const MAX_FLYERS: u16 = 100;
pub const FLYER_MIN_SPEED: f32 = 0.5;
pub const FLYER_MAX_SPEED: f32 = 1.2;
pub const FLYER_MIN_SIZE: u8 = 5;
pub const FLYER_MAX_SIZE: u8 = 20;


