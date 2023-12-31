extern crate image;

use image::{imageops::FilterType, DynamicImage, GrayImage};

const SIZE: u32 = 8;

#[inline]
pub fn from_file(path: &str) -> u64 {
    calculate_dhash(&resizer(&load_img(path)))
}

#[inline]
pub fn calculate_distance(hash1: u64, hash2: u64) -> u32 {
    (hash1 ^ hash2).count_ones()
}

#[inline]
fn calculate_dhash(img: &GrayImage) -> u64 {
    let mut hash = 0u64;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let current_pixel = img.get_pixel(x, y)[0];
            let next_pixel = img.get_pixel(x + 1, y)[0];

            if current_pixel < next_pixel {
                let bit_position = y * SIZE + x;
                hash |= 1 << (63 - bit_position);
            }
        }
    }

    hash
}

#[inline]
fn resizer(img: &DynamicImage) -> GrayImage {
    let resized = img.resize_exact(SIZE + 1, SIZE, FilterType::Lanczos3);
    image::imageops::grayscale(&resized)
}

#[inline]
fn load_img(path: &str) -> DynamicImage {
    let img = image::open(path);

    match img {
        Ok(i) => i,
        Err(e) => panic!("{}", e),
    }
}
