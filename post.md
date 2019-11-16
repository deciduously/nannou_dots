# Oxidize Your Life With One Weird Trick

TODO post as "Creative Coding in Rust with Nannou", cover image a still of final app - #rust #beginners #tutorial #creative // Not sure about 4th tag?!@?

We're going to build a samll interactive demo with the [nannou](https://nannou.cc/) creative coding framework for [Rust](https://www.rust-lang.org/). This example itself very simple, but specifically over-engineered to prepare to scale even further for your own project.

This is a beginner-level post but with conditions. You should be able to follow along with the logic here in a general sense with comfort in any imperative language, but if you wanted to build your own app from this base as a total newcomer to Rust, I absolutely recommend you read at least some of [The Rust Programming Language](https://doc.rust-lang.org/book/), freely available, or equivalent before tackling this framework.

Part of the strength of `nannou` as a framework is that is _doesn't_ reinvent the wheel. It is instead intended to pull together and unify the best the excellent Rust ecosystem already has to offer for each subproblem in one unified package, innovating only where the need is not already met by the community. You could also get here yourself by adding these dependencies one-by-one and gluing everything together, but this is aiming to be a curated batteries-included distribution for creative coding in pure top-to-bottom Rust.

## Table Of Contents

TODO generate with [generator](https://ecotrust-canada.github.io/markdown-toc/)

- [The Motive](#the-motive)
  - [Yes, Really, In Rust](#yes--really--in-rust)
- [Setup](#setup)
  - [Dependencies](#dependencies)
  - [Set Up the Project](#set-up-the-project)
  - [The Structure](#the-structure)
- [Scaling Out](#scaling-out)
  - [Defensive Refactor](#defensive-refactor)
    - [Traits And Composition](#traits-and-composition)
      - [Debug](#debug)
      - [Default](#default)
      - [Clone/Copy/PartialEq/PartialOrd](#clone-copy-partialeq-partialord)
      - [FromStr](#fromstr)
      - [ToString](#tostring)
      - [From/Into](#from-into)
    - [Quality of Life Crates](#quality-of-life-crates)
      - [Command Line Arguments](#command-line-arguments)
      - [Logging](#logging)
      - [Error Handling](#error-handling)
  - [Lots Of Dots](#lots-of-dots)
- [Wrapping Up](#wrapping-up)
  - [Challenges](#challenges)

## The Motive

I was irrationally hell-bent on modelling a problem in Rust that was perfectly suited for [Processing](https://processing.org/) or its [JavaScript](https://p5js.org/) or [Python](https://py.processing.org/) siblings. Luckily, the Rust ecosystem continues to pleasantly surprise, and it's already possible to do! This tool isn't trying to be Processing-the-library for Rust, and is still very much a work in progress, but it occupies a very similar space and is already quite usable, partially thanks to the wealth of strong component crates already available in the ecosystem to lean on.

### Yes, Really, In Rust

Rust might seem like an oddly... _ahem_ combative choice of tool for such a dynamic and exploratory domain. I've found that after the initial learning curve, Rust's expressivity and modelling power help me get tasks done correctly efficiently, which more than outweighs however much its strict and unique semantics slow me down. My argument is essentially that the benefit of using Rust for this sort of program is that to implement your logic, you get to use Rust. This is pretty subjective argument.

My more substantive take is that it's performant by default, has a rich set of expressive, high-level basic language components and a solid standard library, and a highly helpful compiler if you've modelled your code effectively among all languages I've tried. It does impose a strict, unique mental model but once you understand how it works even that is more a positive point than a negative as well, as it gently nudges you towards better code [Pit of Success](https://blog.codinghorror.com/falling-into-the-pit-of-success/) style.

To me the biggest drawback is compilation time, which is admittedly brutal. This can be frustrating when doing such exploratory, iterative work - `nannou` ain't no Jupyter notebook. Working with this sort of code was a test of that limitation. a warm debug takes about 4 seconds and a release build takes four and a half on my 2017 i7. I generally just use the release build. The library itself has a little under 250 dependencies to build, so a cold taking about five minutes. Even with this frustration I found the balance skewed heavily toward positive, as always, your preferences and mileage may vary.

The caveat to any of my pros, also, is familiarity and experiential bias. I'd love hear why you disagree and Language X is more objectively superior for this and should be used instead!

## Setup

Let's get ourselves to a successful compile first.

### Dependencies

- Stable [Rust 2018](https://doc.rust-lang.org/edition-guide/rust-2018/index.html) - the [default installation](https://www.rust-lang.org/tools/install) is sufficient. This code was written with `rustc` version 1.39.

- [Vulkan SDK](https://www.lunarg.com/vulkan-sdk/) - on Gentoo, I had to install `dev-util/vulkan-tools`, not just `dev-util/vulkan-headers`.

I tested this code on Linux, and I'm not sure how to run this code on OS X and don't have access easily to try it myself.

### Set Up the Project

Create a new Rust project director:

```txt
$ cargo new nannou_dots
$ cd nannou_dots
```

Add the dependency:

```toml
# ..

# after other metadata
[dependencies]

nannou = "0.12"
```

To demonstrate the overall structure of the app, start with this simple demonstration:

```rust
use nannou::{color::named, prelude::*};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    bg_color: String,
    x: f32,
    y: f32,
    radius: f32,
}

fn model(_app: &App) -> Model {
    Model {
        bg_color: "honeydew".to_string(),
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.radius < 500.0 {
        model.radius += 1.0;
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background()
        .color(named::from_str(&model.bg_color).unwrap());
    draw.ellipse()
        .color(STEELBLUE)
        .w(model.radius)
        .h(model.radius)
        .x_y(model.x, model.y);
    draw.to_frame(app, &frame).unwrap();
}
```

Even if you've never worked with a tool like this, take a moment to read through this code and try to understand what will happen when you run it. Once you think you've got it, give it a go with `cargo run --release` and go make a cup of tea. The first build will be intense as it compiles all the dependencies the first time, but re-compiles will be quicker! Granted, not _quick_ - this is one of Rust's definite trade-offs, but even my nine year old low-end laptop could keep up enough to iterate without losing my head after an admittedly pretty nuts initial build. Come back when your tea is cool enough to sip and see if you were right! You can kill the program by using the X button in the corner and re-run it to start the animation over from the beginning. Then kill it again, quickly. It came out unexpectedly terrifying for a "hello, world".

### The Structure

If you're already familiar with model/view/update application structure, skip down to _Defensive Refactor_.

The `main()` function only does one thing: instantiate a `nannou` app object and immediately call it's `run()` method. It then continually draws frames based on the parameters we define. Each frame is defined by a _view_:

```rust
fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background()
        .color(named::from_str(&model.bg_color).unwrap());
    draw.ellipse()
        .color(STEELBLUE)
        .w(model.radius)
        .h(model.radius)
        .x_y(model.x, model.y);
    draw.to_frame(app, &frame).unwrap();
}
```

This contains instructions for drawing a single frame. You can actually use `nannou` with _only_ this function if you'd like to experiment with stateless drawing ideas. Simply replace the `main()` entrypoint code with this:

```rust
fn main() {
    nannou::sketch(view);
}
```

We use the library-provided `draw()` methods provided by the `app` parameter to interact with the frame. First, we set the background color and then draw an ellipse. These methods update the state of the `app` object, and then finally we draw the new state to the `frame`. We get all the parameters about what color to use and how to paint the ellipse from the _model_:

```rust
fn model(_app: &App) -> Model {
    Model {
        bg_color: "honeydew".to_string(),
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    }
}
```

The model is the application state. Here, it's just set to these parameters: "honeydew" is a lovely color for the background that corresponds to one of the [defined constants](https://docs.rs/nannou/0.11.1/nannou/color/index.html#constants), and our ellipse starts off super small and at the center of the frame. Between each frame, the model might _update_:

```rust
fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.radius < 500.0 {
        model.radius += 1.0;
    }
}
```

In this function, we're given a mutable borrow of the `Model` to manipulate. This demo will check if the radius smaller than 500 pixels. If so, it's going to bump it up slightly. If not, nothing else happens.

Pulled all together, this app should be expected to load a mostly "honeydew" (off-white) screen with a small blue circle in the center that will quickly animate to grow to a slightly larger size. It then stays that way until the process ends. I know, riveting so far:

![dot gif](https://thepracticaldev.s3.amazonaws.com/i/tybzj1eg0l9ik8h9duy4.gif)

It's a choppy screen record, but the actual run will be smooth. This program is already a jumping off stub for any demo using this library, feel free to ditch my larger demonstration app and go sailing forward with the [API docs](https://docs.rs/nannou/0.11.1/nannou/index.html) on your own if you already know what to write!

To implement any new functionality in our demo, we'll need to write logic that appropriately extends our model, view, and update functions to show the user what we mean.

## Scaling Out

### Defensive Refactor

The above demo gets us up and running, but we don't want to code directly into the main structure. This is a perfect opportunity to refactor into something that we can grow with more easily. This adds a lot of verbosity, but with Rust the more we can help the tooling the more the tooling helps us, and with a properly configured [development environment](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) it's not even that much typing. I also, er, happen to find Rust refactoring therapeutic but that's beside the point.

Now, cross your fingers - we're going to wipe out this implementation in favor of a more idiomatic (and verbose) Rust approach. I'll elaborate below. For now, go ahead and replace the entire file contents of `src/main.rs` with this - there are no functional changes, ony structural:

```rust
use nannou::{color::named, prelude::*};
use std::string::ToString;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

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

/// A circle to paint
#[derive(Debug, Clone, Copy)]
struct Dot {
    color: Color,
    origin: Point,
    radius: f32,
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
    fn new() -> Self {
        Self::default()
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
    dot: Dot,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            bg_color: Color::Honeydew,
            current_bg: usize::default(),
            dot: Dot::new(),
        }
    }
}

impl Nannou for Model {
    /// Show this model
    fn display(&self, draw: &app::Draw) {
        draw.background().color(Rgb::from(self.bg_color));
        self.dot.display(draw);
    }
    /// Update this model
    fn update(&mut self) {
        self.dot.update();
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
```

As before, I'd urge to take your time and step through this sample as well. I haven't actually changed anything at all functionally, just gotten myself organized. Start from `main()` and literally [rubber-duck](https://en.wikipedia.org/wiki/Rubber_duck_debugging) the [control flow](https://www.computerhope.com/jargon/c/contflow.htm) if you don't follow this just yet. I know, it's a lot bigger, but everything is right where it belongs. This is a much better base to build from. Running `cargo run --release` should produce an identical result to before the switch.

#### Traits And Composition

The `Color` [sum type](https://en.wikipedia.org/wiki/Tagged_union) (or [Rust `enum`](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)) is the best example of Rust-style composition:

```rust
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
```

I've also defined a trait of my own:

```rust
/// Things that can be drawn to the screen
trait Nannou {
    fn display(&self, draw: &app::Draw);
    fn update(&mut self);
}
```

If you're already a regular Rust user, this will likely not be new - skip down to _Load Assets_. It was one of the more unfamiliar bits for me at the outset, though, and the sooner you embrace code that looks like this the sooner Rust will click. It's not nearly as complicated as it looks at first.

Rust does not have traditional inheritance at all, which represents an "is-a" relationship between related instances. Think of a `Cat` inheriting from an `Animal` superclass, because a cat "is-a" animal. Instead, everything is extended via composition, or a "has-a" relationship. Our `Cat` might know how to `speak()` and say something different than a `Dog` would with the same method, but have the `Voiced` trait provide it. Cats and dogs both "has-a" voice. They can manage their own behavior behind the common API instead of overriding a base class implementation. The mechanism for this is [traits](https://doc.rust-lang.org/1.8.0/book/traits.html). They fall somewhere in between (I think) a Java interface and a Haskell typeclass, and are very simple to define. For instance, `std::default::Default` is defined in the [compiler's source code](https://doc.rust-lang.org/src/core/default.rs.html#84-116) as this, omitting the doc comment and version tag:

```rust
pub trait Default: Sized {
    fn default() -> Self;
}
```

The trait only defines a single method that a type needs to define, returning some instance of itself that works as a default value. The trait itself doesn't care how, the compiler will decide whether a specific implementation of this trait checks out. The Rust compiler can statically OR dynamically verify whether a given object implements a trait.

Traits are powerful, and in fact not only permeate Rust usage but power exactly those benefits I listed above, and the Rust compiler is powerful enough to derive many useful ones for you. If you ever do want to override that behavior, you can always provide a manual `impl Trait for Struct` block yourself that matches the prescribed API. There's a great overview of the reasoning and usage in [this blog post](https://blog.rust-lang.org/2015/05/11/traits.html) by [Aaron Turon](https://aturon.github.io/about/).

##### Debug

[`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html) provides a simple pretty-print implementation of a data structure that can be used with the `{:?}` or `{:#?}` formatters in `println!()` (etc) invocations. The default dot looks like this:

```txt
Dot { color: SteelBlue, origin: Point { x: 0.0, y: 0.0 }, radius: 10.0 }
```

Here's a [playground link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=cf5af64babe1cd2523e93224862aa9bb)

##### Default

[`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) provides a method `default()` to be used as a default constructor. When derived just calls `default()` on each member. If one or more of your members do not themselves have a `Default` implementation or you'd like to manually specify something else, you can manually define one:

```rust
impl Default for Dot {
    fn default() -> Self {
        Self {
            color: Color::SteelBlue,
            origin: Point::default(),
            radius: 10.0,
        }
    }
}
```

##### Clone/Copy/PartialEq/PartialOrd

These aren't heavily used in this code, but are commonly found in general.

1. [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) - duplicate an arbitrarily nested object - potentially expensive, will call `clone()` on any child.
1. [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) - duplicate an object that's simple enough to just copy bits. My rule of thumb is that if I can have `Copy`, I take it. Must implement `Clone`.
1. [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)/[`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) - allow two instances of this structure to be compared for equality/magnitude respectively.

These traits are often derived, and are included in the [prelude](https://doc.rust-lang.org/std/prelude/index.html) of library functions available to all Rust modules by default.

##### FromStr

This is not in the prelude and must be explicitly included:

```rust
use std::str::FromStr;
```

You will need to import [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) in order to use or implement it, and allows your self-defined types to [`parse()`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) from string values like primitives. It's relatively straightforward to implement, but does include an associated type - I don't use it in this program but it does come up often. From the docs:

```rust
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                                 .split(',')
                                 .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point { x: x_fromstr, y: y_fromstr })
    }
}
```

##### ToString

You only need to import [`std::string::ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html) if you plan to implement it, as I do for the `Color` enum to map to the exact string values that the library has constants for:

```rust
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
```

This `ToString` implementation does rely on the fact that `Debug` is also implemented for `Color`, so I can use the `{:?}` formatter on `self`.

##### From/Into

This is how to convert between types within your program. Implementing [`From`](https://doc.rust-lang.org/std/convert/trait.From.html) gets you [`Into`](https://doc.rust-lang.org/std/convert/trait.Into.html) and vice versa: you get both when you implement one. You only need to directly implement `Into` if you're converting to some type outside the current crate. I use `From` to get from my special personal `Color` type to the library type the `draw` methods expect:

```rust
/// Type alias for nannou named color type
type Rgb = Srgb<u8>;

impl From<Color> for Rgb {
    fn from(c: Color) -> Self {
        named::from_str(&c.to_string()).unwrap()
    }
}
```

Now I can use colors I know and convert with `Rgb::from()`, but have my own control over `Color` behavior:

```rust
impl Nannou for Model {
    /// Show this model
    fn display(&self, draw: &app::Draw) {
        draw.background().color(Rgb::from(self.bg_color)); // Color::Honeydew
        self.dot.display(draw);
    }
    //..
}
```

Special shout-out to the aforementioned `ToString` implementation to provide `Color::to_string()`, which is where I've defined how to produce the library constants...

![kronk meme](https://media.giphy.com/media/KEYEpIngcmXlHetDqz/giphy.gif)

#### Quality of Life Crates

We're expecting this codebase to grow, and the Rust ecosystem has a few other tidbits that can help us spend time working on the problem, not the environment.

##### Command Line Arguments

I find the easiest way to get this done is [`structopt`](https://github.com/TeXitoi/structopt). This crate lets you define a struct with your options, and it custom-derives you an implementation. Add the dependency to `Cargo.toml`:

```diff
  [dependencies]

  nannou = "0.12"
+ structopt = "0.3"
```

Let's test it out by letting the user control the parameters with command-line options. Add the import tot he top and define the struct:

```rust
use structopt::StructOpt;

// ..

/// A nannou demonstration application
#[derive(StructOpt, Debug)]
#[structopt(name = "tiny_dancer")]
pub struct Opt {
    /// Set dot growth rate
    #[structopt(short, long, default_value = "1.0")]
    rate: f32,
}
```

We want this to load when the application starts and be available everywhere. One way to accomplish this is with `lazy_static`, which lets you define static values that have some runtime initialization. This code will get run once the first time this object is accessed and cache the result for future use throughout your program. This is convenient for things like image assets, which are referred to all over the place making for some potentially sticky ownership problems, and any options passed at runtime that will be true for the entire lifetime of the program.

First, add the dependency to `src/Cargo.toml`:

```diff
  [dependencies]

+ lazy_static = "1.4"
  nannou = "0.12"
  structopt = "0.3"
```

I generally handle `lazy_static` usage that's not local to a specific function at the top of the file:

```rust
lazy_static! {
    pub static ref OPT: Opt = Opt::from_args();
}
```

Check out where that gets us by running the auto-generated `--help/-h` flag:

```txt
$ cargo run --release -- -h
   Compiling tiny_dancer v0.1.0 (/home/ben/code/tinydancer)
    Finished release [optimized] target(s) in 4.83s
     Running `target/release/tiny_dancer -h`
tiny_dancer 0.1.0
A nannou demonstration application

USAGE:
    tiny_dancer [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --rate <rate>    Set dot growth rate [default: 1.0]
```

Good stuff. Notice how the triple-slashed doc comment for the struct member became the help string in this message, and the one for the struct itself is displayed at the top. To hook it up, make the following changes:

```diff
  impl Dot {
      fn new() -> Self {
-         Self::default()
+         Self::default().set_growth_rate(OPT.rate)
      }
+     fn set_growth_rate(mut self, rate: f32) -> Self {
+         self.growth_rate = rate;
+         self
+     }
  }
```

I still leverage the default constructor, but instead give myself a method to set the growth rate of the model. It follows the [Builder Pattern](https://dev.to/deciduously/the-builder-pattern-249l) so that my code remains flexible for changing my mind and experimenting.

##### Logging

Another way to set ourselves up for success is by hooking up the standard Rust logging tooling. Now, I'm going to toss three crates at you but don't panic, it's all just one thing. I'm using [`pretty_env_logger`], which dresses up the output from [`env_logger`] with nice colors and formatting. This itself is a wrapper around [`log`](), which provides a bunch of `println!()`-esque macros like `warn!()`, `debug!()`, and `info!()`. The `env_logger` crate (and thus `pretty_env_logger`)

First, add some new dependencies to `Cargo.toml`:

```diff
  [dependencies]

+ log = "0.4"
  nannou = "0.12"
+ pretty_env_logger = "0.3"
```

The [`pretty_env_logger`](https://github.com/seanmonstar/pretty-env-logger) crate provides some nicer output for the main even - the [`log`](https://docs.rs/log/0.4.8/log/) crate. This exposes some macros that we can use like `warn!()`, `info!()`, and `debug!()` in lieu of `println!()`, and then define how verbose we want our application to be at runtime.

To start it up, here's a function I just kinda copy-paste into new projects:

```rust
use std::env::{set_var, var};

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
```

This function takes a number as a level, but also checks if `RUST_BACKTRACE` is set. `RUST_BACKTRACE` will override whatever is passed to this and set `RUST_LOG` to `trace` automatically. If you pass in a `trace` level of 4 or higher it will automatically set `RUST_BACKTRACE` for you. This behavior is usually what I want.

There's a handy way to collect this information built-in to `structopt` - it can handle arguments from number of occurrences:

```diff
  fn main() {
+     init_logging(OPT.verbosity);
      nannou::app(model).update(update).simple_window(view).run();
  }

  /// A nannou demonstration application
  #[derive(StructOpt, Debug)]
  #[structopt(name = "tiny_dancer")]
  struct Opt {
      /// Set dot growth rate
      #[structopt(short, long, default_value = "1.0")]
      rate: f32,
+     /// Verbose mode (-v: warn, -vv: info, -vvv: debug, , -vvvv or more: trace)
+     #[structopt(short, long, parse(from_occurrences))]
+     verbosity: u8,
  }
```

Use `cargo run --release -- -vv` to get the `info` level:

![info screenshot](https://thepracticaldev.s3.amazonaws.com/i/agk7wajz6ypnduqnvzju.png)

Looks like the Nannou window-handling dependency [`winit`](https://github.com/rust-windowing/winit) is onboard! You can specify per-module by setting, e.g. `RUST_LOG=tiny_dancer=info` to only apply to your own included `info!()` statements.

To run it with a backtrace you can just specify 4 (or more) vs to the program with `-vvvv`:

![trace screenshot](https://thepracticaldev.s3.amazonaws.com/i/2a1qfwq47vx6kcvjvx3l.png)

This allows you to basically do "`print`" debugging without having to comment things out and recompile. Instead, you leave everything in and just specify how much to dump out at runtime.

##### Error Handling

This is more of an honorable mention, but nearly every time I write a Rust project, I end up with some enum:

```rust
#[derive(Debug)]
pub enum ProjectError {
    ErrorOne(String),
    ErrorTwo(u8, u8),
    ErrorOther,
}
```

There's always an empty `std::error::Error` impl, a `Result<T>` alias, and a `std::fmt::Display` block so it can be used with the `{}` formatter:

```rust
impl std::error::Error for ProjectError {}

pub type Result<T> = std::result::Result<T, ProjectError>;

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ProjectError::*;
        let e_str = match self {
            ErrorOne(s) => &format!("{}", s),
            ErrorTwo(x, y) => &format!("expected {}, got {}", x, y),
            ErrorOther => "Something went wrong!",
        };
        write!(f, "Error: {}", e_str)
    }
}
```

I do this every time, and it works, and it's nice to have this control. Luckily, there's a crate for that: [`thiserror`](https://github.com/dtolnay/thiserror), which provides some custom-derive magic on it. We could replace the entire above example with this:

```rust
/// Error types
#[derive(Error, Debug)]
enum ProjectError {
    #[error("{0}")]
    StringError(String),
    #[error("expected {expected:?}, got {found:?}")]
    NumberMismatch { expected: u8, found: u8 },
    #[error("Something went wrong!")]
    ErrorOther,
}
```

However, in this app, _I'm too lazy to even muck around with that_. Who's got time for that [`anyhow`](https://github.com/dtolnay/anyhow):

```diff
  [dependencies]

+ anyhow = "1.0"
  lazy_static = "1.4"
  log = "0.4"
  nannou = "0.12"
  pretty_env_logger = "0.3"
  structopt = "0.3"
```

Then, just add `use anyhow::Result` to the top of your file and get `?` for free. If any function you write calls an operation that can fail, just make your function return this `Result<T>` and it'll just work.

### Lots Of Dots

Let's take our newfound structure for a spin by upping the number of dots. First, add a parameter for the user:

```diff
  /// A nannou demonstration application
  #[derive(StructOpt, Debug)]
  #[structopt(name = "tiny_dancer")]
  pub struct Opt {
+     /// How many dots to render
+     #[structopt(short, long, default_value = "1")]
+     num_dots: u8,
      /// Set dot growth rate
      #[structopt(short, long, default_value = "1.0")]
      rate: f32,
      /// Verbose mode (-v: warn, -vv: info, -vvv: debug, , vvvv or more: trace)
      #[structopt(short, long, parse(from_occurrences))]
      verbosity: u8,
  }
```

The user can specify 0 through 255 dots. In the `Model`, we'll keep track of a `Vec`:

```diff
  /// The application state
  #[derive(Debug)]
  struct Model {
      bg_color: Color,
      current_bg: usize,
-     dot: Dot,
+     dots: Vec<Dot>,
  }

  impl Nannou for Model {
    /// Show this model
    fn display(&self, draw: &app::Draw) {
        draw.background().color(Rgb::from(self.bg_color));
-       self.dot.display(draw);
+       self.dots.iter().for_each(|d| d.display(&draw));
    }
    /// Update this model
    fn update(&mut self) {
-       self.dot.update();
+       self.dots.iter_mut().for_each(|d| d.update());
    }
  }
```

Before we initialize them, let's flesh out the `Dot` definition to allow us to specify a start location:

```diff
  impl Dot {
-     fn new() -> Self {
-         Self::default().set_growth_rate(OPT.rate)
+     fn new(point: Option<Point>) -> Self {
+       let mut ret = Self::default();
+       if let Some(loc) = point {
+           ret.set_location(loc);
+       }
+       ret.set_growth_rate(OPT.rate)
      }
      fn set_growth_rate(mut self, rate: f32) -> Self {
          self.growth_rate = rate;
          self
      }
+     fn set_location(mut self, loc: Point) -> Self {
+       self.origin = loc;
+       self
+    }
  }
```

Now we can make a `Model::init_dots()` associated function that the default constructor can use:

```
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
```

Just swap it in

```diff
  impl Default for Model {
      fn default() -> Self {
          Self {
              bg_color: Color::Honeydew,
              current_bg: usize::default(),
-             dot: Dot::new(),
+             dots: Self::init_dots(),
          }
      }
  }
```

![lots of dots](https://thepracticaldev.s3.amazonaws.com/i/j5n1dtlldrfqjp367ibc.gif)

## Wrapping Up

Some features I didn't touch on bundled with `nannou` include UI components, math functions, image handling, and noise generation, things you'd otherwise manually include a crate yourself for. Nannou aims to be a complete, all-in-one solution leveraging the best of the Rust ecosystem to fit this domain, and by my estimation hits the mark.

I apologize if you were falsely duped into thinking the CSI-themed header titles would have anything to do with the content of this post. I don't know what happened there either, this clearly went in an entirely different direction.

### Challenges

Before moving further, myy recommendation would be to split this logic into [separate modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html), instead of putting everything in `main.rs`. You do you though. Here's a few things you could try next:

1. Add some sounds.
1. Make the dots move.
1. Make the dots different colors.
1. Use the `noise` module to distribute the dots.
1. Add sliders to control parameters.
