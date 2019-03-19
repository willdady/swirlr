extern crate image;
extern crate clap;

use std::f64::consts::PI;

use image::*;
use clap::{Arg, App};

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
    let white = Rgb([255, 255, 255]);
    let black = Rgb([0, 0, 0]);

    let (width, height) = source.dimensions();
    let size = if width <= height {
        width as f64
    } else {
        height as f64
    };
    // Keep the spiral within a 5px gutter
    let max_radius = (size * 0.5) - 5.0;

    let mut output = RgbImage::new(size as u32, size as u32);
    fill_image(&mut output, white);

    let origin_x = size * 0.5;
    let origin_y = size * 0.5;
    let mut r;

    // The number of turns of the spiral, set arbitrarily large
    // so the spiral is larger than the source image. Note the
    // loop below will break before this number of turns is reached
    // hence why this is arbitrary.
    let turns = 1000.0;
    let mut theta = 0.0;
    let max_angle = turns * 2.0 * PI;

    let a = 0.0;
    let b = 1.2;
    let sample_length = 7.0;

    while theta < max_angle {
        theta += 0.003;
        r = a + b * theta;
        if r >= max_radius {
            break;
        }
        // The current point on the spiral
        let p0 = Point{
            x: origin_x + r * theta.cos(),
            y: origin_y + r * theta.sin()
        };
        // We generate two points centered around p0 in the direction of theta,
        // one towards the center of the spiral, the other away from.
        let p1 = Point{
            x: p0.x - (sample_length * 0.5) * theta.cos(),
            y: p0.y - (sample_length * 0.5) * theta.sin()
        };
        let p2 = Point{
            x: p0.x + (sample_length * 0.5) * theta.cos(),
            y: p0.y + (sample_length * 0.5) * theta.sin()
        };
        // Get the average rgb between our two points
        let average_rgb = get_average_rgb_between_points(&source, &p1, &p2);
        let luma = average_rgb.to_luma();
        // Convert average rgb into luma and then normalise the luma to a value
        // between 0 and sample_length
        let mut length = ((255.0 - luma.data[0] as f64) / 255.0) * sample_length;
        // Make sure length is not less than 1, purely for asthetic reasons
        if length < 1.0 {
            length = 1.0;
        }
        let p1 = Point{
            x: p0.x - (length * 0.5) * theta.cos(),
            y: p0.y - (length * 0.5) * theta.sin()
        };
        let p2 = Point{
            x: p0.x + (length * 0.5) * theta.cos(),
            y: p0.y + (length * 0.5) * theta.sin()
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

fn get_average_rgb_between_points(source: &RgbImage, p1: &Point, p2: &Point) -> Rgb<u8> {
    let dist = p1.dist(&p2) as u32;
    let theta = p1.angle(&p2);
    let (width, height) = source.dimensions();
    let mut pixels = vec!();
    for i in 0..dist {
        let sx = (p1.x + i as f64 * theta.cos()).round() as u32;
        let sy = (p1.y + i as f64 * theta.sin()).round() as u32;
        if sx > width - 1 || sy > height - 1 {
            continue;
        }
        let pixel = source.get_pixel(sx, sy);
        pixels.push(pixel);
    };
    get_average_rgb(&pixels)
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
        if x > width - 1 || y > height - 1 {
            continue;
        }
        target.put_pixel(x, y, color);
    }
}

fn main() {
    let matches = App::new("Swirl")
        .version("1.0")
        .author("Will Dady <willdady@gmail.com>")
        .about("Swirls an image")
        .arg(
            Arg::with_name("input")
                .index(1)
                .value_name("INPUT")
                .help("Path to input image")
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("output")
                .index(2)
                .value_name("OUTPUT")
                .help("Path to output image")
                .takes_value(true)
                .default_value("output.png")
        )
        .get_matches();

    let input_path = matches.value_of("input").unwrap();
    let output_path = matches.value_of("output").unwrap();

    let mut source = image::open(input_path).unwrap().to_rgb();
    let output = render(&mut source);
    output.save(output_path).unwrap();
}
