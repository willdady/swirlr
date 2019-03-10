extern crate image;
extern crate rand;

use rand::prelude::*;

#[allow(unused_imports)]
use image::*;
#[allow(unused_imports)]
use std::f64::consts::PI;

#[derive(Debug)]
struct Rect {
    x: i64,
    y: i64,
    width: i64,
    height: i64
}

// Get distance between two points
fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

fn render_brush(size: u32) -> RgbImage {
    let mut rng = rand::thread_rng();
    let mut img = DynamicImage::new_rgb8(size, size).to_rgb();
    let black = image::Rgb([0, 0, 0]);
    let white = image::Rgb([255, 255, 255]);
    // let red = image::Rgb([255, 0, 0]);
    let radius = (size / 2) as f64 - 1.0;
    let origin_x = radius + 1.0;
    let origin_y = radius + 1.0;
    const SAMPLES: f64 = 50.0;
    const SAMPLES_USIZE: usize = SAMPLES as usize;
    for y in 0..size {
        for x in 0..size {
            // Supersampling
            let mut colours: [Rgb<u8>; SAMPLES_USIZE] = [black; SAMPLES_USIZE];
            for j in 0..SAMPLES_USIZE {
                let u = x as f64 + 0.5 + rng.gen_range(-0.5, 0.5);
                let v = y as f64 + 0.5 + rng.gen_range(-0.5, 0.5);
                let d = dist(u, v, origin_x, origin_y);
                if d <= radius {
                    colours[j] = white;
                } else {
                    colours[j] = black;
                }
            }
            let mut r: f64 = 0.0;
            let mut g: f64 = 0.0;
            let mut b: f64 = 0.0;
            for k in 0..SAMPLES_USIZE {
                r += colours[k][0] as f64;
                g += colours[k][1] as f64;
                b += colours[k][2] as f64;
            }
            r = r / SAMPLES;
            g = g / SAMPLES;
            b = b / SAMPLES;
            img.put_pixel(x, y, Rgb([r.round() as u8, g.round() as u8, b.round() as u8]));
        }
    }
    return img;
}

fn render(source: &RgbImage) -> RgbImage {
    let size = source.width();
    let mut output = RgbImage::new(size, size);
    let output_width = size as f64;
    let output_height = size as f64;
    let origin_x = (size / 2) as f64;
    let origin_y = (size / 2) as f64;
    let iterations = 100000;
    let mut x;
    let mut y;
    let mut r;
    let mut theta;

    // Create brushes
    let mut brushes = Vec::new();
    for i in 0..=10 {
        brushes.push(render_brush((i + 1) * 2))
    }

    let a = 0.0;
    let b = 5.0;
    let loops = 100.0;

    let slice = 2.0 * PI / iterations as f64;

    for i in 0..iterations {
        theta = loops * i as f64 * slice;
        r = a + b * theta;
        x = origin_x + r * theta.cos();
        y = origin_y + r * theta.sin();
        if x < 0.0 || x > output_width || y < 0.0 || y > output_height {
            break;
        }
        let rect = Rect{
            x: x as i64 - 10,
            y: y as i64 - 10,
            width: 10,
            height: 10
        };
        let average_rgb = get_average_rgb(&source, &rect);
        let average_luma = average_rgb.to_luma();
        // Get brush for luma
        let l = 10 - ((average_luma.data[0] as f64 / 255.0) * 10.0).round() as usize;
        let brush = &brushes[l];
        let target_x = x as i64 - brush.width() as i64 / 2;
        let target_y = y as i64 - brush.height() as i64 / 2;
        apply_brush(&mut output, brush, target_x, target_y);
    }
    output
}

fn get_average_rgb(source: &RgbImage, rect: &Rect) -> Rgb<u8> {
    // println!("get_average_rgb {:?}", rect);
    let mut r: f64 = 0.0;
    let mut g: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut count = 0.0;
    for y in rect.y..rect.y + rect.width {
        for x in rect.x..rect.x + rect.height {
            let p = source.get_pixel(x as u32, y as u32);
            // println!("{:?}", p);
            r += p.data[0] as f64;
            g += p.data[1] as f64;
            b += p.data[2] as f64;
            count += 1.0;
        }
    }
    r = r / count;
    g = g / count;
    b = b / count;
    return Rgb([r.round() as u8, g.round() as u8, b.round() as u8])
}

fn apply_brush(target: &mut RgbImage, brush: &RgbImage, x: i64, y: i64) {
    let target_width = target.width() as i64;
    let target_height = target.height() as i64;
    let red = image::Rgb([255, 0, 0]);
    for v in 0..brush.height() {
        for u in 0..brush.width() {
            let p = brush.get_pixel(u as u32, v as u32);
            if p.data[0] != 0 {
                let target_x = x + u as i64;
                let target_y = y + v as i64;
                if target_x < 0 || target_y < 0 || target_x > target_width - 1 || target_y > target_height - 1 {
                    continue;
                }
                target.put_pixel(target_x as u32, target_y as u32, red);
            }
        }
    }
}

fn main() {
    let norma = image::open("norma.jpg").unwrap().to_rgb();
    let output = render(&norma);
    output.save("output.png").unwrap();
}
