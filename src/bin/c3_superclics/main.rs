slint::include_modules!();
use std::time::Instant;

use rand::Rng;


fn move_circle(handler : &AppWindow) {
    let width = handler.get_area_width();
    let height = handler.get_area_height();

    move_circle_into(&handler, width, height);
}

fn move_circle_into(handler : &AppWindow, width : f32, height : f32) {
    let mut rng = rand::thread_rng();

    let new_x = rng.gen_range(0.0..(width - 60.0));
    let new_y = rng.gen_range(0.0..(height - 60.0));

    handler.set_circle_x(new_x.into());
    handler.set_circle_y(new_y.into());
}

fn incr_score(handler : &AppWindow, start : &mut Instant) {
    let time = start.elapsed().as_secs_f32();

    let score : f32 = handler.get_score() as f32;
    let score = score + (10000.0 / time);

    handler.set_score(score as i32);

    *start = Instant::now();

}

fn main() {

    let mut start = Instant::now();

    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();
    
    let ui_handle_click = ui_handle.clone();
    ui.on_circle_clicked(move || {
        let ui_handle_click = ui_handle_click.unwrap();

        
        incr_score(&ui_handle_click, &mut start);
        move_circle(&ui_handle_click);
    });
    
    let ui_handle_resize = ui_handle.clone();
    ui.on_size_changed(move|w, h| {
        move_circle_into(&ui_handle_resize.unwrap(), w,  h);
    });

    ui.run().unwrap();
}