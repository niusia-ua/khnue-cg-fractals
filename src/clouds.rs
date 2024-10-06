// Based on https://github.com/Vesther/Cloud-Fractal/tree/master

use nannou::prelude::*;
use rand::Rng;

const SIZE: usize = 513; // Must be 2^n + 1
const RANGE: i32 = 128; // Degree of randomness

struct Model {
  fractal: CloudFractal,
}

fn main() {
  nannou::app(model).run();
}

fn model(app: &App) -> Model {
  app
    .new_window()
    .view(view)
    .key_pressed(handle_pressed_key)
    .build()
    .unwrap();
  Model {
    fractal: CloudFractal::new(),
  }
}

fn view(app: &App, model: &Model, frame: Frame) {
  // Prepare to draw.
  let draw = app.draw();
  draw.background().color(BLACK);

  // Draw the fractal.
  for i in 0..SIZE {
    for j in 0..SIZE {
      let brightness = model.fractal.map[i][j] as f32 / 255.0;
      let color = rgb(brightness, brightness, brightness);
      draw
        .rect()
        .x_y(i as f32 - (SIZE / 2) as f32, j as f32 - (SIZE / 2) as f32)
        .w_h(1.0, 1.0)
        .color(color);
    }
  }

  // Write the result of our drawing to the window's frame.
  draw.to_frame(app, &frame).unwrap();
}

fn handle_pressed_key(_app: &App, model: &mut Model, key: Key) {
  match key {
    // Regenerate the fractal when the space key is pressed.
    Key::Space => model.fractal = CloudFractal::new(),
    _ => {}
  }
}

/// A struct to hold the cloud fractal map.
struct CloudFractal {
  map: Box<[[i32; SIZE]; SIZE]>,
  range: i32,
}

impl CloudFractal {
  fn new() -> Self {
    let mut fractal = CloudFractal {
      map: Box::new([[0; SIZE]; SIZE]),
      range: RANGE,
    };
    fractal.init();
    fractal.gen();
    fractal.clamp_map();
    fractal
  }

  /// Initializes corner values with random values.
  fn init(&mut self) {
    let mut rng = rand::thread_rng();
    self.map[0][0] = rng.gen_range(0..256); // top left
    self.map[0][SIZE - 1] = rng.gen_range(0..256); // top right
    self.map[SIZE - 1][0] = rng.gen_range(0..256); // bottom left
    self.map[SIZE - 1][SIZE - 1] = rng.gen_range(0..256); // bottom right
  }

  /// Performs the fractal generation using the Diamond-Square algorithm.
  fn gen(&mut self) {
    self.diamond_step(SIZE);
    self.square_step(SIZE);
    self.range /= 2;

    let mut side_length = SIZE / 2;
    while side_length >= 2 {
      self.diamond_step(side_length + 1);
      self.square_step(side_length + 1);
      side_length /= 2;
      self.range /= 2;
    }
  }

  /// Performs the diamond step.
  fn diamond_step(&mut self, side_length: usize) {
    let r#mod = side_length - 1;
    let half_side = side_length / 2;
    for x in 0..(SIZE / r#mod) {
      for y in 0..(SIZE / r#mod) {
        let cx = x * r#mod + half_side;
        let cy = y * r#mod + half_side;
        let avg = (self.map[x * r#mod][y * r#mod]
          + self.map[x * r#mod][(y + 1) * r#mod]
          + self.map[(x + 1) * r#mod][y * r#mod]
          + self.map[(x + 1) * r#mod][(y + 1) * r#mod])
          / 4;
        self.map[cx][cy] = avg + self.random_offset();
      }
    }
  }

  // Performs the square step.
  fn square_step(&mut self, side_length: usize) {
    let r#mod = side_length - 1;
    let half_side = side_length / 2;
    for x in 0..(SIZE / r#mod) {
      for y in 0..(SIZE / r#mod) {
        let cx = x * r#mod + half_side;
        let cy = y * r#mod + half_side;
        self.average(cx, y * r#mod, side_length); // top
        self.average((x + 1) * r#mod, cy, side_length); // right
        self.average(cx, (y + 1) * r#mod, side_length); // bottom
        self.average(x * r#mod, cy, side_length); // left
      }
    }
  }

  // Averaging helper function for square step to ignore out of bounds points.
  fn average(&mut self, x: usize, y: usize, side_length: usize) {
    let mut count = 0;
    let mut sum = 0;

    let half_side = side_length / 2;
    if x > 0 {
      sum += self.map[y][x - half_side];
      count += 1;
    }
    if y > 0 {
      sum += self.map[y - half_side][x];
      count += 1;
    }
    if x < SIZE - 1 {
      sum += self.map[y][x + half_side];
      count += 1;
    }
    if y < SIZE - 1 {
      sum += self.map[y + half_side][x];
      count += 1;
    }

    self.map[y][x] = (sum / count) + self.random_offset();
  }

  // Generates a random offset for the roughness.
  fn random_offset(&self) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-self.range..=self.range)
  }

  // Clamps all map values to the range 0..255.
  fn clamp_map(&mut self) {
    for i in 0..SIZE {
      for j in 0..SIZE {
        self.map[i][j] = self.map[i][j].clamp(0, 255);
      }
    }
  }
}
