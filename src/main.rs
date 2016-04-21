extern crate image;

use std::fs::File;
use std::path::Path;

use image::{
    GenericImage,
};

mod util;
use util::model::Model;

const COLOR_RED: [u8; 4] = [255, 0, 0, 0];
const COLOR_GREEN: [u8; 4] = [0, 255, 0, 0];
const COLOR_BLUE: [u8; 4] = [0, 0, 255, 0];

const IMAGE_WIDTH: u32 = 800;
const IMAGE_HEIGHT: u32 = 600;

fn main() {
    let mut img = image::DynamicImage::new_rgb8(IMAGE_WIDTH, IMAGE_HEIGHT);

    let model = Model::new("models/african_head.obj");

    for i in 0..model.nfaces() {
        let face = model.face(i);
        for j in 0..3 {
            //println!("{:?}", (j + 1) % 3);
            let v0 = model.vert(face[j] as usize);
            // Deze pakt de twee in plaats van de derde? klopt iets niet
            let v1 = model.vert(face[(j + 1) % 3] as usize);
            let x0 = (v0.x + 1.0) * IMAGE_WIDTH as f64 / 2.0;
            let y0 = (v0.y + 1.0) * IMAGE_HEIGHT as f64 / 2.0;
            let x1 = (v1.x + 1.0) * IMAGE_WIDTH as f64 / 2.0;
            let y1 = (v1.y + 1.0) * IMAGE_HEIGHT as f64 / 2.0;

            draw_line(&mut img, x0 as i32, y0 as i32, x1 as i32, y1 as i32, COLOR_GREEN);
        }
    }

    let ref mut fout = File::create(&Path::new("test.png")).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    let _ = img.save(fout, image::PNG);
}

fn draw_line(i: &mut image::DynamicImage, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, color: [u8; 4]) {
    let mut steep = false;
    if (x0 - x1).abs() < (y0-y1).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;

    let derror2 = (dy * 2).abs();
    let mut error2 = 0;

    let mut y = y0;

    for x in x0..x1 + 1 {
        if steep {
            draw_pixel(i, y, x, color);
        } else {
            draw_pixel(i, x, y, color);
        }

        error2 += derror2;
        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}

fn draw_pixel(i: &mut image::DynamicImage, x: i32, y: i32, color: [u8; 4])
{
    let y = IMAGE_HEIGHT as i32 - y;

    if x < 0 || x >= IMAGE_WIDTH as i32 || y < 0 || y >= IMAGE_HEIGHT as i32 {
        return;
    }

    i.put_pixel(x as u32, y as u32, image::Rgba(color));
}
