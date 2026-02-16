slint::include_modules!();
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

fn incr_score(handler : &AppWindow) {
    handler.set_score(handler.get_score() + 1);
}

fn main() {
    let ui = AppWindow::new().unwrap();
    let ui_handle = ui.as_weak();
    
    let ui_handle_click = ui_handle.clone();
    ui.on_circle_clicked(move || {
        let ui_handle_click = ui_handle_click.unwrap();

        incr_score(&ui_handle_click);
        move_circle(&ui_handle_click);
    });
    
    let ui_handle_resize = ui_handle.clone();
    ui.on_size_changed(move|w, h| {
        move_circle_into(&ui_handle_resize.unwrap(), w,  h);
    });

    ui.run().unwrap();
}