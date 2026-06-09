#[allow(unused_variables,dead_code)]

use macroquad::prelude::*;

mod structs;
use structs::*;

#[macroquad::main("BasicShapes")]
async fn main() {


    let center_x: f32 = screen_width() / 2.0;
    let center_y: f32 = screen_height() / 2.0;



    let mut ships: Vec<Flyer> = Vec::new();


    for ship in 1..=MAX_FLYERS {
        ships.push(Flyer {
            id: ship,
            speed: fastrand::f32() * (FLYER_MAX_SPEED - FLYER_MIN_SPEED) + FLYER_MIN_SPEED,
            size: fastrand::u8(FLYER_MIN_SIZE..=FLYER_MAX_SIZE),
            direction: 0,
            location_x: fastrand::f32() * screen_width(),
            location_y: screen_height(),
            destroyed: false,
        });
    }


    
    let mut sleeper: u32 = 0;

    loop {
        clear_background(BLUE);


        // if is_mouse_button_pressed(MouseButton::Left) {
        //     let (mouse_x, mouse_y) = mouse_position();
        //     println!("Left click registered at: X: {}, Y: {}", mouse_x, mouse_y);
        //     circle_pos_x = mouse_x;
        //     circle_pos_y = mouse_y;
        //     circle_size = start_size;
        //     sleeper = 0;
        // }

        for ship in &ships {
            draw_poly(
                ship.location_x as f32,
                ship.location_y as f32,
                255,
                ship.size as f32,
                0.0,
                BLACK,
            );
        }
        // draw_poly(circle_pos_x, circle_pos_y, 255, circle_size, 0.0, YELLOW);

        // sleep for a certain number of ticks before updating the next position and size of the circle

        if sleeper < SLEEP_TIME {
            sleeper += 1;
        } else {
            // if fastrand::bool() {
            //     circle_pos_x = circle_pos_x + 2.0;
            // } else {
            //     circle_pos_x = circle_pos_x - 2.0
            // };

            // if fastrand::bool() {
            //     circle_pos_y = circle_pos_y + 2.0;
            // } else {
            //     circle_pos_y = circle_pos_y - 2.0
            // };
            // circle_size = circle_size + 1.0;

            sleeper = 0;
        }

        let sleeper_text_template = String::from("Sleeper counter: {counter}");

        let sleeper_text = sleeper_text_template.replace("{counter}", &sleeper.to_string());
        draw_text(&sleeper_text, 20.0, 20.0, 30.0, RED);




        // Update ship locations
        for ship in &mut ships {
            ship.location_y -= ship.speed;
            if ship.location_y < 0.0 {
                ship.destroyed = true;
            } else {
                ship.location_y = ship.location_y - ship.speed;
                // ship.location_x = fastrand::f32() * screen_width();
                // ship.speed = fastrand::u8(FLYER_MIN_SPEED..=FLYER_MAX_SPEED);
                // ship.size = fastrand::u8(FLYER_MIN_SIZE..=FLYER_MAX_SIZE);
            }
        }


        // clean up destroyed ships
        ships.retain(|ship| !ship.destroyed);


        next_frame().await;
    }

}
