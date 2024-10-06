use nannou::prelude::*;

fn main() {
  nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
  // Prepare to draw.
  let draw = app.draw();
  draw.background().color(WHITE);

  // Define the depth of the triangle.
  let depth = 5;

  // Define the vertices of the triangle.
  let p1 = pt2(0.0, -200.0);
  let p2 = pt2(300.0, 200.0);
  let p3 = pt2(-300.0, 200.0);

  // Draw the triangle.
  draw_triangle(&draw, p1, p2, p3, depth);

  // Write the result of our drawing to the window's frame.
  draw.to_frame(app, &frame).unwrap();
}

fn draw_triangle(draw: &Draw, p1: Point2, p2: Point2, p3: Point2, depth: u32) {
  if depth == 0 {
    return;
  }

  // Draw the triangle.
  draw.polyline().color(BLACK).points(vec![p1, p2, p3, p1]);

  // Calculate the midpoints of the triangle's edges.
  let m1 = (p1 + p2) / 2.0;
  let m2 = (p2 + p3) / 2.0;
  let m3 = (p3 + p1) / 2.0;

  // Recursively draw the smaller triangles.
  draw_triangle(draw, p1, m1, m3, depth - 1);
  draw_triangle(draw, m1, p2, m2, depth - 1);
  draw_triangle(draw, m3, m2, p3, depth - 1);
}
