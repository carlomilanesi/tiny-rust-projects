use genpdf::{
    elements::{LinearLayout, OrderedList, Paragraph},
    error::Error,
};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        doc.push(
            OrderedList::new()
                .element(Paragraph::new("First item."))
                .element(Paragraph::new("Second item."))
                .element(
                    LinearLayout::vertical()
                        .element(Paragraph::new("Third item."))
                        .element(
                            OrderedList::new()
                                .element(Paragraph::new("First item of the third item."))
                                .element(Paragraph::new("Second item of the third item.")),
                        ),
                ),
        );
        Ok(())
    })
}
