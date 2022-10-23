use genpdf::{
    elements::{FrameCellDecorator, Paragraph, TableLayout},
    error::Error,
    Element,
};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        let mut table = TableLayout::new(vec![1, 1]);
        table.set_cell_decorator(FrameCellDecorator::new(true, true, false));
        table
            .row()
            .element(Paragraph::new("Row 1, Col 1").padded(1))
            .element(Paragraph::new("Row 1, Col 2").padded(1))
            .push()
            .expect("Invalid table row");
        table
            .row()
            .element(Paragraph::new("Row 2, Col 1").padded(1))
            .element(Paragraph::new("Row 2, Col 2").padded(1))
            .push()
            .expect("Invalid table row");
        doc.push(table.padded(20));
        Ok(())
    })
}
