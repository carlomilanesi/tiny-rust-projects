use genpdf::{
    elements::{BulletPoint, LinearLayout, Paragraph, UnorderedList},
    error::Error,
};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        doc.push(
            UnorderedList::new()
                .element(Paragraph::new("First item."))
                .element(Paragraph::new("Second item."))
                .element(
                    LinearLayout::vertical()
                        .element(Paragraph::new("Sub list:"))
                        .element(
                            BulletPoint::new(Paragraph::new("First item of the third item."))
                                .with_bullet("•"),
                        )
                        .element(
                            BulletPoint::new(Paragraph::new("Second item of the third item."))
                                .with_bullet("•"),
                        ),
                ),
        );
        Ok(())
    })
}
