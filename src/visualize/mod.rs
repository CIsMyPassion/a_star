use image::{Rgb, ImageBuffer};
use gif::{Frame, Encoder, Repeat};
use std::fs::File;
use std::borrow::Cow;

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

pub fn visualize_scaled_field_with_path_steps(field: &Field, path: &Path, scaler: u16, file_path: &str) {

    let image_width = field.width() as u16 * scaler;
    let image_height = field.height() as u16 * scaler;
    let step_count = path.steps().len();
    let color_map_size = if step_count + 2 > 256 {
        256
    } else {
        step_count + 2
    };

    //let color_map = &[0xFF, 0x00];
    let mut color_map: Vec<u8> = Vec::with_capacity(color_map_size * 3);
    color_map.push(0xFF);
    color_map.push(0xFF);
    color_map.push(0xFF);
    color_map.push(0x00);
    color_map.push(0x00);
    color_map.push(0x00);

    for i in 0..color_map_size - 2 {
        let progress = i as f64 / (color_map_size - 2) as f64;
        let r_channel = ((1.0 - progress) * 255.0) as u8;
        let g_channel = (progress * 255.0) as u8;

        color_map.push(r_channel);
        color_map.push(g_channel);
        color_map.push(0x00);
    }

    println!("Color map len {}", color_map.len());

    let mut image_file = File::create(file_path).unwrap();
    let mut encoder = Encoder::new(&mut image_file, image_width, image_height, &color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    let mut field_map = create_color_map_from_field(&field, scaler);

    for step in 0..step_count {
        let mut frame = Frame::default();
        frame.width = image_width;
        frame.height = image_height;

        let start_x = path.steps()[step].x() as u16 * scaler;
        let start_y = path.steps()[step].y() as u16 * scaler;
        let end_x = start_x + scaler;
        let end_y = start_y + scaler;

        let step_to_color_index = remap_value(step as f64, 0.0, step_count as f64, 2.0, color_map_size as f64);

        for y in start_y..end_y {
            for x in start_x..end_x {
                let total_index = x as usize + y as usize * image_width as usize;
                field_map[total_index] = step_to_color_index;
            }
        }

        println!("Step count {}", step);

        frame.buffer = Cow::Borrowed(&*field_map);
        encoder.write_frame(&frame).unwrap();
    }
}

fn remap_value(value: f64, low_1: f64, high_1: f64, low_2: f64, high_2: f64) -> u8 {
    (low_2 + (value - low_1) * (high_2 - low_2) / (high_1 - low_1)) as u8
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

fn create_color_map_from_field(field: &Field, scaler: u16) -> Vec<u8> {

    let scaled_width = field.width() * scaler as usize;
    let scaled_height = field.height() * scaler as usize;

    let mut color_field = vec![0; scaled_width * scaled_height];

    for y in 0..field.height() {
        for x in 0..field.width() {
            if field.get_tile(&Point::new(x, y)) == Tile::Wall {
                for iy in 0..scaler {
                    for ix in 0..scaler {
                        let total_x = x * scaler as usize + ix as usize;
                        let total_y = y * scaler as usize + iy as usize;
                        let total_index = total_x + total_y * scaled_height;
                        color_field[total_index] = 1;
                    }
                }
            }
        }
    }

    color_field
}
