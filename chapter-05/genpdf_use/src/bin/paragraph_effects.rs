use genpdf::{elements::Paragraph, error::Error, style::Effect, Element};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        doc.push(Paragraph::new("Hello, regular!"));
        doc.push(Paragraph::new("Hello, bold!").styled(Effect::Bold));
        doc.push(Paragraph::new("Hello, italic!").styled(Effect::Italic));
        doc.push(
            Paragraph::new("Hello, bold italic!")
                .styled(Effect::Bold)
                .styled(Effect::Italic),
        );
        Ok(())
    })
}
