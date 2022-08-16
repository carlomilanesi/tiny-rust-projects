use cursive::theme::{BaseColor, Color, ColorStyle, ColorType};
use cursive::traits::*;
use cursive::views::{Dialog, EditView, LinearLayout, TextView};
use cursive::Cursive;

fn compute_sum(ui: &mut Cursive) {
    let a1 = ui
        .call_on_name("addend1", |view: &mut EditView| {
            view.get_content().parse::<f64>()
        })
        .unwrap();
    let a2 = ui
        .call_on_name("addend2", |view: &mut EditView| {
            view.get_content().parse::<f64>()
        })
        .unwrap();
    ui.call_on_name("sum", |view: &mut TextView| {
        view.set_content(if let (Ok(a1), Ok(a2)) = (a1, a2) {
            format!("{}", a1 + a2)
        } else {
            "---".to_string()
        })
    });
}

fn main() {
    const MAX_DIGITS: usize = 20;
    let mut ui = cursive::default();
    let label1 = TextView::new("Addend 1:");
    let mut addend1 = EditView::new();
    addend1.set_max_content_width(Some(MAX_DIGITS));
    let addend1 = addend1
        .on_edit(|ui, _, _| compute_sum(ui))
        .with_name("addend1")
        .fixed_width(MAX_DIGITS);
    let label2 = TextView::new("Addend 2:");
    let mut addend2 = EditView::new();
    addend2.set_max_content_width(Some(MAX_DIGITS));
    let addend2 = addend2
        .on_edit(|ui, _, _| compute_sum(ui))
        .with_name("addend2")
        .fixed_width(MAX_DIGITS);
    let sum_label = TextView::new("Sum:     ");
    let sum = TextView::new("---")
        .style(ColorStyle {
            front: ColorType::Color(Color::Dark(BaseColor::Magenta)),
            back: ColorType::InheritParent,
        })
        .with_name("sum");
    ui.add_layer(
        Dialog::new()
            .title("Adder")
            .content(
                LinearLayout::vertical()
                    .child(LinearLayout::horizontal().child(label1).child(addend1))
                    .child(LinearLayout::horizontal().child(label2).child(addend2))
                    .child(LinearLayout::horizontal().child(sum_label).child(sum)),
            )
            .button("Close", |ui| ui.quit()),
    );
    ui.run();
}
// */
/*
fn main() {
    let mut ui = cursive::default();
    use cursive::view::{Nameable, Resizable};
    let mut addend1 = cursive::views::EditView::new();
    addend1.set_max_content_width(Some(12));
    addend1 = addend1.on_edit(|_, _, _| {});
    let addend1 = addend1.with_name("addend1");
    let addend1 = addend1.fixed_width(8);
    ui.add_layer(
        cursive::views::Dialog::new()
            .title("Adder")
            .content(
                cursive::views::LinearLayout::horizontal()
                    .child(cursive::views::TextView::new("Addend 1:"))
                    .child(addend1),
            )
            .button("Cancel", |ui| ui.quit())
            .button("Ok", |ui| ui.quit()),
    );
    ui.run();
}
*/
