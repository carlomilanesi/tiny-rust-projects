use genpdf::{elements::Paragraph, error::Error};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        doc.set_paper_size((120, 30));
        doc.push(Paragraph::new("a ".repeat(300)));
        Ok(())
    })
}
