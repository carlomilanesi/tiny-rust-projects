use genpdf::{elements::Paragraph, error::Error};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        doc.set_title("Use of Genpdf");
        doc.set_minimal_conformance();
        doc.set_line_spacing(2.5);
        doc.push(Paragraph::new("Hello"));
        doc.push(Paragraph::new("world!"));
        Ok(())
    })
}
