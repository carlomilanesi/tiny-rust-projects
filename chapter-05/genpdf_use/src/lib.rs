/*
use genpdf::{fonts, Document};

pub fn create(
    draw_body: fn(&mut Document) -> Result<(), genpdf::error::Error>,
) -> Result<(), genpdf::error::Error> {
    let proportional_sansserif_font = fonts::from_files(
        "assets/fonts",
        "LiberationSans",
        Some(fonts::Builtin::Helvetica),
    )?;
    let mut doc = Document::new(proportional_sansserif_font);
    draw_body(&mut doc)?;
    doc.render_to_file("output.pdf")
        .expect("Failed to write PDF file");
    Ok(())
}
*/
use genpdf::{error::Error, fonts, Document};

pub fn create(draw_body: fn(&mut Document) -> Result<(), Error>) -> Result<(), Error> {
    let proportional_sansserif_font = fonts::from_files(
        "assets/fonts",
        "LiberationSans",
        Some(fonts::Builtin::Helvetica),
    )?;
    let mut doc = Document::new(proportional_sansserif_font);
    draw_body(&mut doc)?;
    doc.render_to_file("output.pdf")
        .expect("Failed to write PDF file");
    Ok(())
}
