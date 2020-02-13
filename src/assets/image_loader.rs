extern crate image;

use image::{DynamicImage, GenericImageView};

pub struct ImageAsset {
    pub width: u32,
    pub height: u32,

    pub data: Vec<u8>
}

pub fn load(file_path: &str) -> ImageAsset {
    match std::env::current_exe() {
        Ok(mut absolute_path) => {
            absolute_path.pop();

            absolute_path.push(file_path);

            match image::open(absolute_path) {
                Err(err) => panic!("Could not load image {}: {}", file_path, err),
                Ok(img) => {
                    println!("Dimensions of image are {:?}", img.dimensions());
    
                    let (width, height) = img.dimensions();

                    let img = match img {
                        DynamicImage::ImageRgba8(img) => img,
                        img => img.to_rgba()
                    };

                    return ImageAsset {
                        width: width,
                        height: height,
                        data: img.into_raw()
                    };
                }
            }

        }
        Err(e) => panic!("failed to get current exe path: {}", e)
    };
}