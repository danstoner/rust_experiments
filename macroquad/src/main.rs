use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {

    const SLEEP_TIME: u32 = 3;

    let start_x: f32 = screen_width() / 2.0;
    let start_y: f32 = screen_height() / 2.0;


    let mut circle_pos_x = start_x;
    let mut circle_pos_y = start_y;
    let mut circle_size: f32 = 15.0;
    let mut sleeper: u32 = 0;

    loop {
        clear_background(BLACK);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_circle(circle_pos_x, circle_pos_y, circle_size, YELLOW);
        draw_poly(circle_pos_x, circle_pos_y, 255, circle_size, 0.0, YELLOW);


        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, RED);


        // only draw if we've slept for a certain number of ticks

        if sleeper < SLEEP_TIME {
            sleeper += 1;
        } else {
        if fastrand::bool() {
            circle_pos_x = circle_pos_x + 2.0;
        } else { circle_pos_x = circle_pos_x - 2.0 };
        
        if fastrand::bool() {
            circle_pos_y = circle_pos_y + 2.0;
        } else { circle_pos_y = circle_pos_y - 2.0 };
        circle_size = circle_size + 1.0;

        sleeper = 0;
        }        

        let sleeper_text_template = String::from("Sleeper counter: {counter}");

        let sleeper_text = sleeper_text_template.replace("{counter}", &sleeper.to_string());
        draw_text(&sleeper_text, 20.0, 20.0, 30.0, RED);
        next_frame().await;
    }
}