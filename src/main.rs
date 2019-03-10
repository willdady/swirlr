extern crate image;
extern crate rand;

use rand::prelude::*;

#[allow(unused_imports)]
use image::*;
use std::f64::consts::PI;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn dist(&self, point: &Point) -> f64 {
        ((point.x - self.x).powi(2) + (point.y - self.y).powi(2)).sqrt()
    }

    fn angle(&self, point: &Point) -> f64 {
        let delta_x = point.x - self.x;
        let delta_y = point.y - self.y;
        delta_y.atan2(delta_x)
    }
}

fn render(source: &mut RgbImage) -> RgbImage {
    let red = Rgb([255, 0, 0]);
    let blue = Rgb([0, 0, 255]);
    let white = Rgb([255, 255, 255]);
    let black = Rgb([0, 0, 0]);

    let sample_length = 1.5;

    let size = source.width();

    let mut output = RgbImage::new(size, size);
    fill_image(&mut output, white);

    let output_width = size as f64;
    let output_height = size as f64;
    let origin_x = (size / 2) as f64;
    let origin_y = (size / 2) as f64;
    let iterations = 1000000;
    let mut x;
    let mut y;
    let mut r;
    let mut theta;

    let a = 0.0;
    let b = 1.0;
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
        let p0 = Point{x, y};
        let p1 = Point{
            x: p0.x - sample_length * theta.cos(),
            y: p0.y - sample_length * theta.sin()
        };
        let p2 = Point{
            x: p0.x + sample_length * theta.cos(),
            y: p0.y + sample_length * theta.sin()
        };

        let dist = p1.dist(&p2) as u32;
        let mut pixels = vec!();
        for i in 0..dist {
            let sx = (p1.x + i as f64 * theta.cos()).round() as u32;
            let sy = (p1.y + i as f64 * theta.sin()).round() as u32;
            if sx > size - 1 || sy > size - 1 {
                continue;
            }
            // println!("Put {}, {}", sx, sy);
            let pixel = source.get_pixel(sx, sy);
            pixels.push(pixel);
        };
        let average_rgb = get_average_rgb(&pixels);
        let luma = average_rgb.to_luma();

        let mut dist2 = ((255.0 - luma.data[0] as f64) / 255.0) * (sample_length * 2.0);
        if dist2 < 1.0 {
            dist2 = 1.0;
        }
        let p1 = Point{
            x: p0.x - dist2 * theta.cos(),
            y: p0.y - dist2 * theta.sin()
        };
        let p2 = Point{
            x: p0.x + dist2 * theta.cos(),
            y: p0.y + dist2 * theta.sin()
        };
        draw_line(&mut output, &p1, &p2, black);
    }
    output
}

fn get_average_rgb(pixels: &Vec<&Rgb<u8>>) -> Rgb<u8> {
    let mut r: f64 = 0.0;
    let mut g: f64 = 0.0;
    let mut b: f64 = 0.0;
    let mut count = 0.0;
    for i in pixels {
        r += i.data[0] as f64;
        g += i.data[1] as f64;
        b += i.data[2] as f64;
        count += 1.0;
    }
    r = r / count;
    g = g / count;
    b = b / count;
    return Rgb([r.round() as u8, g.round() as u8, b.round() as u8])
}

fn fill_image(image: &mut RgbImage, color: Rgb<u8>) {
    let (width, height) = image.dimensions();
    for y in 0..height {
        for x in 0..width {
            image.put_pixel(x, y, color);
        }
    }
}

fn draw_line(target: &mut RgbImage, from: &Point, to: &Point, color: Rgb<u8>) {
    let dist = from.dist(&to) as u32;
    let (width, height) = target.dimensions();
    if dist == 0 {
        return;
    }
    let angle = from.angle(to);
    for i in 0..dist {
        let x = (from.x + i as f64 * angle.cos()).round() as u32;
        let y = (from.y + i as f64 * angle.sin()).round() as u32;
        // println!("Put {}, {}", x, y);
        if x > width - 1 || y > height - 1 {
            continue;
        }
        target.put_pixel(x, y, color);
    }
}

fn main() {
    let mut norma = image::open("norma.jpg").unwrap().to_rgb();
    let output = render(&mut norma);
    // // let output = render(&norma);
    // let p1 = Point{x: 0.0, y: 0.0};
    // let p2 = Point{x: 50.0, y: 50.0};
    // draw_line(&mut norma, &p1, &p2);
    output.save("output.png").unwrap();
}
