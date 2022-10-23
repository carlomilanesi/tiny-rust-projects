use genpdf::{elements::Paragraph, error::Error};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        doc.push(Paragraph::new("Hello, world!"));
        Ok(())
    })
}
