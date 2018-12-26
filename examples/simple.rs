#![allow(dead_code)]
extern crate orbclient;
extern crate rust_cairo;

use orbclient::{Color, Window, Renderer, EventOption};
use rust_cairo::*;

fn main() {
    let w = 800;
    let h = 600;
    let (width, height) = orbclient::get_display_size().unwrap();
    let mut window = Window::new_flags((width as i32) / 4,
                                       (height as i32) / 4,
                                       w,
                                       h,
                                       "Cairo", &[orbclient::WindowFlag::Async]).unwrap();
    let (win_w, win_h) = (w, h);
    window.rect(0, 0, win_w, win_h, Color::rgb(255, 255, 255));


    unsafe {
        let surface = cairo_image_surface_create_for_data(window.data_mut().as_mut_ptr() as *mut u8, CAIRO_FORMAT_ARGB32, win_w as i32, win_h as i32, cairo_format_stride_for_width(CAIRO_FORMAT_ARGB32, win_w as i32));
        let cr = cairo_create(surface);

        let m_pi = 3.14159265;
        cairo_set_line_width(cr, 9.0);
        cairo_set_source_rgb(cr, 0.0, 1.0, 0.0);
        cairo_translate(cr, win_w as f64 /2.0, win_h as f64 /2.0);
        cairo_arc(cr, 0.0, 0.0, 50.0, 0.0, 2.0 * m_pi);
        cairo_stroke_preserve(cr);
        cairo_set_source_rgb(cr, 1.0, 0.0, 0.0);
        cairo_fill(cr);
    }

    window.sync();

    'event: loop {
        for orbital_event in window.events() {
            match orbital_event.to_option() {
                EventOption::Quit(_quit_event) => break 'event,
                _ => (),
            };
        }
    }
}
