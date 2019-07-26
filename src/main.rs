extern crate image;
extern crate clap;

mod geometry;
mod svg;

use std::f64::consts::PI;

use image::*;
use clap::{Arg, App};
use geometry::Point;

fn swirl(source: &mut RgbImage) -> (f64, Vec<Point>) {
    // Output dimension
    let size = 500.0;
    // Centre-crop image to a square and resize to `size`
    let mut im: RgbImage;
    let (width, height) = source.dimensions();
    im = if width == height {
        imageops::crop(source, 0, 0, width, height).to_image()
    } else if width < height {
        imageops::crop(source, 0, ((height as f64 - width as f64) * 0.5).floor() as u32, width, width).to_image()
    } else {
        imageops::crop(source, ((width as f64 - height as f64) * 0.5).floor() as u32, 0, height, height).to_image()
    };
    im = imageops::resize(&im, size as u32, size as u32, imageops::FilterType::Nearest);

    // Keep the spiral within a 5px gutter
    let max_radius = (size * 0.5) - 5.0;

    let origin_x = size * 0.5;
    let origin_y = size * 0.5;
    let mut r;

    // The number of turns of the spiral, set arbitrarily large
    // so the spiral is larger than the source image. Note the
    // loop below will `break` before this number of turns is
    // reached hence why this is arbitrary.
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
        let p0 = Point::new(
            origin_x + r * theta.cos(),
            origin_y + r * theta.sin()
        );
        // We generate two points centered around p0 in the direction of theta,
        // one towards the center of the spiral, the other away from.
        let p1 = Point::new(
            p0.x - (sample_length * 0.5) * theta.cos(),
            p0.y - (sample_length * 0.5) * theta.sin()
        );
        let p2 = Point::new(
            p0.x + (sample_length * 0.5) * theta.cos(),
            p0.y + (sample_length * 0.5) * theta.sin()
        );
        // Get the average rgb between our two points
        let average_rgb = get_average_rgb_between_points(&im, &p1, &p2);
        let luma = average_rgb.to_luma();
        // Convert average rgb into luma and then normalise the luma to a value
        // between 0 and sample_length
        let mut length = ((255.0 - luma.data[0] as f64) / 255.0) * sample_length;
        // Make sure length is not less than 1, purely for asthetic reasons
        if length < 1.0 {
            length = 1.0;
        }
        let p1 = Point::new(
            p0.x - (length * 0.5) * theta.cos(),
            p0.y - (length * 0.5) * theta.sin()
        );
        let p2 = Point::new(
            p0.x + (length * 0.5) * theta.cos(),
            p0.y + (length * 0.5) * theta.sin()
        );
        inner.push(p1);
        outer.push(p2);
    }
    // Combine inner and outer points into a single Vec. Note we reverse the outer Vec.
    let mut points = vec!();
    for p1 in inner.drain(0..) {
        points.push(p1);
    }
    for p2 in outer.drain(0..).rev() {
        points.push(p2);
    }
    (size, points)
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
    let matches = App::new("swirlr")
        .version("1.1")
        .author("Will Dady <willdady@gmail.com>")
        .about("Creates an SVG from an input image sampling along the path of an archimedian spiral")
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
    let (size, points) = swirl(&mut source);

    let mut path = svg::Path::new();
    path.set_fill("black");
    path.d.move_to(size * 0.5, size * 0.5);

    let mut previous_point = &Point::new(size * 0.5, size * 0.5);

    for p in points.iter() {
        // Only render points with at least 2 pixels distance
        // between them for a less-complex path.
        if previous_point.dist(&p) > 2.0 {
            path.d.line_to(p.x, p.y);
            previous_point = &p;
        }
    }

    path.d.close();

    print!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {size} {size}\" width=\"{size}\" height=\"{size}\">
        {path}
    </svg>", size=size, path=path);
}
