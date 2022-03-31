use image::{Rgb, ImageBuffer};

use crate::{Field, Tile, Point, Path};

pub fn visualize_field(field: &Field, file_path: &str) {
    visualize_scaled_field(field, 1, file_path);
}

pub fn visualize_scaled_field(field: &Field, scaler: u32, file_path: &str) {
    let image_buffer = create_image_buffer_from_field(field, scaler);
    image_buffer.save(file_path).unwrap();
}

pub fn visualize_field_with_path(field: &Field, path: &Path, file_path: &str) {
    visualize_scaled_field_with_path(field, path, 1, file_path);
}

pub fn visualize_scaled_field_with_path(field: &Field, path: &Path, scaler: u32, file_path: &str) {
    let mut image_buffer = create_image_buffer_from_field(field, scaler);

    let path_start_color = Rgb([255 as u8, 0 as u8, 0 as u8]);
    let path_goal_color = Rgb([0 as u8, 255 as u8, 0 as u8]);

    let path_length = path.steps().len();

    for (i, step) in path.steps().iter().enumerate() {

        let progress = i as f64 / path_length as f64;
        let current_r = (path_start_color[0] as f64 * (1.0 - progress) + path_goal_color[0] as f64 * progress) as u8;
        let current_g = (path_start_color[1] as f64 * (1.0 - progress) + path_goal_color[1] as f64 * progress) as u8;
        let current_b = (path_start_color[2] as f64 * (1.0 - progress) + path_goal_color[2] as f64 * progress) as u8;

        let start_x = step.x() as u32 * scaler;
        let start_y = step.y() as u32 * scaler;

        let path_color = Rgb([current_r, current_g, current_b]);

        for y in start_y..start_y + scaler {
            for x in start_x..start_x + scaler {
                let pixel = image_buffer.get_pixel_mut(x, y);
                *pixel = path_color;
            }
        }

    }

    image_buffer.save(file_path).unwrap();
}

fn create_image_buffer_from_field(field: &Field, scaler: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image_buffer = ImageBuffer::new(field.width() as u32 * scaler, field.height() as u32 * scaler);

    let floor_color = Rgb([255 as u8, 255 as u8, 255 as u8]);
    let wall_color = Rgb([0 as u8, 0 as u8, 0 as u8]);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {

        match field.get_tile(&Point::new((x / scaler) as usize, (y / scaler) as usize)) {
            Tile::Floor => *pixel = floor_color,
            Tile::Wall => *pixel = wall_color,
        }
    }

    image_buffer
}
