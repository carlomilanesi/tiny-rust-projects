use genpdf::{elements::Paragraph, error::Error, SimplePageDecorator};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        let mut decorator = SimplePageDecorator::new();
        decorator.set_margins((5, 10, 20, 40));
        doc.set_page_decorator(decorator);
        doc.set_paper_size((100, 50));
        doc.push(Paragraph::new("a ".repeat(80)));
        Ok(())
    })
}
