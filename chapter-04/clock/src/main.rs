use chrono::{Local, Timelike};
use speedy2d::{
    color::Color,
    font::{Font, FormattedTextBlock, TextLayout},
    window::{WindowHandler, WindowHelper},
    Graphics2D,
};
use std::f32::consts::PI;
use std::rc::Rc;

struct ClockData {
    margin: f32,
    radius: f32,
    hour_texts: Vec<(Rc<FormattedTextBlock>, (f32, f32))>,
    minute_marks: Vec<((f32, f32), (f32, f32))>,
    hours_len: f32,
    minutes_len: f32,
    seconds_len: f32,
}

impl ClockData {
    fn draw_background(&self, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);
        let center = (self.margin + self.radius, self.margin + self.radius);
        graphics.draw_circle(center, self.radius, Color::from_rgb(0.7, 0.8, 1.0));
        for minute_mark in &self.minute_marks {
            graphics.draw_line(
                minute_mark.0,
                minute_mark.1,
                1.0,
                Color::from_rgb(0.4, 0.4, 0.4),
            );
        }
        for hour_text in &self.hour_texts {
            graphics.draw_text((hour_text.1 .0, hour_text.1 .1), Color::BLACK, &hour_text.0);
        }
    }

    fn draw_hands(&self, graphics: &mut Graphics2D) {
        let center = (self.margin + self.radius, self.margin + self.radius);
        let local_now = Local::now();
        let local_seconds =
            local_now.num_seconds_from_midnight() as f32 + local_now.nanosecond() as f32 * 1e-9;
        let hours = local_seconds / 3600. % 12.;
        let minutes = local_seconds / 60. % 60.;
        let seconds = local_seconds % 60.;

        let hours_angle = hours / 6. * PI;
        let minutes_angle = minutes / 30. * PI;
        let seconds_angle = seconds / 30. * PI;

        let seconds_vector = (
            self.seconds_len * seconds_angle.sin(),
            self.seconds_len * -seconds_angle.cos(),
        );
        graphics.draw_line(
            center,
            (center.0 + seconds_vector.0, center.1 + seconds_vector.1),
            2.0,
            Color::from_rgb(0.3, 0.3, 0.7),
        );

        let minutes_vector = (
            self.minutes_len * minutes_angle.sin(),
            self.minutes_len * -minutes_angle.cos(),
        );
        graphics.draw_line(
            center,
            (center.0 + minutes_vector.0, center.1 + minutes_vector.1),
            3.5,
            Color::from_rgb(0.3, 0.7, 0.3),
        );

        let hours_vector = (
            self.hours_len * hours_angle.sin(),
            self.hours_len * -hours_angle.cos(),
        );
        graphics.draw_line(
            center,
            (center.0 + hours_vector.0, center.1 + hours_vector.1),
            5.0,
            Color::from_rgb(0.7, 0.3, 0.3),
        );
    }
}

impl WindowHandler for ClockData {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.draw_background(graphics);
        self.draw_hands(graphics);
        helper.request_redraw();
        //std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

fn main() {
    let margin = 8.;
    let radius = 140.;
    let hours_len = radius * 0.6;
    let minutes_len = radius * 0.8;
    let seconds_len = radius * 0.9;
    let font_size = 28.;
    let ext_radius = margin + radius;
    let font = Font::new(include_bytes!("../assets/LiberationSans-Regular.ttf")).unwrap();
    let digits_radius = radius - font_size;
    let mut hour_texts = Vec::<(Rc<FormattedTextBlock>, (f32, f32))>::new();
    for hour in 0..12 {
        let string = if hour == 0 {
            "12".to_string()
        } else {
            format!("{}", hour)
        };
        let text = font.layout_text(&string, font_size, speedy2d::font::TextOptions::new());
        let text_size = text.size();
        let angle = 2.0 * PI * hour as f32 / 12.;
        let pos = (
            ext_radius + angle.sin() * digits_radius - text_size.x / 2.,
            ext_radius - angle.cos() * digits_radius - text_size.y / 2.,
        );
        hour_texts.push((text, pos));
    }
    let mut minute_marks = Vec::<((f32, f32), (f32, f32))>::new();
    for minute in 0..60 {
        let minute_mark_radius = radius;
        let angle = 2.0 * PI * minute as f32 / 60.;
        let start_pos = (
            ext_radius + angle.sin() * seconds_len,
            ext_radius + -angle.cos() * seconds_len,
        );
        let end_pos = (
            ext_radius + angle.sin() * minute_mark_radius,
            ext_radius + -angle.cos() * minute_mark_radius,
        );
        minute_marks.push((start_pos, end_pos));
    }
    let clock_data = ClockData {
        margin,
        radius,
        hour_texts,
        minute_marks,
        hours_len,
        minutes_len,
        seconds_len,
    };
    run_app(clock_data);
}

#[cfg(target_family = "wasm")]
fn run_app(clock_data: ClockData) {
    speedy2d::WebCanvas::new_for_id("clock_canvas", clock_data).unwrap();
}

#[cfg(not(target_family = "wasm"))]
fn run_app(clock_data: ClockData) {
    let ext_radius = clock_data.margin + clock_data.radius;
    speedy2d::Window::new_centered(
        "Clock",
        ((ext_radius * 2.) as u32, (ext_radius * 2.) as u32),
    )
    .unwrap()
    .run_loop(clock_data);
}

/*
use chrono::{Local, Timelike};
use speedy2d::color::Color;
use speedy2d::font::{Font, FormattedTextBlock, TextLayout, TextOptions};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use std::f32::consts::PI;
use std::rc::Rc;
use std::time::Duration;

struct ClockData {
    center: (f32, f32),
    hour_texts: Vec<(Rc<FormattedTextBlock>, (f32, f32))>,
    hours_len: f32,
    minutes_len: f32,
    seconds_len: f32,
    radius: f32,
    minute_marks: Vec<((f32, f32), (f32, f32))>,
}

impl WindowHandler for ClockData {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.draw_background(graphics);
        self.draw_hands(graphics);
        std::thread::sleep(Duration::from_millis(50));
        helper.request_redraw();
    }
}

fn main() {
    let font_size = 28.;
    let margin = 6.;
    let radius = 160.;
    let hours_len = radius * 0.6;
    let minutes_len = radius * 0.8;
    let seconds_len = radius * 0.9;
    let center = (radius + margin, radius + margin);
    let font = Font::new(include_bytes!("../assets/LiberationSans-Regular.ttf")).unwrap();
    let digits_radius = radius - font_size / 2.;
    let mut hour_texts = Vec::<(Rc<FormattedTextBlock>, (f32, f32))>::new();
    for hour in 0..12 {
        let string = if hour == 0 {
            "12".to_string()
        } else {
            format!("{}", hour)
        };
        let text = font.layout_text(&string, font_size, TextOptions::new());
        let text_size = text.size();
        let angle = 2.0 * PI * hour as f32 / 12.;
        let pos = (
            center.0 + angle.sin() * digits_radius - text_size.x / 2.,
            center.1 + -angle.cos() * digits_radius - text_size.y / 2.,
        );
        hour_texts.push((text, pos));
    }

    let mut minute_marks = Vec::<((f32, f32), (f32, f32))>::new();
    for minute in 0..60 {
        if minute % 5 == 0 {
            continue;
        }
        let minute_notch_radius = seconds_len + 8.0;
        let angle = 2.0 * PI * minute as f32 / 60.;
        let start_pos = (
            center.0 + angle.sin() * seconds_len,
            center.1 + -angle.cos() * seconds_len,
        );
        let end_pos = (
            center.0 + angle.sin() * minute_notch_radius,
            center.1 + -angle.cos() * minute_notch_radius,
        );
        minute_marks.push((start_pos, end_pos));
    }

    // /*
    let window = speedy2d::Window::new_centered(
        "Clock",
        (
            ((radius + margin) * 2.) as u32,
            ((radius + margin) * 2.) as u32,
        ),
    )
    .unwrap();
    window.run_loop(ClockData {
        center,
        hour_texts,
        hours_len,
        minutes_len,
        seconds_len,
        radius,
        minute_marks,
    })
    // */
    //cargo install wasm-bindgen-cli
    //cargo build --target wasm32-unknown-unknown
    //wasm-bindgen target/wasm32-unknown-unknown/debug/clock.wasm --out-dir target/wasm32-unknown-unknown/debug/web --target web

    //deno install --allow-net --allow-read https://deno.land/std@0.151.0/http/file_server.ts
    //file_server .
    /*
    speedy2d::WebCanvas::new_for_id_with_user_events(
        "my_canvas",
        ClockData {
            center,
            hour_texts,
            hours_len,
            minutes_len,
            seconds_len,
            radius,
            minute_marks,
        },
    )
    .unwrap();
    */
}

impl ClockData {
    fn draw_background(&self, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);
        graphics.draw_circle(self.center, self.radius, Color::from_rgb(0.7, 0.8, 1.0));
        for minute_mark in &self.minute_marks {
            graphics.draw_line(
                minute_mark.0,
                minute_mark.1,
                1.0,
                Color::from_rgb(0.4, 0.4, 0.4),
            );
        }
        for hour_text in &self.hour_texts {
            graphics.draw_text((hour_text.1 .0, hour_text.1 .1), Color::BLACK, &hour_text.0);
        }
    }

    fn draw_hands(&self, graphics: &mut Graphics2D) {
        let local_now = Local::now();
        let local_seconds =
            local_now.num_seconds_from_midnight() as f32 + local_now.nanosecond() as f32 * 1e-9;
        let hours = local_seconds / 3600. % 12.;
        let minutes = local_seconds / 60. % 60.;
        let seconds = local_seconds % 60.;

        let hours_angle = hours / 6. * PI;
        let minutes_angle = minutes / 30. * PI;
        let seconds_angle = seconds / 30. * PI;

        let seconds_vector = (
            self.seconds_len * seconds_angle.sin(),
            self.seconds_len * -seconds_angle.cos(),
        );
        graphics.draw_line(
            self.center,
            (
                self.center.0 + seconds_vector.0,
                self.center.1 + seconds_vector.1,
            ),
            2.0,
            Color::from_rgb(0.3, 0.3, 0.7),
        );

        let minutes_vector = (
            self.minutes_len * minutes_angle.sin(),
            self.minutes_len * -minutes_angle.cos(),
        );
        graphics.draw_line(
            self.center,
            (
                self.center.0 + minutes_vector.0,
                self.center.1 + minutes_vector.1,
            ),
            3.5,
            Color::from_rgb(0.3, 0.7, 0.3),
        );

        let hours_vector = (
            self.hours_len * hours_angle.sin(),
            self.hours_len * -hours_angle.cos(),
        );
        graphics.draw_line(
            self.center,
            (
                self.center.0 + hours_vector.0,
                self.center.1 + hours_vector.1,
            ),
            5.0,
            Color::from_rgb(0.7, 0.3, 0.3),
        );
    }
}
*/
// */
/////////////////////////////////////////////////////////////////////////
/*
use chrono::{Local, Timelike};
use speedy2d::color::Color;
use speedy2d::font::{Font, FormattedTextBlock, TextLayout, TextOptions};
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, WebCanvas};
use std::f32::consts::PI;
use std::rc::Rc;
use std::time::Duration;

struct ClockData {
    center: (f32, f32),
    hour_texts: Vec<(Rc<FormattedTextBlock>, (f32, f32))>,
    hours_len: f32,
    minutes_len: f32,
    seconds_len: f32,
    radius: f32,
    minute_marks: Vec<((f32, f32), (f32, f32))>,
}

impl WindowHandler for ClockData {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        self.draw_background(graphics);
        self.draw_hands(graphics);
        std::thread::sleep(Duration::from_millis(50));
        helper.request_redraw();
    }
}

fn main() {
    let font_size = 28.;
    let margin = 6.;
    let radius = 160.;
    let hours_len = radius * 0.6;
    let minutes_len = radius * 0.8;
    let seconds_len = radius * 0.9;
    let center = (radius + margin, radius + margin);
    let font = Font::new(include_bytes!("../assets/LiberationSans-Regular.ttf")).unwrap();
    let digits_radius = radius - font_size / 2.;
    let mut hour_texts = Vec::<(Rc<FormattedTextBlock>, (f32, f32))>::new();
    for hour in 0..12 {
        let string = if hour == 0 {
            "12".to_string()
        } else {
            format!("{}", hour)
        };
        let text = font.layout_text(&string, font_size, TextOptions::new());
        let text_size = text.size();
        let angle = 2.0 * PI * hour as f32 / 12.;
        let pos = (
            center.0 + angle.sin() * digits_radius - text_size.x / 2.,
            center.1 + -angle.cos() * digits_radius - text_size.y / 2.,
        );
        hour_texts.push((text, pos));
    }

    let mut minute_marks = Vec::<((f32, f32), (f32, f32))>::new();
    for minute in 0..60 {
        if minute % 5 == 0 {
            continue;
        }
        let minute_notch_radius = seconds_len + 8.0;
        let angle = 2.0 * PI * minute as f32 / 60.;
        let start_pos = (
            center.0 + angle.sin() * seconds_len,
            center.1 + -angle.cos() * seconds_len,
        );
        let end_pos = (
            center.0 + angle.sin() * minute_notch_radius,
            center.1 + -angle.cos() * minute_notch_radius,
        );
        minute_marks.push((start_pos, end_pos));
    }

    /*
    let window = speedy2d::Window::new_centered(
        "Clock",
        (
            ((radius + margin) * 2.) as u32,
            ((radius + margin) * 2.) as u32,
        ),
    )
    .unwrap();
    window.run_loop(ClockData {
        center,
        hour_texts,
        hours_len,
        minutes_len,
        seconds_len,
        radius,
        minute_marks,
    })
    */
    // To develop a WASM application, you must have WASM target installed. You can install it using this command:
    // rustup target add wasm32-unknown-unknown

    // To compile for this target, you must use this command:
    // cargo build --target wasm32-unknown-unknown

    //cargo install wasm-bindgen-cli

    //wasm-bindgen target/wasm32-unknown-unknown/debug/clock.wasm --out-dir target/wasm32-unknown-unknown/debug/web --target web

    //deno install --allow-net --allow-read https://deno.land/std@0.151.0/http/file_server.ts
    //file_server .
    // /*
    WebCanvas::new_for_id(
        "my_canvas",
        ClockData {
            center,
            hour_texts,
            hours_len,
            minutes_len,
            seconds_len,
            radius,
            minute_marks,
        },
    )
    .unwrap();
    // */
}

impl ClockData {
    fn draw_background(&self, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);
        graphics.draw_circle(self.center, self.radius, Color::from_rgb(0.7, 0.8, 1.0));
        for minute_mark in &self.minute_marks {
            graphics.draw_line(
                minute_mark.0,
                minute_mark.1,
                1.0,
                Color::from_rgb(0.4, 0.4, 0.4),
            );
        }
        for hour_text in &self.hour_texts {
            graphics.draw_text((hour_text.1 .0, hour_text.1 .1), Color::BLACK, &hour_text.0);
        }
    }

    fn draw_hands(&self, graphics: &mut Graphics2D) {
        let local_now = Local::now();
        let local_seconds =
            local_now.num_seconds_from_midnight() as f32 + local_now.nanosecond() as f32 * 1e-9;
        let hours = local_seconds / 3600. % 12.;
        let minutes = local_seconds / 60. % 60.;
        let seconds = local_seconds % 60.;

        let hours_angle = hours / 6. * PI;
        let minutes_angle = minutes / 30. * PI;
        let seconds_angle = seconds / 30. * PI;

        let seconds_vector = (
            self.seconds_len * seconds_angle.sin(),
            self.seconds_len * -seconds_angle.cos(),
        );
        graphics.draw_line(
            self.center,
            (
                self.center.0 + seconds_vector.0,
                self.center.1 + seconds_vector.1,
            ),
            2.0,
            Color::from_rgb(0.3, 0.3, 0.7),
        );

        let minutes_vector = (
            self.minutes_len * minutes_angle.sin(),
            self.minutes_len * -minutes_angle.cos(),
        );
        graphics.draw_line(
            self.center,
            (
                self.center.0 + minutes_vector.0,
                self.center.1 + minutes_vector.1,
            ),
            3.5,
            Color::from_rgb(0.3, 0.7, 0.3),
        );

        let hours_vector = (
            self.hours_len * hours_angle.sin(),
            self.hours_len * -hours_angle.cos(),
        );
        graphics.draw_line(
            self.center,
            (
                self.center.0 + hours_vector.0,
                self.center.1 + hours_vector.1,
            ),
            5.0,
            Color::from_rgb(0.7, 0.3, 0.3),
        );
    }
}
*/
