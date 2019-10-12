use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel};

use std::fs;
use std::path::Path;

fn images_from_dir(dir: &Path) -> Option<Vec<(String, DynamicImage)>> {
    if dir.is_dir() {
        let images = fs::read_dir(dir)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();
                let file_name = path
                    .file_name()
                    .and_then(|os_str| os_str.to_str())
                    .map(|s| s.to_string())
                    .unwrap();
                if !path.is_dir() {
                    println!("Loading image: {:?}", path);
                    image::open(path).ok().map(|img| (file_name, img))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, DynamicImage)>>();

        return Some(images);
    } else {
        return None;
    }
}

fn main() {
    //let img = image::open("img/circadia-art-full-512.jpg").unwrap();

    let images = images_from_dir(Path::new("img/moodboard-scaled")).unwrap();

    for (file_name, image) in images {
        let dest = format!("img/out/{}", file_name);
        println!("Saving an image: {}", file_name);
        tv_image(700, image).save(dest).unwrap();
    }
}

fn tv_image(width: u32, src: DynamicImage) -> DynamicImage {
    let src_width = src.width();
    let src_height = src.height();

    let height = width * src_height / src_width;

    let width_multiplier = src_width as f32 / width as f32;
    let height_multiplier = src_height as f32 / height as f32;

    let tv_px_height = 5;
    let tv_px_width = 7;

    let img = ImageBuffer::from_fn(width, height, |x, y| {
        // get the pixel in the src image that we should look at
        let mut src_x = (x as f32 * width_multiplier) as u32;
        let mut src_y = (y as f32 * height_multiplier) as u32;

        src_x = src_x - (src_x % tv_px_width);
        src_y = src_y - (src_y % tv_px_height);

        let pixel = src.get_pixel(src_x, src_y).to_rgb();

        if x % tv_px_width == 0 {
            image::Rgb([0, 0, 0])
        } else if y % tv_px_height == 0 {
            let image::Rgb([r, _, _]) = pixel;
            image::Rgb([r, 0, 0])
        } else if y % tv_px_height == 1 {
            let image::Rgb([_, g, _]) = pixel;
            image::Rgb([0, g, 0])
        } else if y % tv_px_height == 2 {
            let image::Rgb([_, _, b]) = pixel;
            image::Rgb([0, 0, b])
        } else {
            image::Rgb([0, 0, 0])
        }
    });
    let dynamic_img = DynamicImage::ImageRgb8(img);

    dynamic_img.brighten(15).adjust_contrast(15.0)
}
