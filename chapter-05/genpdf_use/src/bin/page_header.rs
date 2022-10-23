use genpdf::{
    elements::{Break, LinearLayout, Paragraph},
    error::Error,
    SimplePageDecorator,
};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        let mut decorator = SimplePageDecorator::new();
        decorator.set_margins(5);
        decorator.set_header(|page| {
            let mut header = LinearLayout::vertical();
            if page > 1 {
                header.push(Paragraph::new(format!("Page {page}")));
                header.push(Break::new(0.6));
            }
            header
        });
        doc.set_page_decorator(decorator);
        doc.set_paper_size((120, 30));
        doc.push(Paragraph::new("a ".repeat(200)));
        Ok(())
    })
}
