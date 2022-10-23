use genpdf::{elements::Paragraph, error::Error, style::Color::Rgb, style::Style, Element};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        doc.push(Paragraph::new("Hello, light green").styled(Style::from(Rgb(0, 200, 0))));
        doc.push(Paragraph::new("Hello, dark blue").styled(Style::from(Rgb(0, 0, 128))));
        Ok(())
    })
}
