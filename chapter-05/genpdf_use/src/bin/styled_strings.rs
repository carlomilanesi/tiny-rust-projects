use genpdf::{
    elements::Paragraph,
    error::Error,
    fonts,
    style::{Color::Rgb, Style},
};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        let monospace_font = fonts::from_files(
            "assets/fonts/",
            "LiberationSans",
            Some(fonts::Builtin::Courier),
        )?;
        let monospace_family = doc.add_font_family(monospace_font);
        doc.push(
            Paragraph::new("Hello, ")
                .styled_string(
                    "monospace, ",
                    Style::new().with_font_family(monospace_family),
                )
                .styled_string("big, ", Style::new().with_font_size(18))
                .styled_string("bold, ", Style::new().bold())
                .styled_string("or blue, ", Style::new().with_color(Rgb(0, 0, 255)))
                .string("or normal!"),
        );
        doc.push(
            Paragraph::new("Hello, ").styled_string(
                "monospace, big, bold, and blue.",
                Style::new()
                    .with_font_family(monospace_family)
                    .with_font_size(18)
                    .bold()
                    .with_color(Rgb(0, 0, 255)),
            ),
        );
        Ok(())
    })
}
