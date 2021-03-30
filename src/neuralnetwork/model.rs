use image;
use image::{DynamicImage};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Image {
    pub location: String,
}

impl Image {
    pub fn load(location: &String) -> DynamicImage {
        image::open(location).expect(&format!("cannot load image from {}", location))
    }
}
