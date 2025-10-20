//! `rael_img` is a utility crate for loading and processing images for use with the `rael` terminal rendering engine.
//! It provides a function to load an image from a file, optionally resize it, and convert its pixels
//! into a format suitable for `rael`'s `Canvas`.

use image::{DynamicImage, GenericImageView};
use rael::Color;

/// Loads an image from the given path, processes it according to the specified parameters,
/// and returns a list of (X, Y, color) tuples. These tuples represent the pixels of the
/// processed image, ready to be drawn onto a `rael::Canvas`.
///
/// This function handles image loading, optional resizing (with or without stretching),
/// and provides the pixel data with an applied position offset.
///
/// # Arguments
///
/// * `path` - The file path to the image to be loaded. Supported formats depend on the
///            `image` crate's features enabled (e.g., `webp`, `png`, `jpeg`).
/// * `width` - An `Option<u32>` specifying the target width for the image. If `None`,
///             the original image's width (scaled by `scale`) is used.
/// * `height` - An `Option<u32>` specifying the target height for the image. If `None`,
///              the original image's height (scaled by `scale`) is used.
/// * `position` - A tuple `(u32, u32)` representing the `(x, y)` offset to apply to each
///                pixel's coordinates. This effectively positions the image on the canvas.
/// * `stretch` - A boolean. If `true` and both `width` and `height` are provided, the image
///               will be stretched to exactly match the `target_width` and `target_height`.
///               If `false`, the image will be resized while maintaining its aspect ratio,
///               fitting within the `target_width` and `target_height`.
/// * `scale` - A `f32` value representing a scaling factor. If `width` or `height` are `None`,
///             the original dimensions are multiplied by this factor. If `width` and `height`
///             are provided, this factor is applied to them before resizing.
///
/// # Returns
///
/// A `Result` which is:
/// - `Ok(Vec<(u32, u32, rael::Color)>)`: A vector of tuples, where each tuple contains
///   the `(x, y)` coordinate (offset by `position`) and the `rael::Color` of a pixel.
/// - `Err(image::ImageError)`: If there was an error loading or processing the image.
///
/// # Examples
///
/// ```no_run
/// use rael_img::load_image;
/// use rael::Color;
///
/// let image_pixels = load_image(
///     "./assets/my_image.png",
///     Some(50), // Target width of 50
///     None,     // Auto-calculate height to maintain aspect ratio
///     (10, 5),  // Position offset (x=10, y=5)
///     false,    // Do not stretch
///     1.0,      // No additional scaling
/// ).unwrap();
///
/// // These pixels can then be drawn onto a rael::Canvas
/// // for (x, y, color) in image_pixels {
/// //     canvas.set_pixel(x as usize, y as usize, 1, color);
/// // }
/// ```
pub fn load_image(
    path: &str,
    width: Option<u32>,
    height: Option<u32>,
    position: (u32, u32),
    stretch: bool,
    scale: f32,
) -> Result<Vec<(u32, u32, Color)>, image::ImageError> {
    let img = image::open(path)?;
    let (img_width, img_height) = img.dimensions();

    let target_width = width.unwrap_or((img_width as f32 * scale) as u32);
    let target_height = height.unwrap_or((img_height as f32 * scale) as u32);

    let final_img: DynamicImage;

    if target_width == img_width && target_height == img_height {
        final_img = img;
    } else if stretch && width.is_some() && height.is_some() {
        final_img = img.resize_exact(
            target_width,
            target_height,
            image::imageops::FilterType::Triangle,
        );
    } else {
        final_img = img.resize(
            target_width,
            target_height,
            image::imageops::FilterType::Triangle,
        );
    }

    let mut pixels = Vec::new();
    for (x, y, pixel) in final_img.pixels() {
        let color = Color {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
        };
        pixels.push((x + position.0, y + position.1, color));
    }

    Ok(pixels)
}