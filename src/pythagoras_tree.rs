use nannou::prelude::*;

fn main() {
  nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
  // Prepare to draw.
  let draw = app.draw();
  draw.background().color(WHITE);

  // Define the tree's start point, length, angle, and depth.
  let start = pt2(0.0, app.window_rect().bottom() / 2.0);
  let length = 100.0;
  let angle = -90.0;
  let depth = 6;

  // Draw the tree.
  draw_tree(&draw, start, length, angle, depth);

  // Write the result of our drawing to the window's frame.
  draw.to_frame(app, &frame).unwrap();
}

fn draw_tree(draw: &Draw, start: Point2, length: f32, angle: f32, depth: u32) {
  if depth == 0 {
    return;
  }

  // Draw the line.
  let end = start + pt2(length * angle.to_radians().cos(), -(length * angle.to_radians().sin()));
  draw.line().color(BLACK).points(start, end);

  // Recursive calls to draw the tree's branches.
  draw_tree(draw, end, length * 0.8, angle - 45.0, depth - 1);
  draw_tree(draw, end, length * 0.8, angle + 45.0, depth - 1);
}
