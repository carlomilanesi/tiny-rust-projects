use genpdf::{elements::Paragraph, error::Error, style::Style, Element};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        doc.push(Paragraph::new("Hello, 8 pt!").styled(Style::new().with_font_size(8)));
        doc.push(Paragraph::new("Hello, normal!"));
        doc.push(Paragraph::new("Hello, 12 pt!").styled(Style::new().with_font_size(12)));
        doc.push(Paragraph::new("Hello, 18 pt!").styled(Style::new().with_font_size(18)));
        Ok(())
    })
}
