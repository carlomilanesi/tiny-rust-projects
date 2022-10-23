use genpdf::{elements::Paragraph, error::Error, Alignment};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        doc.push(Paragraph::new("Hello at left").aligned(Alignment::Left));
        doc.push(Paragraph::new("Hello at center").aligned(Alignment::Center));
        doc.push(Paragraph::new("Hello at right").aligned(Alignment::Right));
        doc.set_paper_size((100, 25));
        Ok(())
    })
}
