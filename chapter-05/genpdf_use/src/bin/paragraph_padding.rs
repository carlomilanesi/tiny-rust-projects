use genpdf::{elements::Paragraph, error::Error, Element};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        doc.push(Paragraph::new("a ".repeat(120)).padded((5, 10, 20, 40)));
        doc.push(Paragraph::new("^".repeat(80)));
        Ok(())
    })
}
