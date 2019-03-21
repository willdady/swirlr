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

fn swirl(source: &mut RgbImage) -> (f64, Vec<Point>, Vec<Point>) {
    let (width, height) = source.dimensions();
    let size = if width <= height {
        width as f64
    } else {
        height as f64
    };
    // Keep the spiral within a 5px gutter
    let max_radius = (size * 0.5) - 5.0;

    let origin_x = size * 0.5;
    let origin_y = size * 0.5;
    let mut r;

    // The number of turns of the spiral, set arbitrarily large
    // so the spiral is larger than the source image. Note the
    // loop below will `break` before this number of turns is reached
    // hence why this is arbitrary.
    let turns = 1000.0;
    let mut theta = 0.0;
    let max_angle = turns * 2.0 * PI;

    let a = 0.0;  // The starting radius
    let b = 1.2;  // The growth rate of the spiral through each iteration of the loop
    let sample_length = 7.0;

    let mut inner = vec!();
    let mut outer = vec!();

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
        inner.push(p1);
        outer.push(p2);
    }
    (size, inner, outer)
}

fn get_average_rgb(pixels: &Vec<&Rgb<u8>>) -> Rgb<u8> {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
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
        .get_matches();

    let input_path = matches.value_of("input").unwrap();

    let mut source = image::open(input_path).unwrap().to_rgb();
    let (size, inner, outer) = swirl(&mut source);

    let mut path = String::new();

    path.push_str(&format!("M{:1.2} {:1.2}", size * 0.5, size * 0.5));

    let mut previous_point = &Point{x: size * 0.5, y: size * 0.5};

    for p1 in inner.iter() {
        if previous_point.dist(p1) > 2.0 {
            path.push_str(&format!(" L{:1.2} {:1.2}", p1.x, p1.y));
            previous_point = p1;
        }
    }

    for p2 in outer.iter().rev() {
        if previous_point.dist(p2) > 2.0 {
            path.push_str(&format!(" L{:1.2} {:1.2}", p2.x, p2.y));
            previous_point = p2;
        }
    }

    path.push_str(" Z");

    print!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {size} {size}\" width=\"{size}\" height=\"{size}\">
        <path fill=\"black\" d=\"{path}\" />
    </svg>", size=size, path=path);
}
