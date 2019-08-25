use std::path::{Path};
use image::GenericImageView;
use image::GenericImage;

pub struct Notation {
    pub filename: String
}

impl Notation {
    pub fn test(&self) {
        println!("{:?}:", self.filename);
        let img = image::open(Path::new(&self.filename)).unwrap();
        println!("pixels:{:?}, {:?}", img.width(), img.height());
        self.gray();
    }

    pub fn gray(&self) {
        let mut img = image::open(Path::new(&self.filename)).unwrap();

        let mut img = img.to_bgra();
        img.save(Path::new("out.jpg"));
    }
}