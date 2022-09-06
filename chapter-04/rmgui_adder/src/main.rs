use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Entry, Grid, Label};
use std::rc::Rc;

fn main() {
    let app = Application::new(None, Default::default());
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_resizable(false);
        window.set_title("Adder");

        let addend1 = Rc::new(Entry::new());
        addend1.set_margin(4);
        addend1.set_alignment(1.);
        addend1.set_max_length(18);

        let addend2 = Rc::new(Entry::new());
        addend2.set_margin(4);
        addend2.set_alignment(1.);
        addend2.set_max_length(18);

        let sum = Rc::new(Entry::new());
        sum.set_margin(4);
        sum.set_alignment(1.);
        sum.set_sensitive(false);
        sum.set_widget_name("sum");

        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(b"#sum { color: darkmagenta; }")
            .unwrap();
        gtk::StyleContext::add_provider(
            &sum.style_context(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let grid = Grid::new();

        let addend1_label = Label::new(Some("Addend 1:"));
        addend1_label.set_margin(4);
        addend1_label.set_halign(gtk::Align::Start);
        grid.attach(&addend1_label, 0, 0, 1, 1);

        let addend2_label = Label::new(Some("Addend 2:"));
        addend2_label.set_margin(4);
        addend2_label.set_halign(gtk::Align::Start);
        grid.attach(&addend2_label, 0, 1, 1, 1);

        let sum_label = Label::new(Some("Sum:"));
        sum_label.set_margin(4);
        sum_label.set_halign(gtk::Align::Start);
        grid.attach(&sum_label, 0, 2, 1, 1);

        grid.attach(addend1.as_ref(), 1, 0, 1, 1);
        grid.attach(addend2.as_ref(), 1, 1, 1, 1);
        grid.attach(sum.as_ref(), 1, 2, 1, 1);
        window.add(&grid);

        fn compute_sum(a1: &Entry, a2: &Entry, s: &Entry) {
            let mut new_sum = "---".to_string();
            if let Ok(addend_1) = a1.text().parse::<u64>() {
                if let Ok(addend_2) = a2.text().parse::<u64>() {
                    new_sum = (addend_1 + addend_2).to_string();
                }
            }
            s.set_text(&new_sum);
        }

        compute_sum(&addend1, &addend2, &sum);

        let addend2_2 = addend2.clone();
        let sum_2 = sum.clone();
        addend1.connect_changed(move |a1| {
            compute_sum(&a1, &addend2, &sum);
        });
        addend2_2.connect_changed(move |a2| {
            compute_sum(&addend1, &a2, &sum_2);
        });
        window.show_all();
    });
    app.run();
}
