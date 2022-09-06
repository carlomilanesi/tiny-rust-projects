use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::shape::Rectangle;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

struct AdderData {
    max_num_digits: usize,
    addend_1: String,
    addend_2: String,
    sum: String,
    cursor_at_1: bool,
    scale: f32,
    addend_1_label: String,
    addend_2_label: String,
    sum_label: String,
    font: Font,
}

impl AdderData {
    fn left_margin(&self) -> f32 {
        self.scale * 0.3
    }

    fn right_margin(&self) -> f32 {
        self.scale * 0.3
    }

    fn top_margin(&self) -> f32 {
        self.scale * 0.3
    }

    fn bottom_margin(&self) -> f32 {
        self.scale * 0.3
    }

    fn v_spacing(&self) -> f32 {
        self.scale * 0.3
    }

    fn left_padding(&self) -> f32 {
        self.scale * 0.2
    }

    fn right_padding(&self) -> f32 {
        self.scale * 0.2
    }

    fn top_padding(&self) -> f32 {
        self.scale * 0.1
    }

    fn bottom_padding(&self) -> f32 {
        self.scale * 0.1
    }

    fn label_field_spacing(&self) -> f32 {
        self.scale * 0.2
    }

    fn border_width(&self) -> f32 {
        self.scale / 20.
    }

    fn compute_sum(&mut self) {
        self.sum = "---".to_string();
        if let Ok(addend_1) = self.addend_1.parse::<u64>() {
            if let Ok(addend_2) = self.addend_2.parse::<u64>() {
                self.sum = (addend_1 + addend_2).to_string()
            }
        }
    }

    fn labels_width(&self) -> f32 {
        self.font
            .layout_text(&self.addend_2_label, self.scale, TextOptions::new())
            .size()
            .x
    }

    fn fields_width(&self) -> f32 {
        let digit_size = self
            .font
            .layout_text("0", self.scale, TextOptions::new())
            .size();
        self.left_padding() + self.right_padding() + self.max_num_digits as f32 * digit_size.x
    }

    fn compute_size(&self) -> (u32, u32) {
        let digit_size = self
            .font
            .layout_text("0", self.scale, TextOptions::new())
            .size();

        // Compute window width
        let dx = self.left_margin()
            + self.labels_width()
            + self.label_field_spacing()
            + self.fields_width()
            + self.right_margin();

        // Compute window height
        let dy = self.top_margin()
            + self.bottom_margin()
            + self.v_spacing() * 2.
            + (self.top_padding() + self.bottom_padding() + digit_size.y) * 3.;

        (dx as u32, dy as u32)
    }
}

impl WindowHandler for AdderData {
    fn on_start(
        &mut self,
        helper: &mut WindowHelper<()>,
        _info: speedy2d::window::WindowStartupInfo,
    ) {
        helper.set_resizable(false);
        self.compute_sum();
    }

    fn on_keyboard_char(&mut self, helper: &mut WindowHelper, ch: char) {
        match ch {
            // Backspace
            '\x08' => {
                if self.cursor_at_1 {
                    if self.addend_1.len() > 0 {
                        self.addend_1.pop();
                    }
                } else {
                    if self.addend_2.len() > 0 {
                        self.addend_2.pop();
                    }
                }
                self.compute_sum();
                helper.request_redraw();
            }
            // Tab
            '\t' => {
                self.cursor_at_1 = !self.cursor_at_1;
                helper.request_redraw();
            }
            // Escape
            '\x1b' => {
                helper.terminate_loop();
            }
            // Digit
            '0'..='9' => {
                if self.cursor_at_1 {
                    if self.addend_1.len() < self.max_num_digits {
                        self.addend_1.push(ch);
                    }
                } else {
                    if self.addend_2.len() < self.max_num_digits {
                        self.addend_2.push(ch);
                    }
                }
                self.compute_sum();
                helper.request_redraw();
            }
            _ => {}
        }
    }

    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::LIGHT_GRAY);
        let labels_fore_color = Color::from_rgb(0.2, 0.2, 0.4);
        let fields_fore_color = Color::from_rgb(0.1, 0.1, 0.2);
        let computed_fore_color = Color::from_rgb(0.4, 0.0, 0.4);
        let fields_back_color = Color::from_rgb(1., 1., 1.);
        let fields_border_color = Color::BLACK;
        let caret_color = Color::from_rgb(0.0, 0.0, 0.3);
        let fields_left = self.left_margin() + self.labels_width() + self.label_field_spacing();
        let fields_width = self.fields_width();

        // Draw addend 1
        graphics.draw_text(
            (self.left_margin(), self.top_margin() + self.top_padding()),
            labels_fore_color,
            &self
                .font
                .layout_text(&self.addend_1_label, self.scale, TextOptions::new()),
        );
        graphics.draw_rectangle(
            Rectangle::new(
                Vector2::new(fields_left, self.top_margin()),
                Vector2::new(
                    fields_left + fields_width,
                    self.top_margin() + self.top_padding() + self.bottom_padding() + self.scale,
                ),
            ),
            fields_border_color,
        );
        graphics.draw_rectangle(
            Rectangle::new(
                Vector2::new(
                    fields_left + self.border_width(),
                    self.top_margin() + self.border_width(),
                ),
                Vector2::new(
                    fields_left + fields_width - self.border_width(),
                    self.top_margin() + self.top_padding() + self.bottom_padding() + self.scale
                        - self.border_width(),
                ),
            ),
            fields_back_color,
        );
        let addend1_text = self
            .font
            .layout_text(&self.addend_1, self.scale, TextOptions::new());
        let addend1_text_size = addend1_text.size();
        graphics.draw_text(
            (
                fields_left + fields_width - self.left_padding() - addend1_text_size.x,
                self.top_margin() + self.top_padding(),
            ),
            fields_fore_color,
            &addend1_text,
        );

        // Draw addend 2
        graphics.draw_text(
            (
                self.left_margin(),
                self.top_margin()
                    + self.v_spacing()
                    + self.top_padding() * 2.
                    + self.bottom_padding()
                    + self.scale,
            ),
            labels_fore_color,
            &self
                .font
                .layout_text(&self.addend_2_label, self.scale, TextOptions::new()),
        );
        graphics.draw_rectangle(
            Rectangle::new(
                Vector2::new(
                    fields_left,
                    self.top_margin()
                        + self.v_spacing()
                        + self.top_padding()
                        + self.bottom_padding()
                        + self.scale,
                ),
                Vector2::new(
                    fields_left + fields_width,
                    self.top_margin()
                        + self.v_spacing()
                        + self.top_padding() * 2.
                        + self.bottom_padding() * 2.
                        + self.scale * 2.,
                ),
            ),
            fields_border_color,
        );
        graphics.draw_rectangle(
            Rectangle::new(
                Vector2::new(
                    fields_left + self.border_width(),
                    self.top_margin()
                        + self.v_spacing()
                        + self.top_padding()
                        + self.bottom_padding()
                        + self.scale
                        + self.border_width(),
                ),
                Vector2::new(
                    fields_left + fields_width - self.border_width(),
                    self.top_margin()
                        + self.v_spacing()
                        + self.top_padding() * 2.
                        + self.bottom_padding() * 2.
                        + self.scale * 2.
                        - self.border_width(),
                ),
            ),
            fields_back_color,
        );
        let addend2_text = self
            .font
            .layout_text(&self.addend_2, self.scale, TextOptions::new());
        let addend2_text_size = addend2_text.size();
        graphics.draw_text(
            (
                fields_left + fields_width - self.left_padding() - addend2_text_size.x,
                self.top_margin()
                    + self.v_spacing()
                    + self.top_padding() * 2.
                    + self.bottom_padding()
                    + self.scale,
            ),
            fields_fore_color,
            &addend2_text,
        );

        // Draw sum
        graphics.draw_text(
            (
                self.left_margin(),
                self.top_margin()
                    + self.v_spacing() * 2.
                    + self.top_padding() * 3.
                    + self.bottom_padding() * 2.
                    + self.scale * 2.,
            ),
            labels_fore_color,
            &self
                .font
                .layout_text(&self.sum_label, self.scale, TextOptions::new()),
        );
        let sum_text = self
            .font
            .layout_text(&self.sum, self.scale, TextOptions::new());
        let sum_text_size = sum_text.size();
        graphics.draw_text(
            (
                fields_left + fields_width - self.left_padding() - sum_text_size.x,
                self.top_margin()
                    + self.v_spacing() * 2.
                    + self.top_padding() * 3.
                    + self.bottom_padding() * 2.
                    + self.scale * 2.,
            ),
            computed_fore_color,
            &sum_text,
        );

        // Draw caret
        let caret_top = if self.cursor_at_1 {
            self.top_margin() + self.top_padding()
        } else {
            self.top_margin()
                + self.v_spacing()
                + self.top_padding() * 2.
                + self.bottom_padding()
                + self.scale
        };
        graphics.draw_line(
            Vector2::new(fields_left + fields_width - self.left_padding(), caret_top),
            Vector2::new(
                fields_left + fields_width - self.left_padding(),
                caret_top + self.scale,
            ),
            self.scale / 20.,
            caret_color,
        );
    }
}

fn main() {
    let adder_data = AdderData {
        max_num_digits: 18,
        addend_1: "".to_string(),
        addend_2: "".to_string(),
        sum: "".to_string(),
        cursor_at_1: true,
        scale: 20.,
        addend_1_label: "Addend 1:".to_string(),
        addend_2_label: "Addend 2:".to_string(),
        sum_label: "Sum:".to_string(),
        font: Font::new(include_bytes!("../assets/LiberationSans-Regular.ttf")).unwrap(),
    };
    let window = Window::new_centered("Adder", adder_data.compute_size()).unwrap();
    window.run_loop(adder_data);
}
