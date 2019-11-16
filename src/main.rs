use anyhow::Result;
use lazy_static::lazy_static;
use log::*;
use nannou::{color::named, prelude::*, rand};
use std::{
    env::{set_var, var},
    string::ToString,
};
use structopt::StructOpt;

lazy_static! {
    pub static ref OPT: Opt = Opt::from_args();
}

fn main() {
    init_logging(OPT.verbosity);
    nannou::app(model).update(update).simple_window(view).run();
}

//
// CLI arguments
//

/// A nannou demonstration application
#[derive(StructOpt, Debug)]
#[structopt(name = "tiny_dancer")]
pub struct Opt {
    /// How many dots to render
    #[structopt(short, long, default_value = "1")]
    num_dots: u8,
    /// Set dot growth rate
    #[structopt(short, long, default_value = "1.0")]
    rate: f32,
    /// Verbose mode (-v: warn, -vv: info, -vvv: debug, , vvvv or more: trace)
    #[structopt(short, long, parse(from_occurrences))]
    verbosity: u8,
}

//
// Logging
//

/// Start env_logger
fn init_logging(level: u8) {
    // if RUST_BACKTRACE is set, ignore the arg given and set `trace` no matter what
    let mut overridden = false;
    let verbosity = if std::env::var("RUST_BACKTRACE").unwrap_or_else(|_| "0".into()) == "1" {
        overridden = true;
        "trace"
    } else {
        match level {
            0 => "error",
            1 => "warn",
            2 => "info",
            3 => "debug",
            _ => "trace",
        }
    };
    set_var("RUST_LOG", verbosity);
    pretty_env_logger::init();
    if overridden {
        warn!("RUST_BACKTRACE is set, overriding user verbosity level");
    } else if verbosity == "trace" {
        set_var("RUST_BACKTRACE", "1");
        trace!("RUST_BACKTRACE has been set");
    };
    info!(
        "Set verbosity to {}",
        var("RUST_LOG").expect("Should set RUST_LOG environment variable")
    );
}

//
// Types
//

/// All colors used in this application
#[derive(Debug, Clone, Copy)]
enum Color {
    Honeydew,
    SteelBlue,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}

/// Type alias for nannou color type
type Rgb = Srgb<u8>;

impl From<Color> for Rgb {
    fn from(c: Color) -> Self {
        named::from_str(&c.to_string()).unwrap()
    }
}

/// A coordinate pair - the (0,0) default is the center of the frame
#[derive(Debug, Default, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Things that can be drawn to the screen
trait Nannou {
    fn display(&self, draw: &app::Draw);
    fn update(&mut self);
}

/// A circle to paint
#[derive(Debug, Clone, Copy)]
struct Dot {
    color: Color,
    origin: Point,
    radius: f32,
    max_radius: f32,
    growth_rate: f32,
}

impl Dot {
    fn new(point: Option<Point>) -> Self {
        let mut ret = Self::default();
        if let Some(loc) = point {
            ret = ret.set_location(loc);
        }
        ret.set_growth_rate(OPT.rate)
    }
    fn set_growth_rate(mut self, rate: f32) -> Self {
        self.growth_rate = rate;
        self
    }
    fn set_location(mut self, loc: Point) -> Self {
        self.origin = loc;
        self
    }
}

impl Nannou for Dot {
    fn display(&self, draw: &app::Draw) {
        draw.ellipse()
            .w(self.radius)
            .h(self.radius)
            .x_y(self.origin.x, self.origin.y)
            .color(Rgb::from(self.color));
    }
    fn update(&mut self) {
        if self.radius < self.max_radius {
            self.radius += self.growth_rate;
        }
    }
}

impl Default for Dot {
    fn default() -> Self {
        Self {
            color: Color::SteelBlue,
            origin: Point::default(),
            radius: 10.0,
            max_radius: 200.0,
            growth_rate: 1.0,
        }
    }
}

/// The application state
#[derive(Debug)]
struct Model {
    bg_color: Color,
    current_bg: usize,
    dots: Vec<Dot>,
}

impl Model {
    fn init_dots() -> Vec<Dot> {
        let mut ret = Vec::new();
        for _ in 0..OPT.num_dots {
            let point_x = rand::random_range(-500.0, 500.0);
            let point_y = rand::random_range(-500.0, 500.0);
            ret.push(Dot::new(Some(Point::new(point_x, point_y))));
        }
        ret
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            bg_color: Color::Honeydew,
            current_bg: usize::default(),
            dots: Self::init_dots(),
        }
    }
}

impl Nannou for Model {
    /// Show this model
    fn display(&self, draw: &app::Draw) {
        draw.background().color(Rgb::from(self.bg_color));
        self.dots.iter().for_each(|d| d.display(&draw));
    }
    /// Update this model
    fn update(&mut self) {
        self.dots.iter_mut().for_each(|d| d.update());
    }
}

//
// Nannou interface
//

/// Nannou app model
fn model(_app: &App) -> Model {
    Model::default()
}

/// Nannou app update
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

/// Nannou app view
fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    // Draw model
    model.display(&draw);
    // Render frame
    draw.to_frame(&app, &frame).unwrap();
}
