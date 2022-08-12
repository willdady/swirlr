extern crate clap;
extern crate image;

mod geometry;
mod svg;
mod swirlr;

use clap::Parser;
use swirlr::{Crop, Swirlr};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Path to source image (jpeg or png)
    #[clap(value_parser)]
    source: String,

    /// Whether spiral should extend past the edge of the canvas
    #[clap(default_value_t = Crop::Overflow, long, arg_enum, value_parser)]
    crop: Crop,

    /// Spiral color
    #[clap(default_value_t = String::from("#000"), short, long, value_parser)]
    color: String,

    /// Background color
    #[clap(short, long, value_parser)]
    bg_color: Option<String>,

    /// Rate at which spiral moves away from origin
    #[clap(default_value_t = 1.2, short, long, value_parser)]
    growth_rate: f64,

    /// X coordinate of the center of the spiral
    #[clap(default_value_t = 250.0, short = 'x', long, value_parser)]
    origin_x: f64,

    /// Y coordinate of the center of the spiral
    #[clap(default_value_t = 250.0, short = 'y', long, value_parser)]
    origin_y: f64,

    /// Inverts the image
    #[clap(default_value_t = false, short, long, value_parser)]
    invert: bool,
}

fn main() {
    let cli = Cli::parse();
    let input_path = cli.source;
    let color = cli.color;
    let crop = cli.crop;
    let growth_rate = cli.growth_rate;
    let origin_x = cli.origin_x;
    let origin_y = cli.origin_y;
    let invert = cli.invert;
    let bg_color = cli.bg_color;

    let mut source = image::open(input_path).unwrap().to_rgb8();

    let (size, points) = Swirlr::new(&mut source)
        .set_crop(crop)
        .set_growth_rate(growth_rate)
        .set_origin(origin_x, origin_y)
        .set_invert(invert)
        .get_points();

    let origin_point = &points[0];

    let mut path = svg::Path::new().set_fill(&color);
    path.d.move_to(origin_point.x, origin_point.y);

    let mut previous_point = origin_point;

    for p in points.iter() {
        // Only render points with at least 2 pixels distance
        // between them for a less-complex path.
        if previous_point.dist(&p) > 2.0 {
            path.d.line_to(p.x, p.y);
            previous_point = &p;
        }
    }

    path.d.close();

    let mut _svg = svg::Svg::new(size as u32, size as u32);
    if bg_color.is_some() {
        _svg.add_child(Box::new(
            svg::Rect::new(size as u32, size as u32).set_fill(bg_color),
        ));
    }
    _svg.add_child(Box::new(path));
    print!("{}", _svg);
}
