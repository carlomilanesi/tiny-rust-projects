use genpdf::{elements::Paragraph, error::Error, fonts, style::Style, Element};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        let monospace_font = fonts::from_files(
            "assets/fonts/",
            "LiberationSans",
            Some(fonts::Builtin::Courier),
        )?;
        let monospace_family = doc.add_font_family(monospace_font);
        doc.push(Paragraph::new("Hello, world!"));
        doc.push(Paragraph::new("Hello, code!").styled(Style::from(monospace_family)));
        Ok(())
    })
}
