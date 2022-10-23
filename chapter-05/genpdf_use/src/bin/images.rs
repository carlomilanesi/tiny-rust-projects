use genpdf::{elements::Image, error::Error, Position};
use genpdf_use::create;

fn main() -> Result<(), Error> {
    create(|doc| {
        let path = "assets/images/rust_logo.jpg";
        doc.push(Image::from_path(path).expect("Unable to load image"));
        doc.push(
            Image::from_path(path)
                .expect("Unable to load mage")
                .with_position(Position::new(50, -40))
                .with_scale((0.5, 1.)),
        );
        Ok(())
    })
}
