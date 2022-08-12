extern crate image;

use clap::ArgEnum;
use image::*;

use crate::geometry::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Crop {
    Contain,
    Overflow,
}

pub struct Swirlr<'a> {
    pub crop: Crop,
    pub origin_x: f64,
    pub origin_y: f64,
    pub source: &'a mut RgbImage,
    pub output_size: f64,
    pub growth_rate: f64,
    pub invert: bool,
}

impl<'a> Swirlr<'a> {
    pub fn new(source: &'a mut RgbImage) -> Swirlr {
        Swirlr {
            crop: Crop::Overflow,
            source,
            output_size: 500.0,
            origin_x: 250.0,
            origin_y: 250.0,
            growth_rate: 1.2,
            invert: false,
        }
    }

    pub fn set_crop(mut self, crop: Crop) -> Swirlr<'a> {
        self.crop = crop;
        self
    }

    pub fn set_origin(mut self, x: f64, y: f64) -> Swirlr<'a> {
        self.origin_x = x;
        self.origin_y = y;
        self
    }

    pub fn set_growth_rate(mut self, growth_rate: f64) -> Swirlr<'a> {
        self.growth_rate = growth_rate;
        self
    }

    pub fn set_invert(mut self, invert: bool) -> Swirlr<'a> {
        self.invert = invert;
        self
    }

    pub fn get_points(&mut self) -> (f64, Vec<Point>) {
        // Output dimension
        let size = 500.0;
        // Centre-crop image to a square and resize to `size`
        let mut im: RgbImage;
        let (width, height) = &self.source.dimensions();
        im = if width == height {
            imageops::crop(self.source, 0, 0, *width, *height).to_image()
        } else if width < height {
            imageops::crop(
                self.source,
                0,
                ((*height as f64 - *width as f64) * 0.5).floor() as u32,
                *width,
                *width,
            )
            .to_image()
        } else {
            imageops::crop(
                self.source,
                ((*width as f64 - *height as f64) * 0.5).floor() as u32,
                0,
                *height,
                *height,
            )
            .to_image()
        };
        im = imageops::resize(&im, size as u32, size as u32, imageops::FilterType::Nearest);

        let crop: f64 = match self.crop {
            Crop::Contain => (size * 0.5) - 5.0,
            Crop::Overflow => {
                let tl = Point::new(0.0, 0.0);
                let tr = Point::new(self.output_size, 0.0);
                let br = Point::new(self.output_size, self.output_size);
                let bl = Point::new(0.0, self.output_size);
                let origin: Point = (self.origin_x, self.origin_y).into();

                [
                    origin.dist(&tl),
                    origin.dist(&tr),
                    origin.dist(&br),
                    origin.dist(&bl),
                ]
                .iter()
                .fold(f64::NEG_INFINITY, |prev, curr| prev.max(*curr))
            }
        };

        let mut r;

        let mut theta: f64 = 0.0;

        let a = 0.0; // The starting radius
        let b = self.growth_rate; // The growth rate of the spiral through each iteration of the loop
        let sample_length = 7.0; // Controls line thickness

        let mut inner = vec![];
        let mut outer = vec![];

        // while theta < max_angle {
        loop {
            theta += 0.003;
            r = a + b * theta;
            if r >= crop {
                break;
            }
            // The current point on the spiral
            let p0 = Point::new(
                self.origin_x + r * theta.cos(),
                self.origin_y + r * theta.sin(),
            );
            // We generate two points centered around p0 in the direction of theta,
            // one towards the center of the spiral, the other away from.
            let p1 = Point::new(
                p0.x - (sample_length * 0.5) * theta.cos(),
                p0.y - (sample_length * 0.5) * theta.sin(),
            );
            let p2 = Point::new(
                p0.x + (sample_length * 0.5) * theta.cos(),
                p0.y + (sample_length * 0.5) * theta.sin(),
            );
            // Get the average rgb between our two points
            let average_rgb = get_average_rgb_between_points(&im, &p1, &p2);
            let luma = average_rgb.to_luma();
            // Convert average rgb into luma and then normalise the luma to a value
            // between 0 and sample_length
            let mut length = ((255.0 - luma[0] as f64) / 255.0) * sample_length;

            if self.invert {
                length = sample_length - length;
            }

            // Make sure length is not less than 1, purely for asthetic reasons
            if length < 1.0 {
                length = 1.0;
            }
            let p1 = Point::new(
                p0.x - (length * 0.5) * theta.cos(),
                p0.y - (length * 0.5) * theta.sin(),
            );
            let p2 = Point::new(
                p0.x + (length * 0.5) * theta.cos(),
                p0.y + (length * 0.5) * theta.sin(),
            );
            inner.push(p1);
            outer.push(p2);
        }
        // Combine inner and outer points into a single Vec. Note we reverse the outer Vec.
        let mut points = vec![];
        for p1 in inner.drain(0..) {
            points.push(p1);
        }
        for p2 in outer.drain(0..).rev() {
            points.push(p2);
        }
        (size, points)
    }
}

fn get_average_rgb(pixels: &Vec<&Rgb<u8>>) -> Rgb<u8> {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut count = 0.0;
    for i in pixels {
        r += i[0] as f64;
        g += i[1] as f64;
        b += i[2] as f64;
        count += 1.0;
    }
    r = r / count;
    g = g / count;
    b = b / count;
    return Rgb([r.round() as u8, g.round() as u8, b.round() as u8]);
}

fn get_average_rgb_between_points(source: &RgbImage, p1: &Point, p2: &Point) -> Rgb<u8> {
    let dist = p1.dist(&p2) as u32;
    let theta = p1.angle(&p2);
    let (width, height) = source.dimensions();
    let mut pixels = vec![];
    for i in 0..dist {
        let sx = (p1.x + i as f64 * theta.cos()).round() as u32;
        let sy = (p1.y + i as f64 * theta.sin()).round() as u32;
        if sx > width - 1 || sy > height - 1 {
            continue;
        }
        let pixel = source.get_pixel(sx, sy);
        pixels.push(pixel);
    }
    get_average_rgb(&pixels)
}
