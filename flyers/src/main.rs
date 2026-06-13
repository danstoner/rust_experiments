#[allow(unused_variables,dead_code)]

use macroquad::prelude::*;

mod structs;
use structs::*;


#[macroquad::main("BasicShapes")]
async fn main() {


    let center_x: f32 = screen_width() / 2.0;
    let center_y: f32 = screen_height() / 2.0;


    let mut ships_near: Vec<Flyer> = Vec::new();
    let mut ships_med: Vec<Flyer> = Vec::new();
    let mut ships_far: Vec<Flyer> = Vec::new();

    let mut remaining: u16;

    // depends on having three colors
    let bucket_size = FLYER_MAX_SIZE as f32 / 3.0;

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            // clear and start over
            ships_near.clear();
            ships_med.clear();
            ships_far.clear();
        }

        // if ships.is_empty() {
        //     for ship in 1..=MAX_FLYERS {
        //         ships.push(Flyer {
        //             speed: fastrand::f32() * (FLYER_MAX_SPEED - FLYER_MIN_SPEED) + FLYER_MIN_SPEED,
        //             size: fastrand::u8(FLYER_MIN_SIZE..=FLYER_MAX_SIZE),
        //             direction_radians: (fastrand::f32() * 180.0_f32).to_radians(),
        //             location_x: fastrand::f32() * screen_width(),
        //             location_y: screen_height(),
        //             destroyed: false,
        //         });
        //     }
        // }

        remaining = ships_near.len() as u16  + ships_med.len() as u16 + ships_far.len() as u16;
        draw_text(&format!("Remaining: {}", remaining), 0.0, 35.0, 30.0, RED);

        if remaining < MAX_FLYERS {

            let flyer_size: u8 = fastrand::u8(FLYER_MIN_SIZE..=FLYER_MAX_SIZE);
            let bucket_size = FLYER_MAX_SIZE / 3;

            if flyer_size <= bucket_size {
                ships_far.push(Flyer {
                    speed: fastrand::f32() * (FLYER_MAX_SPEED - FLYER_MIN_SPEED) + FLYER_MIN_SPEED,
                    size: flyer_size,
                    direction_radians: (fastrand::f32() * 180.0_f32).to_radians(),
                    location_x: fastrand::f32() * screen_width(),
                    location_y: screen_height(),
                    destroyed: false,
                    color: LIGHTLIGHTGRAY,
                });
            } else if flyer_size <= bucket_size * 2 {
                 ships_med.push(Flyer {
                    speed: fastrand::f32() * (FLYER_MAX_SPEED - FLYER_MIN_SPEED) + FLYER_MIN_SPEED,
                    size: flyer_size,
                    direction_radians: (fastrand::f32() * 180.0_f32).to_radians(),
                    location_x: fastrand::f32() * screen_width(),
                    location_y: screen_height(),
                    destroyed: false,
                    color: LIGHTLIGHTLIGHTGRAY,
                });
            } else {
                ships_near.push(Flyer {
                    speed: fastrand::f32() * (FLYER_MAX_SPEED - FLYER_MIN_SPEED) + FLYER_MIN_SPEED,
                    size: flyer_size,
                    direction_radians: (fastrand::f32() * 180.0_f32).to_radians(),
                    location_x: fastrand::f32() * screen_width(),
                    location_y: screen_height(),
                    destroyed: false,
                    color: NEARWHITE,
                });
            }
   
            


        }

        // if is_mouse_button_pressed(MouseButton::Left) {
        //     let (mouse_x, mouse_y) = mouse_position();
        //     println!("Left click registered at: X: {}, Y: {}", mouse_x, mouse_y);
        //     circle_pos_x = mouse_x;
        //     circle_pos_y = mouse_y;
        //     circle_size = start_size;
        //     sleeper = 0;
        // }

        for ship in &ships_far {
            draw_poly(
                ship.location_x as f32,
                ship.location_y as f32,
                255,
                ship.size as f32,
                0.0,
                // Color::new(0.0, 0.0, 0.0, 100.0 / (100.0 - ship.size as f32)),
                ship.color
            );
        }

        for ship in &ships_med {
            draw_poly(
                ship.location_x as f32,
                ship.location_y as f32,
                255,
                ship.size as f32,
                0.0,
                // Color::new(0.0, 0.0, 0.0, 100.0 / (100.0 - ship.size as f32)),
                ship.color
            );
        }

        for ship in &ships_near {
            draw_poly(
                ship.location_x as f32,
                ship.location_y as f32,
                255,
                ship.size as f32,
                0.0,
                // Color::new(0.0, 0.0, 0.0, 100.0 / (100.0 - ship.size as f32)),
                ship.color
            );
        }

        // draw_poly(circle_pos_x, circle_pos_y, 255, circle_size, 0.0, YELLOW);


        // let sleeper_text_template = String::from("Sleeper counter: {counter}");

        // let sleeper_text = sleeper_text_template.replace("{counter}", &sleeper.to_string());
        // draw_text(&sleeper_text, 20.0, 20.0, 30.0, RED);




        // Update ship locations
        for ship in &mut ships_far {

            let direction_change = (fastrand::f32() - 0.5) * 0.1; // Random small change in direction
            ship.direction_radians += direction_change;
            ship.location_y -= ship.speed;
            if ship.location_y < 0.0 {
                ship.destroyed = true;
            } else {
                ship.location_x += ship.speed * ship.direction_radians.cos();
                ship.location_y -= ship.speed * ship.direction_radians.sin();
                // ship.location_y = ship.location_y - ship.speed;
                //
                // ship.location_x = fastrand::f32() * screen_width();
                // ship.speed = fastrand::u8(FLYER_MIN_SPEED..=FLYER_MAX_SPEED);
                // ship.size = fastrand::u8(FLYER_MIN_SIZE..=FLYER_MAX_SIZE);
            }
        }
        for ship in &mut ships_med {

            let direction_change = (fastrand::f32() - 0.5) * 0.1; // Random small change in direction
            ship.direction_radians += direction_change;
            ship.location_y -= ship.speed;
            if ship.location_y < 0.0 {
                ship.destroyed = true;
            } else {
                ship.location_x += ship.speed * ship.direction_radians.cos();
                ship.location_y -= ship.speed * ship.direction_radians.sin();
                // ship.location_y = ship.location_y - ship.speed;
                //
                // ship.location_x = fastrand::f32() * screen_width();
                // ship.speed = fastrand::u8(FLYER_MIN_SPEED..=FLYER_MAX_SPEED);
                // ship.size = fastrand::u8(FLYER_MIN_SIZE..=FLYER_MAX_SIZE);
            }
        }

        for ship in &mut ships_near {

            let direction_change = (fastrand::f32() - 0.5) * 0.1; // Random small change in direction
            ship.direction_radians += direction_change;
            ship.location_y -= ship.speed;
            if ship.location_y < 0.0 {
                ship.destroyed = true;
            } else {
                ship.location_x += ship.speed * ship.direction_radians.cos();
                ship.location_y -= ship.speed * ship.direction_radians.sin();
                // ship.location_y = ship.location_y - ship.speed;
                //
                // ship.location_x = fastrand::f32() * screen_width();
                // ship.speed = fastrand::u8(FLYER_MIN_SPEED..=FLYER_MAX_SPEED);
                // ship.size = fastrand::u8(FLYER_MIN_SIZE..=FLYER_MAX_SIZE);
            }
        }


        // clean up destroyed ships
        ships_far.retain(|ship| !ship.destroyed);
        ships_med.retain(|ship| !ship.destroyed);
        ships_near.retain(|ship| !ship.destroyed);


        draw_fps();
        next_frame().await;
    }

}
